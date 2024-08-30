use std::fmt::Formatter;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::model::{CFact, SubjectId, TransactionId};

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
    pub transaction_id: TransactionId,
    pub product_instance: String,
    pub initiator: SubjectId,
    pub executor: SubjectId,
}

impl TransactionInstance {
    pub fn new(transaction_id: TransactionId, product_instance: String, initiator: SubjectId, executor: SubjectId) -> Self {
        TransactionInstance {
            id: TransactionInstanceId(Uuid::new_v4()),
            transaction_id,
            product_instance,
            initiator,
            executor,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct CWorldItem {
    pub timestamp: DateTime<Utc>,
    pub performer: SubjectId,
    pub addressee: SubjectId,
    pub intention: CFact,
    pub transaction_instance_id: TransactionInstanceId,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct PWorldItem {
    pub timestamp: DateTime<Utc>,
    pub performer: SubjectId,
    pub transaction_instance_id: TransactionInstanceId,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum CPWorldItem {
    CWorldItem(CWorldItem),
    PWorldItem(PWorldItem),
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Default)]
pub struct Execution {
    pub transaction_instances: Vec<TransactionInstance>,
    pub c_p_world: Vec<CPWorldItem>,
}