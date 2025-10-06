use std::{fs, path::Path};

use anyhow::Result;

use crate::models::{DBState, Epic, Status, Story};

/// Main interface for interacting with the Jira-like database.
pub struct JiraDatabase {
    pub database: Box<dyn Database>,
}

impl JiraDatabase {
    /// Creates a new JiraDatabase instance with a JSON file backend.
    /// The database file will be created at the specified file_path.
    /// If a file already exists at that path, an error will be returned.
    ///
    /// ```rust
    /// use ironyy::db::JiraDatabase;
    ///
    /// // Remove test file if it exists
    /// if std::path::Path::new("test_db_new.json").exists() {
    ///    std::fs::remove_file("test_db_new.json").unwrap();
    /// }
    ///
    /// let db = JiraDatabase::new("test_db_new.json".to_string());
    /// assert_eq!(db.is_ok(), true);
    ///
    /// let db_state = db.as_ref().unwrap().read_db().unwrap();
    /// assert_eq!(db_state.last_item_id, 0);
    /// assert_eq!(db_state.epics.len(), 0);
    /// assert_eq!(db_state.stories.len(), 0);
    ///
    /// // Clean up the created file after the test
    /// std::fs::remove_file("test_db_new.json").unwrap();
    ///
    /// ```
    pub fn new(file_path: String) -> Result<Self> {
        let path = Path::new(&file_path);

        // Ensure parent directory exists (if any) so fs::write won't fail with NotFound.
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }

