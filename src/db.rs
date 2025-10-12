use std::path::PathBuf;
use serde::{Serialize, Deserialize};

use crate::{models::{Epic, Story}};

/// Represents the state of the database, including epics and stories.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DBState {
    persistence_path: PathBuf,
    epics: Vec<Epic>,
    stories: Vec<Story>,
}

impl DBState {
    /// Creates a DBState instance from a given file path. 
    /// If the file does not exist, it creates the file and 
    /// initializes an empty state
    pub fn from(path: PathBuf) -> Self {
        // Read the file if it exists, otherwise create it
        if let Ok(contents) = std::fs::read_to_string(&path) {
            if let Ok(state) = serde_json::from_str::<DBState>(&contents) {
                return state;
            }
        } else {
            // Create the file if it doesn't exist
            std::fs::File::create(&path).expect("Failed to create the database file");
        }
        DBState {
            persistence_path: path,
            epics: Vec::new(),
            stories: Vec::new(),
        }
    }
}