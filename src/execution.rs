use crate::model::CAct;
use crate::model::CFact::Promised;
use crate::model::{CFact, CPAct, CPFact, Model, Subject, SubjectId, Transaction, TransactionId};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[cfg(test)]
mod tests;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TransactionInstanceId(Uuid);

impl std::fmt::Display for TransactionInstanceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct TransactionInstance {
    pub id: TransactionInstanceId,
    pub parent_transaction_instance_id: Option<TransactionInstanceId>,
    pub transaction_id: TransactionId,
    pub product_instance: String,
    pub initiator_id: SubjectId,
    pub executor_id: SubjectId,
}

impl TransactionInstance {
    pub fn new(
        parent_transaction_instance_id: Option<TransactionInstanceId>,
        transaction_id: TransactionId,
        product_instance: String,
        initiator: SubjectId,
        executor: SubjectId,
    ) -> Self {
        TransactionInstance {
            id: TransactionInstanceId(Uuid::new_v4()),
            parent_transaction_instance_id,
            transaction_id,
            product_instance,
            initiator_id: initiator,
            executor_id: executor,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq)]
pub struct CWorldItem {
    pub timestamp: DateTime<Utc>,
    pub transaction_instance_id: TransactionInstanceId,
    pub performer: Subject,
    pub addressee: Subject,
    pub fact: CFact,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq)]
pub struct PWorldItem {
    pub timestamp: DateTime<Utc>,
    pub transaction_instance_id: TransactionInstanceId,
    pub performer: Subject,
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Eq, Debug, Clone)]
pub enum CPWorldItem {
    CWorldItem(CWorldItem),
    PWorldItem(PWorldItem),
}

impl CPWorldItem {
    pub fn get_timestamp(&self) -> &DateTime<Utc> {
        use CPWorldItem::*;
        match self {
            CWorldItem(c) => &c.timestamp,
            PWorldItem(p) => &p.timestamp,
        }
    }

    pub fn get_transaction_instance_id(&self) -> &TransactionInstanceId {
        use CPWorldItem::*;
        match self {
            CWorldItem(c) => &c.transaction_instance_id,
            PWorldItem(p) => &p.transaction_instance_id,
        }
    }

    pub fn get_performer(&self) -> &Subject {
        use CPWorldItem::*;
        match self {
            CWorldItem(c) => &c.performer,
            PWorldItem(p) => &p.performer,
        }
    }
    pub fn to_fact(&self) -> CPFact {
        use CPWorldItem::*;
        match self {
            CWorldItem(c) => CPFact::CFact(c.fact.to_owned()),
            PWorldItem(_) => CPFact::PFact,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq)]
pub struct AgendaItem {
    pub timestamp: DateTime<Utc>,
    pub transaction_instance_id: TransactionInstanceId,
    pub performer_id: SubjectId,
    pub fact: CPFact,
}

impl AgendaItem {
    pub fn new(
        transaction_instance_id: TransactionInstanceId,
        performer_id: SubjectId,
        fact: CPFact,
    ) -> Self {
        Self {
            timestamp: Utc::now(),
            transaction_instance_id,
            performer_id,
            fact,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Default)]
pub struct Execution {
    pub transactions_instances: Vec<TransactionInstance>,
    pub c_p_world: Vec<CPWorldItem>,
    pub agendas: Vec<(SubjectId, AgendaItem)>,
}

impl Execution {
    pub fn get_transaction_instance(
        &self,
        transaction_instance_id: &TransactionInstanceId,
    ) -> &TransactionInstance {
        self.transactions_instances
            .iter()
            .find(|t_i| t_i.id == *transaction_instance_id)
            .unwrap()
    }

    pub fn can_delete_transaction_instance(&self, id: &TransactionInstanceId) -> bool {
        self.transactions_instances
            .iter()
            .any(|instance| &instance.id == id)
            && !self
                .transactions_instances
                .iter()
                .any(|instance| instance.parent_transaction_instance_id.as_ref() == Some(id))
    }

    pub fn delete_transaction_instance(
        &mut self,
        transaction_instance_id: &TransactionInstanceId,
    ) -> Result<(), String> {
        if !self.can_delete_transaction_instance(transaction_instance_id) {
            return Err("Remove child instances before deleting their parent".into());
        }
        if let Some(pos) = self
            .transactions_instances
            .iter()
            .position(|t_i| t_i.id == *transaction_instance_id)
        {
            self.transactions_instances.remove(pos);
        }
        use CPWorldItem::*;
        self.c_p_world
            .retain(|c_p_world_item| match c_p_world_item {
                PWorldItem(p_world_item) => {
                    p_world_item.transaction_instance_id != *transaction_instance_id
                }
                CWorldItem(c_world_item) => {
                    c_world_item.transaction_instance_id != *transaction_instance_id
                }
            });
        self.agendas.retain(|(_, agenda_item)| {
            agenda_item.transaction_instance_id != *transaction_instance_id
        });
        Ok(())
    }

