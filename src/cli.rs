use uuid::Uuid;

/// Types of pages in the application which can be presented to user.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PageType {
    HomePage,
    EpicDetail(Uuid), // Epic ID
    StoryDetail(Uuid), // Story ID
    CreateEpic,
    CreateStory(Uuid), // Epic ID
    EditEpic(Uuid),   // Epic ID
    EditStory(Uuid),  // Story ID
    DeleteEpic(Uuid), // Epic ID
    DeleteStory(Uuid), // Story ID
}

impl PageType {
    /// Displays a CLI representation of the page type, including applicable data and prompts.
    pub fn display(&self) {
        match self {
            // TODO: Implement display logic for each page type. 
            // Home page must show a list of epics
            // EpicDetail must show epic details and its stories
            // StoryDetail must show story details
            // Create/Edit/Delete pages must show appropriate prompts
        }
    }
}