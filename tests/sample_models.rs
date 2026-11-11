use demosim::execution::{Execution, TransactionInstance, TransactionInstanceId};
use demosim::model::{CAct, CPAct, Model, Transaction};
use std::collections::HashSet;

const SAMPLES: &[(&str, &str)] = &[
    ("Volley", include_str!("fixtures/case Volley.dms")),
    ("Pizzeria", include_str!("fixtures/pizzeria.dms")),
    ("Rent-a-car", include_str!("fixtures/rent-a-car.dms")),
    ("Tree", include_str!("fixtures/tree-test.dms")),
    ("Flat tree", include_str!("fixtures/tree-test-flat.dms")),
];

fn assert_integrity(model: &Model) {
    model.validate().unwrap();
    let roles: HashSet<_> = model.actor_roles.iter().map(|r| &r.id).collect();
    let subjects: HashSet<_> = model.subjects.iter().map(|s| &s.id).collect();
    let kinds: HashSet<_> = model.transactions.iter().map(|t| &t.id).collect();
    assert_eq!(roles.len(), model.actor_roles.len());
    assert_eq!(subjects.len(), model.subjects.len());
    assert_eq!(kinds.len(), model.transactions.len());
    for (role, subject) in model.adt.mappings.keys() {
        assert!(roles.contains(role));
        assert!(subjects.contains(subject));
    }
    for kind in &model.transactions {
        assert!(roles.contains(&kind.initiator_id));
        assert!(roles.contains(&kind.executor_id));
        for link in &kind.initiations {
            assert!(kinds.contains(&link.initiated_transaction_id));
        }
        for link in &kind.impediments {
            assert!(kinds.contains(&link.impeding_transaction_id));
        }
    }
}

#[test]
fn all_five_sample_models_load_with_valid_references_and_round_trip() {
    for (name, text) in SAMPLES {
        let model: Model = ron::from_str(text).unwrap_or_else(|e| panic!("{name}: {e}"));
        assert_integrity(&model);
        let restored: Model = ron::from_str(&ron::to_string(&model).unwrap()).unwrap();
        assert_integrity(&restored);
        assert_eq!(model.name, restored.name);
        assert_eq!(model.actor_roles, restored.actor_roles);
        assert_eq!(model.subjects, restored.subjects);
        assert_eq!(model.adt, restored.adt);
        assert_eq!(model.transactions.len(), restored.transactions.len());
        for (before, after) in model.transactions.iter().zip(&restored.transactions) {
            assert_eq!(
                ron::to_string(before).unwrap(),
                ron::to_string(after).unwrap()
            );
        }
    }
}

struct Scenario {
    model: Model,
    execution: Execution,
}

impl Scenario {
    fn new(source: &str) -> Self {
        Self {
            model: ron::from_str(source).unwrap(),
            execution: Execution::default(),
        }
    }

    fn kind(&self, label: &str) -> &Transaction {
        self.model
            .transactions
            .iter()
            .find(|t| t.t_id == label)
            .unwrap()
    }

    fn blocked(&self, label: &str, scope: Option<TransactionInstanceId>, act: &CPAct) -> bool {
        self.execution
            .get_act_impediments(&self.model, self.kind(label), &scope, act)
            .is_some()
    }

    fn request(
        &mut self,
        label: &str,
        parent: Option<&TransactionInstance>,
    ) -> TransactionInstance {
        let kind = self.kind(label);
        let initiator = self.model.get_initiator_subjects_ids(&kind.id)[0].clone();
        let executor = self.model.adt.get_adt_options_for_role(&kind.executor_id)[0]
            .0
            .clone();
        let scope = parent.map(|p| p.id.clone());
        assert!(
            !self.blocked(label, scope.clone(), &CPAct::CAct(CAct::Request)),
            "blocked request {label}"
        );
        match parent {
            Some(parent) => assert!(self
                .execution
                .startable_subtransactions(&self.model, parent, &initiator)
                .iter()
                .any(|t| t.id == kind.id)),
            None => assert!(self
                .model
                .directly_startable_transactions(&initiator)
                .iter()
                .any(|t| t.id == kind.id)),
        }
        let instance = TransactionInstance::new(
            scope,
            kind.id.clone(),
            format!("{label} product"),
            initiator,
            executor,
        );
        self.execution
            .add_transaction_instance(&self.model, instance.clone())
            .unwrap();
        instance
    }

