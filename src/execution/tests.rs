use super::*;
use crate::model::{CAct, MaxMultiplicity};
use crate::test_support::Fixture;

#[test]
fn request_records_participants_and_assigns_executor_agenda() {
    let mut f = Fixture::new();
    let instance = f.root();
    let execution = &f.context.execution;
    assert_eq!(execution.c_p_world.len(), 1);
    match &execution.c_p_world[0] {
        CPWorldItem::CWorldItem(event) => {
            assert_eq!(event.transaction_instance_id, instance.id);
            assert_eq!(event.performer.id, f.customer.id);
            assert_eq!(event.addressee.id, f.supplier.id);
            assert_eq!(event.fact, CFact::Requested);
        }
        _ => panic!("request must produce a coordination fact"),
    }
    assert_eq!(execution.agenda_for(&f.supplier.id).len(), 1);
    assert!(execution.agenda_for(&f.customer.id).is_empty());
    assert!(execution.agenda_for(&f.worker.id).is_empty());
}

#[test]
fn successful_transaction_routes_every_step_and_settles_agendas() {
    let mut f = Fixture::new();
    let instance = f.root();
    let steps = [
        (CPAct::CAct(CAct::Promise), Some(f.supplier.id.clone())),
        (CPAct::PAct, Some(f.supplier.id.clone())),
        (CPAct::CAct(CAct::Declare), Some(f.customer.id.clone())),
        (CPAct::CAct(CAct::Accept), None),
    ];
    for (act, owner) in steps {
        f.act(&instance, act);
        match owner {
            Some(owner) => {
                assert_eq!(f.context.execution.agendas.len(), 1);
                assert_eq!(f.context.execution.agendas[0].0, owner);
            }
            None => assert!(f.context.execution.agendas.is_empty()),
        }
    }
    let events = &f.context.execution.c_p_world;
    assert_eq!(
        events.iter().map(CPWorldItem::to_fact).collect::<Vec<_>>(),
        vec![
            CPFact::CFact(CFact::Requested),
            CPFact::CFact(CFact::Promised),
            CPFact::PFact,
            CPFact::CFact(CFact::Declared),
            CPFact::CFact(CFact::Accepted),
        ]
    );
    let expected = [
        (&f.customer, &f.supplier),
        (&f.supplier, &f.customer),
        (&f.supplier, &f.customer),
        (&f.customer, &f.supplier),
    ];
    let coordination: Vec<_> = events
        .iter()
        .filter_map(|event| match event {
            CPWorldItem::CWorldItem(c) => Some(c),
            _ => None,
        })
        .collect();
    for (event, (performer, addressee)) in coordination.iter().zip(expected) {
        assert_eq!(event.performer.id, performer.id);
        assert_eq!(event.addressee.id, addressee.id);
    }
    assert_eq!(events[2].get_performer().id, f.supplier.id);
}

#[test]
fn rejection_can_repeat_declaration_without_repeating_production() {
    let mut f = Fixture::new();
    let instance = f.root();
    for act in [
        CPAct::CAct(CAct::Promise),
        CPAct::PAct,
        CPAct::CAct(CAct::Declare),
    ] {
        f.act(&instance, act);
    }
    for _ in 0..3 {
        f.act(&instance, CPAct::CAct(CAct::Reject));
        assert_eq!(f.context.execution.agenda_for(&f.supplier.id).len(), 1);
        f.act(&instance, CPAct::CAct(CAct::Declare));
    }
    f.act(&instance, CPAct::CAct(CAct::Accept));
    assert!(f.context.execution.agendas.is_empty());
    assert_eq!(
        f.context
            .execution
            .c_p_world
            .iter()
            .filter(|e| matches!(e, CPWorldItem::PWorldItem(_)))
            .count(),
        1
    );
    assert_eq!(f.context.execution.c_p_world.len(), 11);
}