    /// Only instances belonging to this parent. Ancestry is resolved separately for waits.
    pub fn get_instances_of_transaction(
        &self,
        transaction_id: &TransactionId,
        parent: &Option<TransactionInstanceId>,
    ) -> Vec<&TransactionInstance> {
        self.transactions_instances
            .iter()
            .filter(|instance| {
                instance.transaction_id == *transaction_id
                    && instance.parent_transaction_instance_id == *parent
            })
            .collect()
    }

    #[inline]
    fn get_addressee_for_c_fact(
        &self,
        transaction_instance: &TransactionInstance,
        c_fact: &CFact,
    ) -> SubjectId {
        use CFact::*;
        match c_fact {
            Requested => transaction_instance.executor_id.clone(),
            Promised => transaction_instance.initiator_id.clone(),
            Declined => transaction_instance.initiator_id.clone(),
            Declared => transaction_instance.initiator_id.clone(),
            Accepted => transaction_instance.executor_id.clone(),
            Rejected => transaction_instance.executor_id.clone(),
            // RequestRevoked => transaction_instance.executor_id.clone(),
            // PromiseRevoked => transaction_instance.initiator_id.clone(),
            // DeclineRevoked => transaction_instance.initiator_id.clone(),
            // AcceptRevoked => transaction_instance.executor_id.clone(),
            // RejectRevoked => transaction_instance.executor_id.clone(),
        }
    }

    pub fn validate_request(
        &self,
        model: &Model,
        instance: &TransactionInstance,
    ) -> Result<(), String> {
        let transaction = model
            .transactions
            .iter()
            .find(|t| t.id == instance.transaction_id)
            .ok_or("Transaction kind no longer exists")?;
        if self
            .transactions_instances
            .iter()
            .any(|i| i.id == instance.id)
        {
            return Err("Transaction instance already exists".into());
        }
        if instance.product_instance.trim().is_empty() {
            return Err("Enter the requested product".into());
        }
        if !model.subject_has_role(&instance.initiator_id, &transaction.initiator_id) {
            return Err("Initiator does not hold the required actor role".into());
        }
        if !model.subject_has_role(&instance.executor_id, &transaction.executor_id) {
            return Err("Addressee does not hold the executor role".into());
        }
        if let Some(parent_id) = &instance.parent_transaction_instance_id {
            let parent = self
                .transactions_instances
                .iter()
                .find(|i| &i.id == parent_id)
                .ok_or("Parent instance no longer exists")?;
            if !self
                .startable_subtransactions(model, parent, &instance.initiator_id)
                .iter()
                .any(|t| t.id == transaction.id)
            {
                return Err(
                    "This child transaction is not available or its maximum has been reached"
                        .into(),
                );
            }
        } else if !model
            .directly_startable_transactions(&instance.initiator_id)
            .iter()
            .any(|t| t.id == transaction.id)
        {
            return Err("This transaction must be initiated from its parent".into());
        }
        if let Some(messages) = self.get_act_impediments(
            model,
            transaction,
            &instance.parent_transaction_instance_id,
            &CPAct::CAct(CAct::Request),
        ) {
            return Err(messages.join("\n"));
        }
        Ok(())
    }

    pub fn add_transaction_instance(
        &mut self,
        model: &Model,
        instance: TransactionInstance,
    ) -> Result<(), String> {
        self.validate_request(model, &instance)?;
        let id = instance.id.clone();
        let initiator = instance.initiator_id.clone();
        self.transactions_instances.push(instance);
        self.record_fact(model, id, initiator, CPFact::CFact(CFact::Requested));
        Ok(())
    }

