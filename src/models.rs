use std::collections::HashMap;
use uuid::Uuid;
use serde::{ Deserialize, Serialize };

#[repr(u8)]
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum Status {
    Open = 0,
    InProgress = 20,
    Resolved = 240,
    Closed = 255,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Epic {
    pub uuid: Uuid,
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<u32>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Epic {
            uuid: Uuid::new_v4(),
            name,
            description,
            status: Status::Open,
            stories: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Story {
    pub uuid: Uuid,
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String, db_state: &mut DBState) -> Self {
        Story {
            uuid: Uuid::new_v4(),
            name,
            description,
            status: Status::Open,
        }
    }
}

/// This struct represents the entire db state which includes the last_item_id, epics, and stories
/// TODO: Convert from u32 to Uuid for all ID tracking
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct DBState {
    #[serde(alias = "last_item_id")]
    pub last_item_serial_id: u32,
    pub epics: HashMap<u32, Epic>,
    pub stories: HashMap<u32, Story>,
}