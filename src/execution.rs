use std::fmt::Formatter;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::model::{CFact, CPFact, Model, Subject, SubjectId, Transaction, TransactionId};
use crate::model::CFact::Promised;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TransactionInstanceId(Uuid);

impl std::fmt::Display for TransactionInstanceId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
    pub fn new(parent_transaction_instance_id: Option<TransactionInstanceId>, transaction_id: TransactionId, product_instance: String, initiator: SubjectId, executor: SubjectId) -> Self {
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
    pub fn new(transaction_instance_id: TransactionInstanceId, performer_id: SubjectId, fact: CPFact) -> Self {
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
    pub fn get_transaction_instance(&self, transaction_instance_id: &TransactionInstanceId) -> &TransactionInstance {
        self.transactions_instances.iter().find(|t_i| t_i.id == *transaction_instance_id).unwrap()
    }

    pub fn delete_transaction_instance(&mut self, transaction_instance_id: &TransactionInstanceId) {
        if let Some(pos) = self.transactions_instances.iter().position(|t_i| t_i.id == *transaction_instance_id) {
            self.transactions_instances.remove(pos);
        }
        use CPWorldItem::*;
        self.c_p_world.retain(|c_p_world_item| match c_p_world_item {
            PWorldItem(p_world_item) => p_world_item.transaction_instance_id != *transaction_instance_id,
            CWorldItem(c_world_item) => c_world_item.transaction_instance_id != *transaction_instance_id
        });
        self.agendas.retain(|(_, agenda_item)| agenda_item.transaction_instance_id != *transaction_instance_id);
    }

    #[inline]
    pub fn get_instances_of_transaction(&self, transaction_id: &TransactionId) -> Vec<&TransactionInstance> {
        self.transactions_instances.iter().filter(|t_i| t_i.transaction_id == *transaction_id).collect()
    }

    #[inline]
    fn get_addressee_for_c_fact(&self, transaction_instance: &TransactionInstance, c_fact: &CFact) -> SubjectId {
        use CFact::*;
        match c_fact  {
            Requested => transaction_instance.executor_id.clone(),
            Promised => transaction_instance.initiator_id.clone(),
            Declined => transaction_instance.initiator_id.clone(),
            Declared => transaction_instance.initiator_id.clone(),
            Accepted => transaction_instance.executor_id.clone(),
            Rejected => transaction_instance.executor_id.clone(),
            RequestRevoked => transaction_instance.executor_id.clone(),
            PromiseRevoked => transaction_instance.initiator_id.clone(),
            DeclineRevoked => transaction_instance.initiator_id.clone(),
            AcceptRevoked => transaction_instance.executor_id.clone(),
            RejectRevoked => transaction_instance.executor_id.clone(),
        }
    }

    pub fn add_transaction_instance(&mut self, model: &Model, transaction_instance: TransactionInstance) {
        let transaction_instance_id = transaction_instance.id.clone();
        let initiator_id = transaction_instance.initiator_id.clone();
        self.transactions_instances.push(transaction_instance);
        self.process_new_fact(model, transaction_instance_id, initiator_id, CPFact::CFact(CFact::Requested));
    }

    pub fn process_new_fact(&mut self, model: &Model, transaction_instance_id: TransactionInstanceId, performer_id: SubjectId, fact: CPFact) {
        let transaction_instance = self.get_transaction_instance(&transaction_instance_id).to_owned();
        let performer = model.get_subject(&performer_id).to_owned();
        let agenda_item = AgendaItem::new(transaction_instance_id.clone(), performer_id.clone(), fact.clone());
        use CPFact::*;
        match &fact {
            PFact => {
                let new_c_p_world_item = CPWorldItem::PWorldItem(PWorldItem {
                    timestamp: agenda_item.timestamp.clone(),
                    transaction_instance_id: transaction_instance_id.clone(),
                    performer,
                });
                self.c_p_world.push(new_c_p_world_item);
                self.agendas.push((transaction_instance.executor_id.clone(), agenda_item)); },
            CFact(c_fact) => {
                let addressee_id = self.get_addressee_for_c_fact(&transaction_instance, &c_fact);
                let addressee = model.get_subject(&addressee_id).to_owned();
                let new_c_p_world_item = CPWorldItem::CWorldItem(CWorldItem {
                    timestamp: agenda_item.timestamp.clone(),
                    transaction_instance_id: transaction_instance_id.clone(),
                    performer,
                    addressee,
                    fact: c_fact.clone(),
                });
                self.c_p_world.push(new_c_p_world_item);
                if *c_fact == Promised {
                    self.agendas.push((performer_id, agenda_item));
                } else if !fact.next_c_acts().is_empty() {
                    self.agendas.push((addressee_id, agenda_item));
                }
            },
        };
    }

    pub fn agenda_for(&self, subject_id: &SubjectId) -> Vec<AgendaItem> {
       self.agendas.iter()
           .filter_map(|(subject_id1, c_p_world_item)| if subject_id1 == subject_id { Some(c_p_world_item.clone()) } else { None })
           .collect()
    }

    pub fn remove_agenda_item(&mut self, agenda_item: &AgendaItem) {
        if let Some(pos) = self.agendas.iter().position(|(_, agenda_item1)| *agenda_item1 == *agenda_item) {
            self.agendas.remove(pos);
        }
    }

    pub fn startable_subtransactions<'a>(model: &'a Model, transaction_instance: &TransactionInstance, subject_id: &SubjectId) -> Vec<&'a Transaction> {
       let impeding_transactions: Vec<&Transaction> = model
           .get_transaction(&transaction_instance.transaction_id)
           .impediments.iter().map(|i| &i.impeding_transaction_id)
           .map(|t_i| model.get_transaction(t_i))
           .collect();
        impeding_transactions.into_iter().filter(|t| {
            model.get_initiator_subjects_ids(&t.id).contains(&subject_id)
        }).collect()
    }

    pub fn get_facts_for_transaction_instance(&self, transaction_instance_id: &TransactionInstanceId) -> Vec<&CPWorldItem> {
       self.c_p_world.iter().filter(|c_p_world_item| c_p_world_item.get_transaction_instance_id() == transaction_instance_id).collect()
    }

    pub fn get_c_p_world_item_by_fact<'a>(&'a self, transaction_instance_id: &TransactionInstanceId, fact: &CPFact) -> Option<&'a CPWorldItem> {
       self.c_p_world.iter().find(|c_p_world_item| {
           use CPWorldItem::*;
           match c_p_world_item {
               PWorldItem(p_world_item) => match fact {
                   CPFact::PFact => p_world_item.transaction_instance_id == *transaction_instance_id,
                   CPFact::CFact(_) => false,
               },
               CWorldItem(c_world_item) => match fact {
                   CPFact::PFact => false,
                   CPFact::CFact(c_fact) => c_world_item.transaction_instance_id == *transaction_instance_id && c_world_item.fact == *c_fact,
               }
           }
       })
    }
}