#[test]
fn identical_product_labels_do_not_merge_transaction_instances() {
    let mut f = Fixture::new();
    let first = f.root();
    let second = f.root();
    assert_ne!(first.id, second.id);
    f.finish(&first);
    assert_eq!(
        f.context
            .execution
            .get_facts_for_transaction_instance(&second.id)
            .len(),
        1
    );
    let agenda = f.context.execution.agenda_for(&f.supplier.id);
    assert_eq!(agenda.len(), 1);
    assert_eq!(agenda[0].transaction_instance_id, second.id);
}

#[test]
fn removing_a_settled_agendum_twice_preserves_other_work() {
    let mut f = Fixture::new();
    let first = f.root();
    let second = f.root();
    let item = f.context.execution.agenda_for(&f.supplier.id)[0].clone();
    assert_eq!(item.transaction_instance_id, first.id);
    f.context.execution.remove_agenda_item(&item);
    f.context.execution.remove_agenda_item(&item);
    assert_eq!(f.context.execution.agendas.len(), 1);
    assert_eq!(
        f.context.execution.agendas[0].1.transaction_instance_id,
        second.id
    );
}

#[test]
fn child_availability_requires_trigger_authority_and_remaining_capacity() {
    let mut f = Fixture::linked(1, MaxMultiplicity::Numeric(1));
    let parent = f.root();
    assert!(f
        .context
        .execution
        .startable_subtransactions(&f.context.model, &parent, &f.supplier.id)
        .is_empty());
    f.act(&parent, CPAct::CAct(CAct::Promise));
    assert_eq!(
        f.context
            .execution
            .startable_subtransactions(&f.context.model, &parent, &f.supplier.id)
            .len(),
        1
    );
    assert!(f
        .context
        .execution
        .startable_subtransactions(&f.context.model, &parent, &f.customer.id)
        .is_empty());
    f.child_of(&parent);
    assert!(f
        .context
        .execution
        .startable_subtransactions(&f.context.model, &parent, &f.supplier.id)
        .is_empty());
}

#[test]
fn required_child_blocks_until_acceptance() {
    let mut f = Fixture::linked(1, MaxMultiplicity::Numeric(1));
    let parent = f.root();
    f.act(&parent, CPAct::CAct(CAct::Promise));
    assert!(f.blocked(&parent, &CPAct::PAct));
    assert!(!f.blocked(&parent, &CPAct::CAct(CAct::Decline)));
    let child = f.child_of(&parent);
    for act in [
        CPAct::CAct(CAct::Promise),
        CPAct::PAct,
        CPAct::CAct(CAct::Declare),
    ] {
        f.act(&child, act);
        assert!(f.blocked(&parent, &CPAct::PAct));
    }
    f.act(&child, CPAct::CAct(CAct::Accept));
    assert!(!f.blocked(&parent, &CPAct::PAct));
    f.act(&parent, CPAct::PAct);
}

#[test]
fn optional_children_may_be_absent_but_all_existing_children_must_complete() {
    let mut f = Fixture::linked(0, MaxMultiplicity::Any);
    let parent = f.root();
    f.act(&parent, CPAct::CAct(CAct::Promise));
    assert!(!f.blocked(&parent, &CPAct::PAct));
    let first = f.child_of(&parent);
    let second = f.child_of(&parent);
    assert!(f.blocked(&parent, &CPAct::PAct));
    f.finish(&first);
    assert!(f.blocked(&parent, &CPAct::PAct));
    f.finish(&second);
    assert!(!f.blocked(&parent, &CPAct::PAct));
}

#[test]
fn sibling_cases_do_not_share_capacity_or_satisfy_each_others_waits() {
    let mut f = Fixture::linked(1, MaxMultiplicity::Numeric(1));
    let first = f.root();
    let second = f.root();
    f.act(&first, CPAct::CAct(CAct::Promise));
    f.act(&second, CPAct::CAct(CAct::Promise));
    let child = f.child_of(&first);
    f.finish(&child);
    assert!(!f.blocked(&first, &CPAct::PAct));
    assert!(f.blocked(&second, &CPAct::PAct));
    assert_eq!(
        f.context
            .execution
            .startable_subtransactions(&f.context.model, &second, &f.supplier.id)
            .len(),
        1
    );
}

