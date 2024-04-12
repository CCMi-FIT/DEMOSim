use std::collections::HashMap;
use std::fmt::Formatter;
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CAct {
    Request,
    Promise,
    Decline,
    Accept,
    Reject,
    RevokeRequest,
    RevokePromise,
    RevokeDecline,
    RevokeAccept,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq)]
pub enum CFAct {
    Requested,
    Promised,
    Declined,
    Accepted,
    Rejected,
    RequestRevoked,
    PromiseRevoked,
    DeclineRevoked,
    AcceptRevoked,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct ActorRole {
    pub name: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct TransactionId(Uuid);

impl std::fmt::Display for TransactionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub type Impediments = HashMap<CAct, Vec<(TransactionId, CAct)>>;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Transaction {
    pub id: TransactionId,
    pub t_id: String,
    pub name: String,
    pub product: String,
    pub initiator: ActorRole,
    pub executor: ActorRole,
    pub impediments: Impediments,
}

impl Transaction {
    pub fn new() -> Self {
        Self {
            id: TransactionId(Uuid::new_v4()),
            t_id: String::default(),
            name: String::default(),
            product: String::default(),
            initiator: ActorRole::default(),
            executor: ActorRole::default(),
            impediments: Impediments::default(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct Model {
    pub transactions: Vec<Transaction>,
    pub actor_roles: Vec<ActorRole>,
}

