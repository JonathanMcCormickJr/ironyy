use std::{collections::HashMap, fs, path::{self, Path}};

use anyhow::{anyhow, Ok, Result};
use serde::{Deserialize, Serialize};

use crate::models::{ Epic, Status, Story};

/// Represents the overall state of the database, including all epics and stories.
/// 
/// If you want to read a specific field, use the corresponding getter method: `get_last_item_id`, `get_epics`, or `get_stories`.
/// 
/// All public methods that modify the in-memory database state will automatically persist the changes to the JSON file. Due to this, you don't need to worry about whether your calls to getters will return up-to-date information.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DBState {
    last_item_id: u32,
    epics: HashMap<u32, Epic>,
    stories: HashMap<u32, Story>,
}

impl DBState {
    /// Returns the `last_item_id` field.
    pub fn get_last_item_id(&self) -> u32 {
        self.last_item_id
    }

    /// Returns a reference to the `epics` HashMap.
    pub fn get_epics(&self) -> &HashMap<u32, Epic> {
        &self.epics
    }

    /// Returns a reference to the `stories` HashMap.
    pub fn get_stories(&self) -> &HashMap<u32, Story> {
        &self.stories
    }

    /// Reads the database state from the specified JSON file.
    /// If the file does not exist, it initializes a new database state.
    fn read_db() -> Result<Self> {
        let db_path = Path::new("../data/db.json");
        if db_path.exists() {
            let db_content = fs::read_to_string(db_path)?;
            let db_state: Self = serde_json::from_str(&db_content)?;
            Ok(db_state)
        } else {
            // If the file does not exist, then create the file and initialize an empty database state
            if let Some(parent) = db_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(db_path, "{}")?;
            Ok(Self {
                last_item_id: 0,
                epics: HashMap::new(),
                stories: HashMap::new(),
            })
        }
    }

    /// Writes the current database state to the specified JSON file.
    fn write_db(&self) -> Result<()> {
        let db_path = Path::new("../data/db.json");
        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let db_content = serde_json::to_string_pretty(self)?;
        fs::write(db_path, db_content)?;
        Ok(())
    }

    /// Create a new epic with the given title and description.
    pub fn create_epic(&mut self, title: String, description: String) -> Result<()> {
        self.last_item_id += 1;
        let epic = Epic::new(title, description);
        self.epics.insert(self.last_item_id, epic);
        self.write_db()?;
        Ok(())
    }

    /// Create a new story under the specified epic with the given title and description.
    /// 
    /// Failure results in the DBState being reverted to the current contents of the JSON file.
    pub fn create_story(&mut self, epic_id: u32, title: String, description: String) -> Result<()> {
        if let Some(epic) = self.epics.get_mut(&epic_id) {
            self.last_item_id += 1;
            let story = Story::new(title, description);
            epic.stories.push(self.last_item_id);
            self.stories.insert(self.last_item_id, story);
            self.write_db()?;
            Ok(())
        } else {
            *self = Self::read_db()?;
            Err(anyhow::anyhow!("Epic with ID {} does not exist", epic_id))
        }
    }

    pub fn update_epic_status(&mut self, epic_id: u32, new_status: Status) -> Result<()> {
        // TODO
    }



    // TODO: Complete CRUD

    /// Looks up the epic ID that contains the given story ID.
    /// Returns `Some(epic_id)` if found, or `None` if the story ID does not exist in any epic.
    pub fn get_epic_id_by_story_id(&self, story_id: u32) -> Option<u32> {
        for (epic_id, epic) in &self.epics {
            if epic.stories.contains(&story_id) {
                return Some(*epic_id);
            }
        }
        None
    }
}