#[test]
fn deleting_a_leaf_removes_its_events_and_agendas_only() {
    let mut f = Fixture::linked(1, MaxMultiplicity::Numeric(1));
    let parent = f.root();
    f.act(&parent, CPAct::CAct(CAct::Promise));
    let child = f.child_of(&parent);
    f.context
        .execution
        .delete_transaction_instance(&child.id)
        .unwrap();
    assert_eq!(f.context.execution.transactions_instances.len(), 1);
    assert_eq!(f.context.execution.transactions_instances[0].id, parent.id);
    assert_eq!(f.context.execution.c_p_world.len(), 2);
    assert_eq!(f.context.execution.agendas.len(), 1);
    assert!(f.blocked(&parent, &CPAct::PAct));
}

#[test]
fn execution_round_trip_preserves_history_agendas_and_can_continue() {
    let mut f = Fixture::new();
    let instance = f.root();
    f.act(&instance, CPAct::CAct(CAct::Promise));
    let restored: Execution =
        ron::from_str(&ron::to_string(&f.context.execution).unwrap()).unwrap();
    assert_eq!(restored.c_p_world, f.context.execution.c_p_world);
    assert_eq!(restored.agendas, f.context.execution.agendas);
    assert_eq!(restored.transactions_instances[0].id, instance.id);
    f.context.execution = restored;
    for act in [
        CPAct::PAct,
        CPAct::CAct(CAct::Declare),
        CPAct::CAct(CAct::Accept),
    ] {
        f.act(&instance, act);
    }
    assert!(f.context.execution.agendas.is_empty());
}

#[test]
fn one_subject_in_both_roles_can_complete_without_duplicate_agendas() {
    let mut f = Fixture::new();
    f.context.model.adt.mappings.insert(
        (f.parent.executor_id.clone(), f.customer.id.clone()),
        crate::model::AdtOption::Authorisation,
    );
    let instance = TransactionInstance::new(
        None,
        f.parent.id.clone(),
        "Self service".into(),
        f.customer.id.clone(),
        f.customer.id.clone(),
    );
    f.context
        .execution
        .add_transaction_instance(&f.context.model, instance.clone())
        .unwrap();
    f.finish(&instance);
    assert_eq!(f.context.execution.c_p_world.len(), 5);
    assert!(f.context.execution.agendas.is_empty());
}

mod known_bugs {
    use super::*;

    #[test]
    fn decline_retains_an_agendum_for_the_initiator() {
        let mut f = Fixture::new();
        let instance = f.root();
        f.act(&instance, CPAct::CAct(CAct::Decline));
        assert_eq!(f.context.execution.agenda_for(&f.customer.id).len(), 1);
    }

    #[test]
    fn minimum_two_children_requires_two_accepted_children() {
        let mut f = Fixture::linked(2, MaxMultiplicity::Numeric(2));
        let parent = f.root();
        f.act(&parent, CPAct::CAct(CAct::Promise));
        let first = f.child_of(&parent);
        f.finish(&first);
        assert!(f.blocked(&parent, &CPAct::PAct));
        let second = f.child_of(&parent);
        f.finish(&second);
        assert!(!f.blocked(&parent, &CPAct::PAct));
    }

