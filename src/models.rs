use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

/// Represents an action that can be performed in the application by the user.
#[derive(Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Action {
    NavigateToEpicDetail { epic_id: u32 },
    NavigateToStoryDetail { story_id: u32 },
    NavigateToPreviousPage,
    CreateEpic,
    UpdateEpicStatus { epic_id: u32 },
    DeleteEpic { epic_id: u32 },
    CreateStory { epic_id: u32 },
    UpdateStoryStatus { story_id: u32 },
    DeleteStory { story_id: u32 },
    Exit,
}

/// Represents the status of an epic or story.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Status {
    /// The item is newly created and not yet started.
    Open,
    /// The item is currently being worked on.
    InProgress,
    /// The item has been completed but not yet verified.
    Resolved,
    /// The item has been verified and closed.
    Closed,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_str = match self {
            Status::Open => "OPEN",
            Status::InProgress => "IN PROGRESS",
            Status::Resolved => "RESOLVED",
            Status::Closed => "CLOSED",
        };
        write!(f, "{}", status_str)
    }
}

/// Represents an epic, which can contain multiple stories.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Epic {
    /// The name of the epic.
    pub name: String,
    /// A detailed description of the epic.
    pub description: String,
    /// The current status of the epic.
    pub status: Status,
    /// A list of story IDs associated with this epic.
    pub stories: Vec<u32>,
}

impl Epic {
    /// Creates a new epic with the given name and description.
    /// The status is set to `Open` and the stories list is initialized as empty.
    ///
    /// ```rust
    /// use ironyy::models::{Epic, Status};
    /// let epic = Epic::new("New Epic".to_string(), "This is a new epic".to_string());
    /// assert_eq!(epic.name, "New Epic");
    /// assert_eq!(epic.description, "This is a new epic");
    /// assert_eq!(epic.status, Status::Open);
    /// assert!(epic.stories.is_empty());
    /// ```
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::Open,
            stories: Vec::new(),
        }
    }
}

/// Represents a story, which is a smaller task within an epic.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Story {
    /// The name of the story.
    pub name: String,
    /// A detailed description of the story.
    pub description: String,
    /// The current status of the story.
    pub status: Status,
}

impl Story {
    /// Creates a new story with the given name and description.
    /// The status is set to `Open`.
    ///
    /// ```rust
    /// use ironyy::models::{Story, Status};
    /// let story = Story::new("New Story".to_string(), "This is a new story".to_string());
    /// assert_eq!(story.name, "New Story");
    /// assert_eq!(story.description, "This is a new story");
    /// assert_eq!(story.status, Status::Open);
    /// ```
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::Open,
        }
    }
}

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