        if !path.exists() {
            fs::write(
                &file_path,
                r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#,
            )?;
            Ok(Self {
                database: Box::new(JSONFileDatabase::new(file_path)),
            })
        } else {
            Err(anyhow::anyhow!(
                "Database file with that path already exists"
            ))
        }
    }

    /// Reads the current state of the database.
    ///
    /// ```rust
    /// use ironyy::{ db::JiraDatabase, models::DBState };
    /// use std::fs::File;
    /// use std::io::Write;
    ///
    /// // Remove test file if it exists
    /// if std::path::Path::new("./test_dir/test_db_read_db.json").exists() {
    ///  std::fs::remove_file("./test_dir/test_db_read_db.json").unwrap();
    /// }
    ///
    /// let db = JiraDatabase::new("./test_dir/test_db_read_db.json".to_string()).unwrap();
    ///
    /// let db_state = db.read_db().unwrap();
    /// assert_eq!(db_state, DBState { last_item_id: 0, epics: std::collections::HashMap::new(), stories: std::collections::HashMap::new() });
    ///
    /// // Delete the file after the test
    /// std::fs::remove_file("./test_dir/test_db_read_db.json").unwrap();
    /// ```
    pub fn read_db(&self) -> Result<DBState> {
        self.database.read_db()
    }

    /// Creates a new epic in the database and returns its ID.
    ///
    /// ```rust
    /// use ironyy::{ db::JiraDatabase, models::Epic };
    /// use std::fs::File;
    /// use std::io::Write;
    ///
    /// // Remove test file if it exists
    /// if std::path::Path::new("./test_dir/test_db_create_epic.json").exists() {
    ///   std::fs::remove_file("./test_dir/test_db_create_epic.json").unwrap();
    /// }
    ///
    /// let db = JiraDatabase::new("./test_dir/test_db_create_epic.json".to_string()).unwrap();
    /// let epic = Epic::new("Epic 1".to_owned(), "Description of Epic 1".to_owned());
    ///
    /// let result = db.create_epic(epic);
    /// assert_eq!(result.is_ok(), true);
    ///
    /// let epic_id = result.unwrap();
    /// assert_eq!(epic_id, 1);
    ///
    /// let db_state = db.read_db().unwrap();
    /// assert_eq!(db_state.last_item_id, 1);
    /// assert_eq!(db_state.epics.get(&epic_id).is_some(), true);
    /// assert_eq!(db_state.epics.get(&epic_id).unwrap().name, "Epic 1");
    /// assert_eq!(db_state.epics.get(&epic_id).unwrap().description, "Description of Epic 1");
    /// assert_eq!(db_state.epics.get(&epic_id).unwrap().status.to_string(), "OPEN");
    /// assert_eq!(db_state.epics.get(&epic_id).unwrap().stories.len(), 0);
    ///
    /// // Delete the file after the test
    /// std::fs::remove_file("./test_dir/test_db_create_epic.json").unwrap();
    /// ```
    pub fn create_epic(&self, epic: Epic) -> Result<u32> {
        let mut db_state = self.read_db()?;
        let new_id = db_state.last_item_id + 1;
        db_state.last_item_id = new_id;
        db_state.epics.insert(new_id, epic);

        self.database.write_db(&db_state)?;
        Ok(new_id)
    }

    /// Creates a new story in the database under the specified epic and returns the new story's ID.
    /// If the specified epic does not exist, an error is returned.
    ///
    /// ```rust
    /// use ironyy::{ db::JiraDatabase, models::{Epic, Story} };
    /// use std::fs::File;
    /// use std::io::Write;
    ///
    /// // Remove test file if it exists
    /// if std::path::Path::new("./test_dir/test_db_create_story.json").exists() {
    ///   std::fs::remove_file("./test_dir/test_db_create_story.json").unwrap();
    /// }
    ///
    /// let db = JiraDatabase::new("./test_dir/test_db_create_story.json".to_string()).unwrap();
    /// let epic = Epic::new("Epic 1".to_owned(), "Description of Epic 1".to_owned());
    /// let story = Story::new("Story 1".to_owned(), "Description of Story 1".to_owned());
    ///
    /// let result = db.create_epic(epic);
    /// assert_eq!(result.is_ok(), true);
    ///
    /// let epic_id = result.unwrap();
    ///
    /// let result = db.create_story(story, epic_id);
    /// assert_eq!(result.is_ok(), true);
    ///
    /// let story_id = result.unwrap();
    /// assert_eq!(story_id, 2);
    ///
    /// let db_state = db.read_db().unwrap();
    /// assert_eq!(db_state.last_item_id, 2);
    /// assert_eq!(db_state.stories.get(&story_id).is_some(), true);
    /// assert_eq!(db_state.stories.get(&story_id).unwrap().name, "Story 1");
    /// assert_eq!(db_state.stories.get(&story_id).unwrap().description, "Description of Story 1");
    /// assert_eq!(db_state.stories.get(&story_id).unwrap().status.to_string(), "OPEN");
    /// assert_eq!(db_state.epics.get(&epic_id).unwrap().stories.contains(&story_id), true);
    ///
    /// // Delete the file after the test
    /// std::fs::remove_file("./test_dir/test_db_create_story.json").unwrap();
    /// ```
    pub fn create_story(&self, story: Story, epic_id: u32) -> Result<u32> {
        let mut db_state = self.read_db()?;

        if !db_state.epics.contains_key(&epic_id) {
            anyhow::bail!("Epic with id {} does not exist", epic_id);
        }

        let new_id = db_state.last_item_id + 1;
        db_state.last_item_id = new_id;
        db_state.stories.insert(new_id, story);
        db_state
            .epics
            .get_mut(&epic_id)
            .ok_or_else(|| anyhow::anyhow!("Epic with id {} does not exist", epic_id))?
            .stories
            .push(new_id);

        self.database.write_db(&db_state)?;
        Ok(new_id)
    }

    /// Deletes the specified epic and all its associated stories from the database.
    /// If the specified epic does not exist, an error is returned.
    /// ```rust
    /// use ironyy::{ db::JiraDatabase, models::{Epic, Story} };
    /// use std::fs::File;
    /// use std::io::Write;
    ///
    /// // Remove test file if it exists
    /// if std::path::Path::new("./test_dir/test_db_delete_epic.json").exists() {
    ///   std::fs::remove_file("./test_dir/test_db_delete_epic.json").unwrap();
    /// }
    ///
    /// let db = JiraDatabase::new("./test_dir/test_db_delete_epic.json".to_string()).unwrap();
    /// let epic = Epic::new("Epic 1".to_owned(), "Description of Epic 1".to_owned());
    /// let story = Story::new("Story 1".to_owned(), "Description of Story 1".to_owned());
    ///
    /// let result = db.create_epic(epic);
    /// assert_eq!(result.is_ok(), true);
    ///
    /// let epic_id = result.unwrap();
    ///
    /// let result = db.create_story(story, epic_id);
    /// assert_eq!(result.is_ok(), true);
    ///
    /// let story_id = result.unwrap();
    ///
    /// let result = db.delete_epic(epic_id);
    /// assert_eq!(result.is_ok(), true);
    ///
    /// let db_state = db.read_db().unwrap();
    /// assert_eq!(db_state.last_item_id, 2);
    /// assert_eq!(db_state.epics.get(&epic_id), None);
    /// assert_eq!(db_state.stories.get(&story_id), None);
    ///
    /// // Delete the file after the test
    /// std::fs::remove_file("./test_dir/test_db_delete_epic.json").unwrap();
    /// ```
    pub fn delete_epic(&self, epic_id: u32) -> Result<()> {
        let mut db_state = self.read_db()?;

        let epic = db_state
            .epics
            .remove(&epic_id)
            .ok_or_else(|| anyhow::anyhow!("Epic with id {} does not exist", epic_id))?;

        for story_id in epic.stories {
            db_state.stories.remove(&story_id);
        }

        self.database.write_db(&db_state)?;
        Ok(())
    }

    /// Deletes the specified story from the specified epic in the database.
    /// If the specified epic or story does not exist, an error is returned.
    /// ```rust
    /// use ironyy::{ db::JiraDatabase, models::{Epic, Story} };
    /// use std::fs::File;
    /// use std::io::Write;
    ///
    /// // Remove test file if it exists
    /// if std::path::Path::new("./test_dir/test_db_delete_story.json").exists() {
    ///   std::fs::remove_file("./test_dir/test_db_delete_story.json").unwrap();
    /// }
    ///
    /// let db = JiraDatabase::new("./test_dir/test_db_delete_story.json".to_string()).unwrap();
    /// let epic = Epic::new("Epic 1".to_owned(), "Description of Epic 1".to_owned());
    /// let story = Story::new("Story 1".to_owned(), "Description of Story 1".to_owned());
    ///
    /// let result = db.create_epic(epic);
    /// assert_eq!(result.is_ok(), true);
    ///
    /// let epic_id = result.unwrap();
    ///
    /// let result = db.create_story(story, epic_id);
    /// assert_eq!(result.is_ok(), true);
    ///
    /// let story_id = result.unwrap();
    ///
    /// let result = db.delete_story(epic_id, story_id);
    /// assert_eq!(result.is_ok(), true);
    ///
    /// let db_state = db.read_db().unwrap();
    /// assert_eq!(db_state.last_item_id, 2);
    /// assert_eq!(db_state.epics.get(&epic_id).unwrap().stories.contains(&story_id), false);
    /// assert_eq!(db_state.stories.get(&story_id), None);
    ///
    /// // Delete the file after the test
    /// std::fs::remove_file("./test_dir/test_db_delete_story.json").unwrap();
    /// ```
    pub fn delete_story(&self, epic_id: u32, story_id: u32) -> Result<()> {
        let mut db_state = self.read_db()?;

        let epic = db_state
            .epics
            .get_mut(&epic_id)
            .ok_or_else(|| anyhow::anyhow!("Epic with id {} does not exist", epic_id))?;

        if !epic.stories.contains(&story_id) {
            anyhow::bail!(
                "Story with id {} does not exist in epic with id {}",
                story_id,
                epic_id
            );
        }

        epic.stories.retain(|&id| id != story_id);
        db_state.stories.remove(&story_id);

        self.database.write_db(&db_state)?;
        Ok(())
    }

    /// Updates the status of the specified epic in the database.
    /// If the specified epic does not exist, an error is returned.
    /// ```rust
    /// use ironyy::{ db::JiraDatabase, models::{Epic, Status} };
    /// use std::fs::File;
    /// use std::io::Write;
    ///
    /// // Remove test file if it exists
    /// if std::path::Path::new("./test_dir/test_db_update_epic_status.json").exists() {
    ///   std::fs::remove_file("./test_dir/test_db_update_epic_status.json").unwrap();
    /// }
    ///
    /// let db = JiraDatabase::new("./test_dir/test_db_update_epic_status.json".to_string()).unwrap();
    /// let epic = Epic::new("Epic 1".to_owned(), "Description of Epic 1".to_owned());
    ///
    /// let result = db.create_epic(epic);
    /// assert_eq!(result.is_ok(), true);
    ///
    /// let epic_id = result.unwrap();
    ///
    /// let result = db.update_epic_status(epic_id, Status::Closed);
    /// assert_eq!(result.is_ok(), true);
    ///
    /// let db_state = db.read_db().unwrap();
    /// assert_eq!(db_state.epics.get(&epic_id).unwrap().status, Status::Closed);
    ///
    /// // Delete the file after the test
    /// std::fs::remove_file("./test_dir/test_db_update_epic_status.json").unwrap();
    /// ```
    pub fn update_epic_status(&self, epic_id: u32, status: Status) -> Result<()> {
        let mut db_state = self.read_db()?;

        let epic = db_state
            .epics
            .get_mut(&epic_id)
            .ok_or_else(|| anyhow::anyhow!("Epic with id {} does not exist", epic_id))?;

        epic.status = status;

        self.database.write_db(&db_state)?;
        Ok(())
    }

    /// Updates the status of the specified story in the database.
    /// If the specified story does not exist, an error is returned.
    /// ```rust
    /// use ironyy::{ db::JiraDatabase, models::{Epic, Story, Status} };
    /// use std::fs::File;
    /// use std::io::Write;
    ///
    /// // Remove test file if it exists
    /// if std::path::Path::new("./test_dir/test_db_update_story_status.json").exists() {
    ///   std::fs::remove_file("./test_dir/test_db_update_story_status.json").unwrap();
    /// }
    ///
    /// let db = JiraDatabase::new("./test_dir/test_db_update_story_status.json".to_string()).unwrap();
    /// let epic = Epic::new("Epic 1".to_owned(), "Description of Epic 1".to_owned());
    /// let story = Story::new("Story 1".to_owned(), "Description of Story 1".to_owned());
    ///
    /// let result = db.create_epic(epic);
    /// assert_eq!(result.is_ok(), true);
    ///
    /// let epic_id = result.unwrap();
    ///
    /// let result = db.create_story(story, epic_id);
    /// assert_eq!(result.is_ok(), true);
    ///
    /// let story_id = result.unwrap();
    ///
    /// let result = db.update_story_status(story_id, Status::Closed);
    /// assert_eq!(result.is_ok(), true);
    ///
    /// let db_state = db.read_db().unwrap();
    /// assert_eq!(db_state.stories.get(&story_id).unwrap().status, Status::Closed);
    ///
    /// // Delete the file after the test
    /// std::fs::remove_file("./test_dir/test_db_update_story_status.json").unwrap();
    /// ```
    pub fn update_story_status(&self, story_id: u32, status: Status) -> Result<()> {
        let mut db_state = self.read_db()?;

        let story = db_state
            .stories
            .get_mut(&story_id)
            .ok_or_else(|| anyhow::anyhow!("Story with id {} does not exist", story_id))?;

        story.status = status;

        self.database.write_db(&db_state)?;
        Ok(())
    }
}