    fn act(&mut self, instance: &TransactionInstance, act: CPAct) {
        let actor = match act {
            CPAct::CAct(CAct::Accept | CAct::Reject | CAct::Request) => &instance.initiator_id,
            _ => &instance.executor_id,
        };
        let item = self
            .execution
            .agenda_for(actor)
            .into_iter()
            .find(|i| i.transaction_instance_id == instance.id)
            .unwrap();
        assert!(item.fact.next_acts().contains(&act));
        let kind = self.model.get_transaction(&instance.transaction_id);
        assert!(
            !self.blocked(&kind.t_id, Some(instance.id.clone()), &act),
            "blocked {act} in {}",
            kind.t_id
        );
        self.execution
            .process_new_fact(
                &self.model,
                instance.id.clone(),
                actor.clone(),
                act.to_fact(),
            )
            .unwrap();
        self.execution.remove_agenda_item(&item);
    }

    fn finish(&mut self, instance: &TransactionInstance) {
        for act in [
            CPAct::CAct(CAct::Promise),
            CPAct::PAct,
            CPAct::CAct(CAct::Declare),
            CPAct::CAct(CAct::Accept),
        ] {
            self.act(instance, act);
        }
    }
}

fn pizzeria(delivery: bool) {
    let mut s = Scenario::new(SAMPLES[1].1);
    let sale = s.request("T1", None);
    s.act(&sale, CPAct::CAct(CAct::Promise));
    assert!(s.blocked("T1", Some(sale.id.clone()), &CPAct::PAct));
    assert!(s.blocked("T2", Some(sale.id.clone()), &CPAct::CAct(CAct::Request)));
    let payment = s.request("T3", Some(&sale));
    s.act(&payment, CPAct::CAct(CAct::Promise));
    let preparation = s.request("T2", Some(&sale));
    let delivery = delivery.then(|| s.request("T4", Some(&sale)));
    if let Some(delivery) = &delivery {
        s.act(delivery, CPAct::CAct(CAct::Promise));
        assert!(s.blocked("T4", Some(delivery.id.clone()), &CPAct::PAct));
    }
    for act in [
        CPAct::PAct,
        CPAct::CAct(CAct::Declare),
        CPAct::CAct(CAct::Accept),
    ] {
        s.act(&payment, act);
    }
    assert!(s.blocked("T1", Some(sale.id.clone()), &CPAct::PAct));
    s.finish(&preparation);
    if let Some(delivery) = delivery {
        assert!(s.blocked("T1", Some(sale.id.clone()), &CPAct::PAct));
        for act in [
            CPAct::PAct,
            CPAct::CAct(CAct::Declare),
            CPAct::CAct(CAct::Accept),
        ] {
            s.act(&delivery, act);
        }
    }
    for act in [
        CPAct::PAct,
        CPAct::CAct(CAct::Declare),
        CPAct::CAct(CAct::Accept),
    ] {
        s.act(&sale, act);
    }
    assert!(s.execution.agendas.is_empty());
    assert_eq!(
        s.execution.c_p_world.len(),
        s.execution.transactions_instances.len() * 5
    );
}

#[test]
fn pizzeria_completes_without_optional_delivery() {
    pizzeria(false);
}

#[test]
fn pizzeria_delivery_waits_for_preparation_and_blocks_sale_completion() {
    pizzeria(true);
}

#[test]
fn rental_requires_deposit_then_take_then_return_then_invoice() {
    let mut s = Scenario::new(SAMPLES[2].1);
    let rental = s.request("T1", None);
    s.act(&rental, CPAct::CAct(CAct::Promise));
    for label in ["T2", "T3", "T5"] {
        assert!(s.blocked(label, Some(rental.id.clone()), &CPAct::CAct(CAct::Request)));
    }
    for label in ["T4", "T2", "T3", "T5"] {
        assert!(s.blocked("T1", Some(rental.id.clone()), &CPAct::PAct));
        let child = s.request(label, Some(&rental));
        s.finish(&child);
    }
    for act in [
        CPAct::PAct,
        CPAct::CAct(CAct::Declare),
        CPAct::CAct(CAct::Accept),
    ] {
        s.act(&rental, act);
    }
    assert_eq!(s.execution.transactions_instances.len(), 5);
    assert_eq!(s.execution.c_p_world.len(), 25);
    assert!(s.execution.agendas.is_empty());
}