    #[test]
    fn ancestor_children_do_not_consume_nested_parent_capacity() {
        let mut f = Fixture::linked(1, MaxMultiplicity::Numeric(1));
        let outer = f.root();
        f.act(&outer, CPAct::CAct(CAct::Promise));
        f.child_of(&outer);
        let inner = TransactionInstance::new(
            Some(outer.id.clone()),
            f.parent.id.clone(),
            "Nested parent".into(),
            f.customer.id.clone(),
            f.supplier.id.clone(),
        );
        // This deliberately synthetic hierarchy tests scope lookup independently
        // of the editor's supported initiation topology.
        f.context
            .execution
            .transactions_instances
            .push(inner.clone());
        f.context.execution.record_fact(
            &f.context.model,
            inner.id.clone(),
            f.customer.id.clone(),
            CPFact::CFact(CFact::Requested),
        );
        f.act(&inner, CPAct::CAct(CAct::Promise));
        assert_eq!(
            f.context
                .execution
                .startable_subtransactions(&f.context.model, &inner, &f.supplier.id)
                .len(),
            1
        );
    }

    #[test]
    fn deleting_a_parent_preserves_referential_integrity() {
        let mut f = Fixture::linked(1, MaxMultiplicity::Numeric(1));
        let parent = f.root();
        f.act(&parent, CPAct::CAct(CAct::Promise));
        f.child_of(&parent);
        f.context
            .execution
            .delete_transaction_instance(&parent.id)
            .unwrap_err();
        // Either refusing deletion or safely removing/reparenting descendants is valid.
        for instance in &f.context.execution.transactions_instances {
            if let Some(parent_id) = &instance.parent_transaction_instance_id {
                assert!(f
                    .context
                    .execution
                    .transactions_instances
                    .iter()
                    .any(|i| &i.id == parent_id));
            }
        }
    }

    #[test]
    fn illegal_transition_does_not_change_history() {
        let mut f = Fixture::new();
        let instance = f.root();
        let before = f.context.execution.c_p_world.clone();
        f.context
            .execution
            .process_new_fact(
                &f.context.model,
                instance.id,
                f.customer.id,
                CPFact::CFact(CFact::Accepted),
            )
            .unwrap_err();
        assert_eq!(f.context.execution.c_p_world, before);
    }

    #[test]
    fn outsider_cannot_promise_on_behalf_of_the_executor() {
        let mut f = Fixture::new();
        let instance = f.root();
        let before = f.context.execution.c_p_world.clone();
        f.context
            .execution
            .process_new_fact(
                &f.context.model,
                instance.id,
                f.worker.id,
                CPFact::CFact(CFact::Promised),
            )
            .unwrap_err();
        assert_eq!(f.context.execution.c_p_world, before);
    }
}

#[test]
fn decline_renewal_loops_then_completes_the_same_instance() {
    let mut f = Fixture::new();
    let instance = f.root();
    for _ in 0..3 {
        f.act(&instance, CPAct::CAct(CAct::Decline));
        f.act(&instance, CPAct::CAct(CAct::Request));
        assert_eq!(f.context.execution.agendas.len(), 1);
        assert_eq!(f.context.execution.agendas[0].0, f.supplier.id);
    }
    f.finish(&instance);
    assert_eq!(f.context.execution.transactions_instances.len(), 1);
    assert_eq!(f.context.execution.c_p_world.len(), 11);
    assert!(f.context.execution.agendas.is_empty());
}

#[test]
fn blocked_response_does_not_consume_agenda_or_record_an_event() {
    let mut f = Fixture::linked(1, MaxMultiplicity::Numeric(1));
    let parent = f.root();
    f.act(&parent, CPAct::CAct(CAct::Promise));
    let history = f.context.execution.c_p_world.clone();
    let agendas = f.context.execution.agendas.clone();
    assert!(f
        .context
        .execution
        .process_new_fact(&f.context.model, parent.id, f.supplier.id, CPFact::PFact)
        .is_err());
    assert_eq!(f.context.execution.c_p_world, history);
    assert_eq!(f.context.execution.agendas, agendas);
}

