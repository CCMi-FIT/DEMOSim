use std::fmt::Formatter;
use uuid::Uuid;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use crate::model::CAct::Decline;
use crate::model::CFact::Declined;

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
        }
    }
}

pub fn all_c_facts() -> Vec<CFact> {
    CFact::iter().collect()
}


#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct ActorRole {
    pub name: String,
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
    pub initiator: ActorRole,
    pub executor: ActorRole,
    pub impediments: Vec<Impediment>,
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

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct Model {
    pub transactions: Vec<Transaction>,
    pub actor_roles: Vec<ActorRole>,
    pub selected_c_act: CAct,
}

