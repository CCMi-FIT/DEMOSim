use std::collections::HashMap;
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


#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SubjectId(Uuid);

impl std::fmt::Display for SubjectId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Subject {
    pub id: SubjectId,
    pub name: String,
}

impl Default for Subject {
    fn default() -> Self {
        Subject {
            id: SubjectId(Uuid::new_v4()),
            name: "".to_string(),
        }
    }
}


#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq, EnumIter)]
pub enum AdtOption {
    Authorisation,
    Delegation,
}

impl AdtOption {
    pub fn from_str(input: &str) -> Option<AdtOption> {
        match input {
            "A" => Some(AdtOption::Authorisation),
            "D" => Some(AdtOption::Delegation),
            _ => None,
        }
    }
}

impl Default for AdtOption {
    fn default() -> Self {
        AdtOption::Authorisation
    }
}

impl std::fmt::Display for AdtOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use AdtOption::*;
        match self {
            Authorisation => write!(f, "A"),
            Delegation => write!(f, "D"),
        }
    }
}

pub fn all_adt_options() -> Vec<AdtOption> {
    AdtOption::iter().collect()
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Default, PartialEq)]
pub struct Adt { // Authorisation Delegation Table
    pub mappings: HashMap<(ActorRoleId, SubjectId), AdtOption>
}

impl Adt {
    pub fn is_mapped(&self, ar_id: &ActorRoleId) -> bool {
        self.mappings.keys().find(|(ar_id1, _)| ar_id1 == ar_id).is_some()
    }

    pub fn get_roles_of_subject(&self, subject: &Subject) -> Vec<&ActorRoleId> {
        self.mappings.iter()
            .filter_map(|((actor_role_id, subject_id), _)| if *subject_id == subject.id { Some(actor_role_id) } else { None })
            .collect()
    }

    pub fn get_adt_options_for_role(&self, ar_id: &ActorRoleId) -> Vec<(&SubjectId, &AdtOption)> {
        self.mappings.iter()
            .filter_map(|((actor_role_id, subject_id), adt_option)| if actor_role_id == ar_id { Some((subject_id, adt_option)) } else { None })
            .collect()
    }
}


#[derive(serde::Deserialize, serde::Serialize)]
pub struct Model {
    pub name: String,
    pub actor_roles: Vec<ActorRole>,
    pub transactions: Vec<Transaction>,
    pub subjects: Vec<Subject>,
    pub adt: Adt,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            name: "No name".to_string(),
            actor_roles: Vec::new(),
            transactions: Vec::new(),
            subjects: Vec::new(),
            adt: Adt::default(),
        }
    }
}

impl Model {
    pub fn get_actor_role(&self, ar_id: &ActorRoleId) -> &ActorRole {
        self.actor_roles.iter().find(|ar| ar.id == *ar_id).unwrap()
    }

    pub fn get_transaction(&self, t_id: &TransactionId) -> &Transaction {
        self.transactions.iter().find(|t| t.id == *t_id).unwrap()
    }

    pub fn get_subject(&self, s_id: &SubjectId) -> &Subject {
        self.subjects.iter().find(|s| s.id == *s_id).unwrap()
    }

    fn can_start_transaction(&self, subject: &Subject) -> bool {
        //A subject can start a transaction iff he is not an executor of another one
        let roles_ids = self.adt.get_roles_of_subject(subject);
        self.transactions.iter().find(|t| roles_ids.contains(&&t.executor_id)) == None
    }

    pub fn startable_transactions(&self, subject: &Subject) -> Vec<&Transaction> {
        let roles_ids = self.adt.get_roles_of_subject(subject);
        if !self.can_start_transaction(subject) {
            Vec::new()
        } else {
            self.transactions.iter().filter(|t| roles_ids.contains(&&t.initiator_id)).collect()
        }
    }
}