/// Trait defining the interface for database backends.
pub trait Database {
    /// Reads the current state of the database.
    fn read_db(&self) -> Result<DBState>;
    /// Writes the given state to the database.
    fn write_db(&self, db_state: &DBState) -> Result<()>;
}

struct JSONFileDatabase {
    pub file_path: String,
}

impl JSONFileDatabase {
    pub fn new(file_path: String) -> Self {
        JSONFileDatabase { file_path }
    }
}

impl Database for JSONFileDatabase {
    fn read_db(&self) -> Result<DBState> {
        let db_content = fs::read_to_string(&self.file_path)?;
        let parsed: DBState = serde_json::from_str(&db_content)?;
        Ok(parsed)
    }

    fn write_db(&self, db_state: &DBState) -> Result<()> {
        fs::write(&self.file_path, &serde_json::to_vec(db_state)?)?;
        Ok(())
    }
}

/// Mock database implementation for testing purposes.
pub mod test_utils {
    use std::{cell::RefCell, collections::HashMap};

    use super::*;

    /// Mock database that stores the last written state in memory.
    /// This is useful for unit tests to avoid file I/O.
    pub struct MockDB {
        last_written_state: RefCell<DBState>,
    }

    impl MockDB {
        /// Creates a new MockDB instance with an initial empty state.
        pub fn new() -> Self {
            Self {
                last_written_state: RefCell::new(DBState {
                    last_item_id: 0,
                    epics: HashMap::new(),
                    stories: HashMap::new(),
                }),
            }
        }
    }

