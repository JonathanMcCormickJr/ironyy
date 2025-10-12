use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Status {
    Open,
    InProgress,
    Closed,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Epic {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: Status,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Story {
    pub id: Uuid,
    pub epic_id: Uuid,
    pub title: String,
    pub description: String,
    pub status: Status,
}