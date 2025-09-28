use uuid::Uuid;

#[repr(u8)]
pub enum Status {
    Open = 0,
    InProgress = 20,
    Resolved = 240,
    Closed = 255,
}

pub struct Epic {
    name: String,
    description: String,
    status: Status,
    stories: Vec<Uuid>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Epic {
            name,
            description,
            status: Status::Open,
            stories: Vec::new(),
        }
    }
}

pub struct Story {
    id: Uuid,
    name: String,
    description: String,
    status: Status,
}

impl Story {
    pub fn new(name: String, description: String, db_state: &mut DBState) -> Self {
        let new_last_item_id = db_state.last_item_id as u128 + 1;
        db_state.last_item_id = Uuid::from_u128(new_last_item_id);
        Story {
            // id: Uuid::new_v4(), // TODO: uncomment this line
            id: Uuid::from_u128(new_last_item_id), // TODO: remove this line and everything about "last_item_id" from DBState
            name,
            description,
            status: Status::Open,
        }
    }
}

/// This struct represents the entire db state which includes the last_item_id, epics, and stories
pub struct DBState {
    last_item_id: Uuid,
    epics: Vec<Epic>,
    stories: Vec<Story>,
}