    impl Database for MockDB {
        fn read_db(&self) -> Result<DBState> {
            let state = self.last_written_state.borrow().clone();
            Ok(state)
        }

        fn write_db(&self, db_state: &DBState) -> Result<()> {
            let latest_state = &self.last_written_state;
            *latest_state.borrow_mut() = db_state.clone();
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::test_utils::MockDB;
    use super::*;

    #[test]
    fn create_epic_should_work() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic.clone());

        assert_eq!(result.is_ok(), true);

        let id = result.unwrap();
        let db_state = db.read_db().unwrap();

        let expected_id = 1;

        assert_eq!(id, expected_id);
        assert_eq!(db_state.last_item_id, expected_id);
        assert_eq!(db_state.epics.get(&id), Some(&epic));
    }

    #[test]
    fn create_story_should_error_if_invalid_epic_id() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let story = Story::new("".to_owned(), "".to_owned());

        let non_existent_epic_id = 999;

        let result = db.create_story(story, non_existent_epic_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn create_story_should_work() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.create_story(story.clone(), epic_id);
        assert_eq!(result.is_ok(), true);

        let id = result.unwrap();
        let db_state = db.read_db().unwrap();

        let expected_id = 2;

        assert_eq!(id, expected_id);
        assert_eq!(db_state.last_item_id, expected_id);
        assert_eq!(
            db_state.epics.get(&epic_id).unwrap().stories.contains(&id),
            true
        );
        assert_eq!(db_state.stories.get(&id), Some(&story));
    }

