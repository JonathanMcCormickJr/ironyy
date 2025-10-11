use std::{collections::HashMap, fs, path::{self, Path}};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::models::{ Epic, Status, Story};

/// Represents the overall state of the database, including all epics and stories.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DBState {
    /// The ID of the last item added to the database.
    pub last_item_id: u32,
    /// A mapping of epic IDs to their corresponding `Epic` objects.
    pub epics: HashMap<u32, Epic>,
    /// A mapping of story IDs to their corresponding `Story` objects.
    pub stories: HashMap<u32, Story>,
}

impl DBState {
    /// Reads the database state from the specified JSON file.
    /// If the file does not exist, it initializes a new database state.
    pub fn read_db(&self) -> Result<DBState> {
        let db_path = Path::new("../data/db.json");
        if db_path.exists() {
            let db_content = fs::read_to_string(db_path)?;
            let db_state: DBState = serde_json::from_str(&db_content)?;
            Ok(db_state)
        } else {
            // If the file does not exist, return an empty database state
            Ok(DBState {
                last_item_id: 0,
                epics: HashMap::new(),
                stories: HashMap::new(),
            })
        }
    }
    /// Writes the current database state to the specified JSON file.
    pub fn write_db(&self) -> Result<()> {
        let db_path = Path::new("../data/db.json");
        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let db_content = serde_json::to_string_pretty(self)?;
        fs::write(db_path, db_content)?;
        Ok(())
    }
    /// Looks up the epic ID that contains the given story ID.
    /// Returns `Some(epic_id)` if found, or `None` if the story ID does not exist in any epic.    /// ```
    pub fn get_epic_id_by_story_id(&self, story_id: u32) -> Option<u32> {
        for (epic_id, epic) in &self.epics {
            if epic.stories.contains(&story_id) {
                return Some(*epic_id);
            }
        }
        None
    }
}