#[test]
fn duplicate_request_and_exhausted_capacity_leave_execution_unchanged() {
    let mut f = Fixture::linked(1, MaxMultiplicity::Numeric(1));
    let parent = f.root();
    f.act(&parent, CPAct::CAct(CAct::Promise));
    let child = f.child_of(&parent);
    let history = f.context.execution.c_p_world.clone();
    let agendas = f.context.execution.agendas.clone();
    assert!(f
        .context
        .execution
        .add_transaction_instance(&f.context.model, child.clone())
        .is_err());
    let another = TransactionInstance::new(
        Some(parent.id),
        f.child.id,
        "Another product".into(),
        f.supplier.id,
        f.worker.id,
    );
    assert!(f
        .context
        .execution
        .add_transaction_instance(&f.context.model, another)
        .is_err());
    assert_eq!(f.context.execution.transactions_instances.len(), 2);
    assert_eq!(f.context.execution.c_p_world, history);
    assert_eq!(f.context.execution.agendas, agendas);
}

#[test]
fn removing_executor_authority_blocks_response_without_losing_work() {
    let mut f = Fixture::new();
    let instance = f.root();
    f.context
        .model
        .adt
        .mappings
        .remove(&(f.parent.executor_id, f.supplier.id.clone()));
    let agendas = f.context.execution.agendas.clone();
    assert!(f
        .context
        .execution
        .process_new_fact(
            &f.context.model,
            instance.id,
            f.supplier.id,
            CPFact::CFact(CFact::Promised)
        )
        .is_err());
    assert_eq!(f.context.execution.c_p_world.len(), 1);
    assert_eq!(f.context.execution.agendas, agendas);
}

#[test]
fn unauthorised_addressee_and_missing_parent_cannot_create_partial_instances() {
    let mut f = Fixture::new();
    let wrong_actor = TransactionInstance::new(
        None,
        f.parent.id.clone(),
        "Order".into(),
        f.customer.id.clone(),
        f.worker.id.clone(),
    );
    assert!(f
        .context
        .execution
        .add_transaction_instance(&f.context.model, wrong_actor)
        .is_err());
    assert!(f.context.execution.transactions_instances.is_empty());
    assert!(f.context.execution.agendas.is_empty());
    assert!(f.context.execution.c_p_world.is_empty());
    let absent = TransactionInstance::new(
        None,
        f.parent.id.clone(),
        "Absent".into(),
        f.customer.id.clone(),
        f.supplier.id.clone(),
    );
    let orphan = TransactionInstance::new(
        Some(absent.id),
        f.child.id.clone(),
        "Orphan".into(),
        f.supplier.id.clone(),
        f.worker.id.clone(),
    );
    assert!(f
        .context
        .execution
        .add_transaction_instance(&f.context.model, orphan)
        .is_err());
    assert!(f.context.execution.transactions_instances.is_empty());
}

#[test]
fn completed_transaction_rejects_duplicate_acceptance() {
    let mut f = Fixture::new();
    let instance = f.root();
    f.finish(&instance);
    assert!(f
        .context
        .execution
        .process_new_fact(
            &f.context.model,
            instance.id,
            f.customer.id,
            CPFact::CFact(CFact::Accepted)
        )
        .is_err());
    assert_eq!(f.context.execution.c_p_world.len(), 5);
    assert!(f.context.execution.agendas.is_empty());
}

#[test]
fn multiple_initiation_links_report_ambiguous_wait_instead_of_guessing_a_minimum() {
    let mut f = Fixture::linked(1, MaxMultiplicity::Numeric(1));
    let parent = f.root();
    f.act(&parent, CPAct::CAct(CAct::Promise));
    let child = f.child_of(&parent);
    f.finish(&child);
    let duplicate = f.context.model.transactions[0].initiations[0].clone();
    f.context.model.transactions[0].initiations.push(duplicate);
    let messages = f
        .context
        .execution
        .get_act_impediments(
            &f.context.model,
            &f.context.model.transactions[0],
            &Some(parent.id),
            &CPAct::PAct,
        )
        .unwrap();
    assert!(messages
        .iter()
        .any(|m| m.contains("multiple initiation links")));
}
