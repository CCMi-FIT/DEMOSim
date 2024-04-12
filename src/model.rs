use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq)]
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

pub type Impediment = (CAct, Vec<CAct>);

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Transaction {
    pub id: Uuid,
    pub t_id: String,
    pub name: String,
    pub product: String,
    pub initiator: ActorRole,
    pub executor: ActorRole,
    // pub impediments:
}

impl Transaction {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            t_id: String::default(),
            name: String::default(),
            product: String::default(),
            initiator: ActorRole::default(),
            executor: ActorRole::default(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct Model {
    pub transactions: Vec<Transaction>,
    pub actor_roles: Vec<ActorRole>,
}

