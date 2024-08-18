use std::fmt::Formatter;
use uuid::Uuid;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq, Hash, EnumIter)]
pub enum CAct {
    Request,
    Promise,
    Decline,
    Declare,
    Accept,
    Reject,
    RevokeRequest,
    RevokePromise,
    RevokeDecline,
    RevokeAccept,
    RevokeReject,
}

impl Default for CAct {
    fn default() -> Self {
        Self::Request
    }
}

impl std::fmt::Display for CAct {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use CAct::*;
        match self {
            Request => write!(f, "Request"),
            Promise => write!(f, "Promise"),
            Decline => write!(f, "Decline"),
            Declare => write!(f, "Declare"),
            Accept => write!(f, "Accept"),
            Reject => write!(f, "Reject"),
            RevokeRequest => write!(f, "Revoke Request"),
            RevokePromise => write!(f, "Revoke Promise"),
            RevokeDecline => write!(f, "Revoke Decline"),
            RevokeAccept => write!(f, "Revoke Accept"),
            RevokeReject => write!(f, "Revoke Reject"),
        }
    }
}

pub fn all_c_acts() -> Vec<CAct> {
    CAct::iter().collect()
}


#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq, EnumIter)]
pub enum CFact {
    Requested,
    Promised,
    Declined,
    Declared,
    Accepted,
    Rejected,
    RequestRevoked,
    PromiseRevoked,
    DeclineRevoked,
    AcceptRevoked,
    RejectRevoked,
}

impl Default for CFact {
    fn default() -> Self {
        CFact::Requested
    }
}

impl std::fmt::Display for CFact {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use CFact::*;
        match self {
            Requested => write!(f, "Requested"),
            Promised => write!(f, "Promised"),
            Declined => write!(f, "Declined"),
            Declared => write!(f, "Declared"),
            Accepted => write!(f, "Accepted"),
            Rejected => write!(f, "Rejected"),
            RequestRevoked => write!(f, "Request Revoked"),
            PromiseRevoked => write!(f, "Promise Revoked"),
            DeclineRevoked => write!(f, "Decline Revoked"),
            AcceptRevoked => write!(f, "Accept Revoked"),
            RejectRevoked => write!(f, "Reject Revoked"),
        }
    }
}

pub fn all_c_facts() -> Vec<CFact> {
    CFact::iter().collect()
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ActorRoleId(Uuid);

impl std::fmt::Display for ActorRoleId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ActorRole {
    pub id: ActorRoleId,
    pub name: String,
}

impl Default for ActorRole {
    fn default() -> Self {
        ActorRole {
            id: ActorRoleId(Uuid::new_v4()),
            name: "".to_string(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TransactionId(Uuid);

impl std::fmt::Display for TransactionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Impediment {
    pub impeded_c_act: CAct,
    pub impeding_transaction_id: TransactionId,
    pub impeding_c_fact: CFact,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Transaction {
    pub id: TransactionId,
    pub t_id: String,
    pub name: String,
    pub product: String,
    pub initiator_id: ActorRoleId,
    pub executor_id: ActorRoleId,
    pub impediments: Vec<Impediment>,
}

impl Transaction {
    pub fn new() -> Self {
        Self {
            id: TransactionId(Uuid::new_v4()),
            t_id: String::default(),
            name: String::default(),
            product: String::default(),
            initiator_id: ActorRole::default().id,
            executor_id: ActorRole::default().id,
            impediments: Vec::default(),
        }
    }
}

impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Transaction {}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Model {
    pub name: String,
    pub transactions: Vec<Transaction>,
    pub actor_roles: Vec<ActorRole>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            name: "No name".to_string(),
            transactions: Vec::new(),
            actor_roles: Vec::new(),
        }
    }
}