    #[test]
    fn delete_epic_should_error_if_invalid_epic_id() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };

        let non_existent_epic_id = 999;

        let result = db.delete_epic(non_existent_epic_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn delete_epic_should_work() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);
        assert_eq!(result.is_ok(), true);

        let story_id = result.unwrap();

        let result = db.delete_epic(epic_id);
        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();

        let expected_last_id = 2;

        assert_eq!(db_state.last_item_id, expected_last_id);
        assert_eq!(db_state.epics.get(&epic_id), None);
        assert_eq!(db_state.stories.get(&story_id), None);
    }

    #[test]
    fn delete_story_should_error_if_invalid_epic_id() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);
        assert_eq!(result.is_ok(), true);

        let story_id = result.unwrap();

        let non_existent_epic_id = 999;

        let result = db.delete_story(non_existent_epic_id, story_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn delete_story_should_error_if_story_not_found_in_epic() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);
        assert_eq!(result.is_ok(), true);

        let non_existent_story_id = 999;

        let result = db.delete_story(epic_id, non_existent_story_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn delete_story_should_work() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);
        assert_eq!(result.is_ok(), true);

        let story_id = result.unwrap();

        let result = db.delete_story(epic_id, story_id);
        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();

        let expected_last_id = 2;

        assert_eq!(db_state.last_item_id, expected_last_id);
        assert_eq!(
            db_state
                .epics
                .get(&epic_id)
                .unwrap()
                .stories
                .contains(&story_id),
            false
        );
        assert_eq!(db_state.stories.get(&story_id), None);
    }

    #[test]
    fn update_epic_status_should_error_if_invalid_epic_id() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };

        let non_existent_epic_id = 999;

        let result = db.update_epic_status(non_existent_epic_id, Status::Closed);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn update_epic_status_should_work() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);

        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.update_epic_status(epic_id, Status::Closed);

        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();

        assert_eq!(db_state.epics.get(&epic_id).unwrap().status, Status::Closed);
    }

    #[test]
    fn update_story_status_should_error_if_invalid_story_id() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };

        let non_existent_story_id = 999;

        let result = db.update_story_status(non_existent_story_id, Status::Closed);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn update_story_status_should_work() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);

        let story_id = result.unwrap();

        let result = db.update_story_status(story_id, Status::Closed);

        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();

        assert_eq!(
            db_state.stories.get(&story_id).unwrap().status,
            Status::Closed
        );
    }

    mod database {
        use std::collections::HashMap;
        use std::io::Write;

        use super::*;

        #[test]
        fn read_db_should_fail_with_invalid_path() {
            let db = JSONFileDatabase {
                file_path: "INVALID_PATH".to_owned(),
            };
            assert_eq!(db.read_db().is_err(), true);
        }

        #[test]
        fn read_db_should_fail_with_invalid_json() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0 epics: {} stories {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let db = JSONFileDatabase {
                file_path: tmpfile
                    .path()
                    .to_str()
                    .expect("failed to convert tmpfile path to str")
                    .to_string(),
            };

            let result = db.read_db();

            assert_eq!(result.is_err(), true);
        }

        #[test]
        fn read_db_should_parse_json_file() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let db = JSONFileDatabase {
                file_path: tmpfile
                    .path()
                    .to_str()
                    .expect("failed to convert tmpfile path to str")
                    .to_string(),
            };

            let result = db.read_db();

            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn write_db_should_work() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let db = JSONFileDatabase {
                file_path: tmpfile
                    .path()
                    .to_str()
                    .expect("failed to convert tmpfile path to str")
                    .to_string(),
            };

            let story = Story {
                name: "epic 1".to_owned(),
                description: "epic 1".to_owned(),
                status: Status::Open,
            };
            let epic = Epic {
                name: "epic 1".to_owned(),
                description: "epic 1".to_owned(),
                status: Status::Open,
                stories: vec![2],
            };

            let mut stories = HashMap::new();
            stories.insert(2, story);

            let mut epics = HashMap::new();
            epics.insert(1, epic);

            let state = DBState {
                last_item_id: 2,
                epics,
                stories,
            };

            let write_result = db.write_db(&state);
            let read_result = db.read_db().unwrap();

            assert_eq!(write_result.is_ok(), true);
            assert_eq!(read_result, state);
        }
    }
}