    /// Validate and commit a response atomically, including settlement of its agendum.
    pub fn process_new_fact(
        &mut self,
        model: &Model,
        transaction_instance_id: TransactionInstanceId,
        performer_id: SubjectId,
        fact: CPFact,
    ) -> Result<(), String> {
        let instance = self
            .transactions_instances
            .iter()
            .find(|i| i.id == transaction_instance_id)
            .ok_or("Transaction instance no longer exists")?;
        let transaction = model
            .transactions
            .iter()
            .find(|t| t.id == instance.transaction_id)
            .ok_or("Transaction kind no longer exists")?;
        let last = self
            .c_p_world
            .iter()
            .rev()
            .find(|event| event.get_transaction_instance_id() == &transaction_instance_id)
            .ok_or("Transaction has no history")?;
        let act = last
            .to_fact()
            .next_acts()
            .into_iter()
            .find(|act| act.to_fact() == fact)
            .ok_or("That act is not available in the current transaction state")?;
        let (actor, role) = match act {
            CPAct::CAct(CAct::Request | CAct::Accept | CAct::Reject) => {
                (&instance.initiator_id, &transaction.initiator_id)
            }
            _ => (&instance.executor_id, &transaction.executor_id),
        };
        if &performer_id != actor || !model.subject_has_role(&performer_id, role) {
            return Err("Only the authorised participant can perform this act".into());
        }
        if !model.subjects.iter().any(|s| s.id == instance.initiator_id)
            || !model.subjects.iter().any(|s| s.id == instance.executor_id)
        {
            return Err("A participant no longer exists".into());
        }
        let agenda_index = self
            .agendas
            .iter()
            .position(|(owner, item)| {
                owner == actor
                    && item.transaction_instance_id == transaction_instance_id
                    && item.fact == last.to_fact()
                    && item.timestamp == *last.get_timestamp()
            })
            .ok_or("The triggering agendum has already been settled")?;
        let scope = if act == CPAct::CAct(CAct::Request) {
            instance.parent_transaction_instance_id.clone()
        } else {
            Some(transaction_instance_id.clone())
        };
        if let Some(messages) = self.get_act_impediments(model, transaction, &scope, &act) {
            return Err(messages.join("\n"));
        }
        self.agendas.remove(agenda_index);
        self.record_fact(model, transaction_instance_id, performer_id, fact);
        Ok(())
    }

    fn record_fact(
        &mut self,
        model: &Model,
        transaction_instance_id: TransactionInstanceId,
        performer_id: SubjectId,
        fact: CPFact,
    ) {
        let transaction_instance = self
            .get_transaction_instance(&transaction_instance_id)
            .to_owned();
        let performer = model.get_subject(&performer_id).to_owned();
        let agenda_item = AgendaItem::new(
            transaction_instance_id.clone(),
            performer_id.clone(),
            fact.clone(),
        );
        use CPFact::*;
        match &fact {
            PFact => {
                let new_c_p_world_item = CPWorldItem::PWorldItem(PWorldItem {
                    timestamp: agenda_item.timestamp,
                    transaction_instance_id: transaction_instance_id.clone(),
                    performer,
                });
                self.c_p_world.push(new_c_p_world_item);
                self.agendas
                    .push((transaction_instance.executor_id.clone(), agenda_item));
            }
            CFact(c_fact) => {
                let addressee_id = self.get_addressee_for_c_fact(&transaction_instance, c_fact);
                let addressee = model.get_subject(&addressee_id).to_owned();
                let new_c_p_world_item = CPWorldItem::CWorldItem(CWorldItem {
                    timestamp: agenda_item.timestamp,
                    transaction_instance_id: transaction_instance_id.clone(),
                    performer,
                    addressee,
                    fact: c_fact.clone(),
                });
                self.c_p_world.push(new_c_p_world_item);
                if *c_fact == Promised {
                    self.agendas.push((performer_id, agenda_item));
                } else if !fact.next_acts().is_empty() {
                    self.agendas.push((addressee_id, agenda_item));
                }
            }
        };
    }

    pub fn agenda_for(&self, subject_id: &SubjectId) -> Vec<AgendaItem> {
        self.agendas
            .iter()
            .filter_map(|(subject_id1, c_p_world_item)| {
                if subject_id1 == subject_id {
                    Some(c_p_world_item.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn remove_agenda_item(&mut self, agenda_item: &AgendaItem) {
        if let Some(pos) = self
            .agendas
            .iter()
            .position(|(_, agenda_item1)| *agenda_item1 == *agenda_item)
        {
            self.agendas.remove(pos);
        }
    }

    pub fn get_facts_for_transaction_instance(
        &self,
        transaction_instance_id: &TransactionInstanceId,
    ) -> Vec<&CPWorldItem> {
        self.c_p_world
            .iter()
            .filter(|c_p_world_item| {
                c_p_world_item.get_transaction_instance_id() == transaction_instance_id
            })
            .collect()
    }

    pub fn get_c_p_world_item_by_fact<'a>(
        &'a self,
        transaction_instance_id: &TransactionInstanceId,
        fact: &CPFact,
    ) -> Option<&'a CPWorldItem> {
        self.c_p_world.iter().find(|c_p_world_item| {
            use CPWorldItem::*;
            match c_p_world_item {
                PWorldItem(p_world_item) => match fact {
                    CPFact::PFact => {
                        p_world_item.transaction_instance_id == *transaction_instance_id
                    }
                    CPFact::CFact(_) => false,
                },
                CWorldItem(c_world_item) => match fact {
                    CPFact::PFact => false,
                    CPFact::CFact(c_fact) => {
                        c_world_item.transaction_instance_id == *transaction_instance_id
                            && c_world_item.fact == *c_fact
                    }
                },
            }
        })
    }

    pub fn startable_subtransactions<'a>(
        &self,
        model: &'a Model,
        parent_transaction_instance: &TransactionInstance,
        subject_id: &SubjectId,
    ) -> Vec<&'a Transaction> {
        let parent_transaction = model.get_transaction(&parent_transaction_instance.transaction_id);

        let mut res: Vec<&Transaction> = parent_transaction
            .initiations
            .iter()
            .filter_map(|initiation| {
                if initiation.initiated_c_act != CAct::Request {
                    return None;
                }
                let initiated_transaction = model
                    .transactions
                    .iter()
                    .find(|t| t.id == initiation.initiated_transaction_id)?;
                // Check if the multiplicity constraint is met
                let instance_count = self
                    .get_instances_of_transaction(
                        &initiated_transaction.id,
                        &Some(parent_transaction_instance.id.clone()),
                    )
                    .len();
                let within_bound = initiation.multiplicity.max.is_within_bound(instance_count);
                within_bound.then_some(())?;

                // Check if the subject is an initiator
                let is_initiator = model
                    .get_initiator_subjects_ids(&initiation.initiated_transaction_id)
                    .contains(subject_id);
                is_initiator.then_some(())?;

                // Check if the initiating C-fact condition matches
                let fact_matches = self
                    .get_c_p_world_item_by_fact(
                        &parent_transaction_instance.id,
                        &CPFact::CFact(initiation.initiating_c_fact.clone()),
                    )
                    .is_some();
                fact_matches.then_some(initiated_transaction)
            })
            .collect();
        res.sort();
        res.dedup();
        res
    }

    /// Find the nearest ancestor that actually declares this dependency's initiation.
    /// This keeps sibling waits working without counting children from other scopes.
    fn dependency_scope(
        &self,
        model: &Model,
        target: &TransactionId,
        start: &Option<TransactionInstanceId>,
    ) -> Result<(Option<TransactionInstanceId>, usize), String> {
        let mut scope = start.clone();
        let mut visited = std::collections::HashSet::new();
        while let Some(id) = &scope {
            if !visited.insert(id.clone()) {
                break;
            }
            let Some(instance) = self.transactions_instances.iter().find(|i| &i.id == id) else {
                break;
            };
            if let Some(kind) = model
                .transactions
                .iter()
                .find(|t| t.id == instance.transaction_id)
            {
                let links: Vec<_> = kind
                    .initiations
                    .iter()
                    .filter(|i| &i.initiated_transaction_id == target)
                    .collect();
                if links.len() > 1 {
                    return Err("Cannot resolve a wait with multiple initiation links to the same transaction kind".into());
                }
                if let Some(link) = links.first() {
                    return Ok((scope, usize::from(link.multiplicity.min)));
                }
            }
            scope = instance.parent_transaction_instance_id.clone();
        }
        // A wait with no matching initiation is still a required condition.
        Ok((start.clone(), 1))
    }

    pub fn get_act_impediments(
        &self,
        model: &Model,
        transaction: &Transaction,
        parent: &Option<TransactionInstanceId>,
        act: &CPAct,
    ) -> Option<Vec<String>> {
        let mut messages = Vec::new();
        for wait in transaction
            .impediments
            .iter()
            .filter(|wait| &wait.impeded_act == act)
        {
            let Some(kind) = model
                .transactions
                .iter()
                .find(|t| t.id == wait.impeding_transaction_id)
            else {
                messages.push("Wait link refers to a missing transaction".into());
                continue;
            };
            let (scope, minimum) = match self.dependency_scope(model, &kind.id, parent) {
                Ok(scope) => scope,
                Err(error) => {
                    messages.push(error);
                    continue;
                }
            };
            let instances = self.get_instances_of_transaction(&kind.id, &scope);
            if instances.len() < minimum {
                messages.push(format!(
                    "Waiting for at least {minimum} instance(s) of {} ({} exist)",
                    kind.t_id,
                    instances.len()
                ));
            }
            for instance in instances {
                if self
                    .get_c_p_world_item_by_fact(
                        &instance.id,
                        &CPFact::CFact(wait.impeding_c_fact.clone()),
                    )
                    .is_none()
                {
                    messages.push(format!(
                        "Waiting for transaction instance {} being {}",
                        instance.id, wait.impeding_c_fact
                    ));
                }
            }
        }
        if messages.is_empty() {
            None
        } else {
            Some(messages)
        }
    }
}
