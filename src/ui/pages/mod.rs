use std::{any::Any, rc::Rc};

use anyhow::{Result, anyhow};
use itertools::Itertools;

use crate::db::JiraDatabase;
use crate::models::Action;

mod page_helpers;
use page_helpers::*;

/// Trait representing a UI page.
pub trait Page {
    /// Draws the page to the console.
    fn draw_page(&self) -> Result<Vec<String>, anyhow::Error>;
    /// Handles user input and returns a result including an optional action.
    fn handle_input(&self, input: &str) -> Result<Option<Action>>;
    /// Returns a reference to self as `Any` for downcasting.
    fn as_any(&self) -> &dyn Any;
}

/// The home page displaying a list of epics.
pub struct HomePage {
    /// Reference to the Jira database.
    pub db: Rc<JiraDatabase>,
}
impl Page for HomePage {
    /// Draws the home page with a list of epics.
    /// Returns a Result indicating success or failure.
    ///
    /// ```rust
    /// use ironyy::db::JiraDatabase;
    /// use ironyy::ui::pages::{HomePage, Page};
    /// use std::rc::Rc;
    ///
    /// // Remove test file if it exists
    /// if std::path::Path::new("test_homepage_draw_page.json").exists() {
    ///    std::fs::remove_file("test_homepage_draw_page.json").unwrap();
    /// }
    ///
    /// let jdb = JiraDatabase::new("test_homepage_draw_page.json".to_string());
    /// assert_eq!(jdb.is_ok(), true);
    ///
    /// // Add an epic to the database
    /// let mut epic0 = ironyy::models::Epic::new("Epic - Project 1".to_owned(), "This is Project 1 for the first epic!!!".to_owned());
    /// epic0.status = ironyy::models::Status::InProgress;
    /// jdb.as_ref().unwrap().create_epic(epic0).unwrap();
    ///
    /// // Add 2 stories to the epic
    /// let story0 = ironyy::models::Story::new("Story - Project 1 Solution".to_owned(), "This is Task 1 for the first story!!!".to_owned());
    /// let story1 = ironyy::models::Story::new("Story - Project 1 README".to_owned(), "This is Task 2 for the first story!!!".to_owned());
    /// jdb.as_ref().unwrap().create_story(story0, 1).unwrap();
    /// jdb.as_ref().unwrap().create_story(story1, 1).unwrap();
    ///
    /// // Close the first story and resolve the second story
    /// jdb.as_ref().unwrap().update_story_status(2, ironyy::models::Status::Closed).unwrap();
    /// jdb.as_ref().unwrap().update_story_status(3, ironyy::models::Status::Resolved).unwrap();
    ///
    /// // Add another epic to the database
    /// let epic1 = ironyy::models::Epic::new("Epic - Project 2".to_owned(), "This is Project 2 for the second epic!!!".to_owned());
    /// jdb.as_ref().unwrap().create_epic(epic1).unwrap();
    ///
    /// let page = HomePage { db: jdb.unwrap().into() };
    /// let draw_result = page.draw_page();
    /// assert_eq!(draw_result.is_ok(), true);
    /// assert_eq!(draw_result.unwrap(), vec![
    ///     "----------------------------- EPICS -----------------------------".to_string(),
    ///     "     id     |               name               |      status      ".to_string(),
    ///     "1           | Epic - Project 1                 | IN PROGRESS     ".to_string(),
    ///     "4           | Epic - Project 2                 | OPEN            ".to_string(),
    ///     "".to_string(),
    ///     "".to_string(),
    ///     "[q] quit | [c] create epic | [:id:] navigate to epic".to_string(),
    /// ]);
    /// 
    /// // Clean up test file
    /// std::fs::remove_file("test_homepage_draw_page.json").unwrap();
    ///
    /// ```
    fn draw_page(&self) -> Result<Vec<String>, anyhow::Error> {
        let mut vec_of_lines: Vec<String> = Vec::new();

        vec_of_lines.push(String::from(
            "----------------------------- EPICS -----------------------------",
        ));
        vec_of_lines.push(String::from(
            "     id     |               name               |      status      ",
        ));

        let db_state = self.db.read_db()?;
        let epics = &db_state.epics;
        let epics_formatted_lines: Vec<String> = epics
            .into_iter()
            .map(|(id, epic)| {
                format!(
                    "{} | {} | {}",
                    get_column_string(&id.to_string(), 11),
                    get_column_string(&epic.name, 32),
                    get_column_string(&epic.status.to_string(), 16)
                )
            })
            .sorted()
            .collect();

        for line in epics_formatted_lines {
            vec_of_lines.push(line);
        }

        vec_of_lines.push(String::new());
        vec_of_lines.push(String::new());

        vec_of_lines.push(String::from(
            "[q] quit | [c] create epic | [:id:] navigate to epic",
        ));

        vec_of_lines.iter().for_each(|line| println!("{}", line));
        Ok(vec_of_lines)
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        match input {
            "q" => Ok(Some(Action::Exit)),
            "c" => Ok(Some(Action::CreateEpic)),
            id_str => {
                if let Ok(id) = id_str.parse::<u32>() {
                    let db_state = self.db.read_db()?;
                    if db_state.epics.contains_key(&id) {
                        Ok(Some(Action::NavigateToEpicDetail { epic_id: id }))
                    } else {
                        Ok(None)
                    }
                } else {
                    Ok(None)
                }
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// The detail page for a specific epic, showing its description and associated stories.
pub struct EpicDetail {
    /// The ID of the epic to display.
    pub epic_id: u32,
    /// Reference to the Jira database.
    pub db: Rc<JiraDatabase>,
}

impl Page for EpicDetail {
    /// Draws the epic detail page with its stories.
    /// Returns a Result indicating success or failure.
    ///
    /// ```rust
    /// use ironyy::db::JiraDatabase;
    /// use ironyy::ui::pages::{EpicDetail, Page};
    /// use std::rc::Rc;
    ///
    /// // Remove test file if it exists
    /// if std::path::Path::new("test_epic_detail_draw_page.json").exists() {
    ///    std::fs::remove_file("test_epic_detail_draw_page.json").unwrap();
    /// }
    /// let jdb = JiraDatabase::new("test_epic_detail_draw_page.json".to_string());
    /// assert_eq!(jdb.is_ok(), true);
    ///
    /// // Add an epic to the database
    /// let mut epic0 = ironyy::models::Epic::new("Epic - Project 1".to_owned(), "This is Project 1 for the first epic!!!".to_owned());
    /// epic0.status = ironyy::models::Status::InProgress;
    /// jdb.as_ref().unwrap().create_epic(epic0).unwrap();
    ///
    /// // Add 2 stories to the epic
    /// let story0 = ironyy::models::Story::new("Story - Project 1 Solution".to_owned(), "This is Task 1 for the first story!!!".to_owned());
    /// let story1 = ironyy::models::Story::new("Story - Project 1 README".to_owned(), "This is Task 2 for the first story!!!".to_owned());
    /// jdb.as_ref().unwrap().create_story(story0, 1).unwrap();
    /// jdb.as_ref().unwrap().create_story(story1, 1).unwrap();
    ///
    /// // Close the first story and resolve the second story
    /// jdb.as_ref().unwrap().update_story_status(2, ironyy::models::Status::Closed).unwrap();
    /// jdb.as_ref().unwrap().update_story_status(3, ironyy::models::Status::Resolved).unwrap();
    ///
    /// let page = EpicDetail { epic_id: 1, db: jdb.unwrap().into() };
    /// let draw_result = page.draw_page();
    /// assert_eq!(draw_result.is_ok(), true);
    /// assert_eq!(draw_result.unwrap(), vec![
    ///     "------------------------------ EPIC ------------------------------".to_string(),
    ///     "  id  |     name     |         description         |    status    ".to_string(),
    ///     "1     | Epic - Pr... | This is Project 1 for th... | IN PROGRESS ".to_string(),
    ///     "".to_string(),
    ///     "---------------------------- STORIES ----------------------------".to_string(),
    ///     "     id     |               name               |      status      ".to_string(),
    ///     "2           | Story - Project 1 Solution       | CLOSED          ".to_string(),
    ///     "3           | Story - Project 1 README         | RESOLVED        ".to_string(),
    ///     "".to_string(),
    ///     "".to_string(),
    ///     "[p] previous | [u] update epic | [d] delete epic | [c] create story | [:id:] navigate to story".to_string()
    /// ]);
    /// 
    /// // Clean up test file
    /// std::fs::remove_file("test_epic_detail_draw_page.json").unwrap();
    /// ```
    fn draw_page(&self) -> Result<Vec<String>, anyhow::Error> {
        let mut vec_of_lines: Vec<String> = Vec::new();

        vec_of_lines.push(String::from(
            "------------------------------ EPIC ------------------------------",
        ));
        vec_of_lines.push(String::from(
            "  id  |     name     |         description         |    status    ",
        ));

        let db_state = self.db.read_db()?;
        let epic = db_state
            .epics
            .get(&self.epic_id)
            .ok_or_else(|| anyhow!("could not find epic!"))?;

        vec_of_lines.push(format!(
            "{} | {} | {} | {}",
            get_column_string(&self.epic_id.to_string(), 5),
            get_column_string(&epic.name, 12),
            get_column_string(&epic.description, 27),
            get_column_string(&epic.status.to_string(), 12)
        ));

        vec_of_lines.push(String::new());

        vec_of_lines.push(String::from(
            "---------------------------- STORIES ----------------------------",
        ));
        vec_of_lines.push(String::from(
            "     id     |               name               |      status      ",
        ));

        let stories = &db_state.stories;

        for story_id in epic.stories.iter() {
            if let Some(story) = stories.get(story_id) {
                vec_of_lines.push(format!(
                    "{} | {} | {}",
                    get_column_string(&story_id.to_string(), 11),
                    get_column_string(&story.name, 32),
                    get_column_string(&story.status.to_string(), 16)
                ));
            }
        }

        vec_of_lines.push(String::new());
        vec_of_lines.push(String::new());

        vec_of_lines.push(String::from(
            "[p] previous | [u] update epic | [d] delete epic | [c] create story | [:id:] navigate to story"
        ));

        vec_of_lines.iter().for_each(|line| println!("{}", line));

        Ok(vec_of_lines)
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        match input {
            "p" => Ok(Some(Action::NavigateToPreviousPage)),
            "u" => Ok(Some(Action::UpdateEpicStatus {
                epic_id: self.epic_id,
            })),
            "d" => Ok(Some(Action::DeleteEpic {
                epic_id: self.epic_id,
            })),
            "c" => Ok(Some(Action::CreateStory {
                epic_id: self.epic_id,
            })),
            id_str => {
                if let Ok(id) = id_str.parse::<u32>() {
                    let db_state = self.db.read_db()?;
                    if db_state.stories.contains_key(&id) {
                        Ok(Some(Action::NavigateToStoryDetail { story_id: id }))
                    } else {
                        Ok(None)
                    }
                } else {
                    Ok(None)
                }
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// The detail page for a specific story, showing its description and status.
pub struct StoryDetail {
    /// The ID of the story to display.
    pub story_id: u32,
    /// Reference to the Jira database.
    pub db: Rc<JiraDatabase>,
}

impl Page for StoryDetail {
    /// Draws the story detail page.
    /// Returns a Result indicating success or failure.
    ///
    /// ```rust
    /// use ironyy::db::JiraDatabase;
    /// use ironyy::ui::pages::{StoryDetail, Page};
    /// use std::rc::Rc;
    ///
    /// // Remove test file if it exists
    /// if std::path::Path::new("test_story_detail_draw_page.json").exists() {
    ///    std::fs::remove_file("test_story_detail_draw_page.json").unwrap();
    /// }
    /// let jdb = JiraDatabase::new("test_story_detail_draw_page.json".to_string());
    /// assert_eq!(jdb.is_ok(), true);
    ///
    /// // Add an epic to the database
    /// let mut epic0 = ironyy::models::Epic::new("Epic - Project 1".to_owned(), "This is Project 1 for the first epic!!!".to_owned());
    /// epic0.status = ironyy::models::Status::InProgress;
    /// jdb.as_ref().unwrap().create_epic(epic0).unwrap();
    ///
    /// // Add 2 stories to the epic
    /// let story0 = ironyy::models::Story::new("Story - Project 1 Solution".to_owned(), "Please provide full implementation of this stuff.".to_owned());
    /// let story1 = ironyy::models::Story::new("Story - Project 1 README".to_owned(), "This is Task 2 for the first story!!!".to_owned());
    /// jdb.as_ref().unwrap().create_story(story0, 1).unwrap();
    /// jdb.as_ref().unwrap().create_story(story1, 1).unwrap();
    ///
    /// // Close the first story and resolve the second story
    /// jdb.as_ref().unwrap().update_story_status(2, ironyy::models::Status::Closed).unwrap();
    /// jdb.as_ref().unwrap().update_story_status(3, ironyy::models::Status::Resolved).unwrap();
    ///
    /// let page = StoryDetail {story_id: 2, db: jdb.unwrap().into() };
    /// let draw_result = page.draw_page();
    /// assert_eq!(draw_result.is_ok(), true);
    /// assert_eq!(draw_result.unwrap(), vec![
    ///     "------------------------------ STORY ------------------------------".to_string(),
    ///     "  id  |     name     |         description         |    status    ".to_string(),
    ///     "2     | Story - P... | Please provide full impl... | CLOSED       ".to_string(),
    ///     "".to_string(),
    ///     "".to_string(),
    ///     "[p] previous | [u] update story | [d] delete story".to_string()
    /// ]);
    /// 
    /// // Clean up test file
    /// std::fs::remove_file("test_story_detail_draw_page.json").unwrap();
    /// ```
    fn draw_page(&self) -> Result<Vec<String>, anyhow::Error> {
        let mut vec_of_lines: Vec<String> = Vec::new();

        vec_of_lines.push(String::from(
            "------------------------------ STORY ------------------------------",
        ));
        vec_of_lines.push(String::from(
            "  id  |     name     |         description         |    status    ",
        ));

        let db_state = self.db.read_db()?;
        let story = db_state
            .stories
            .get(&self.story_id)
            .ok_or_else(|| anyhow!("could not find story!"))?;

        vec_of_lines.push(format!(
            "{} | {} | {} | {}",
            get_column_string(&self.story_id.to_string(), 5),
            get_column_string(&story.name, 12),
            get_column_string(&story.description, 27),
            get_column_string(&story.status.to_string(), 13)
        ));

        vec_of_lines.push(String::new());
        vec_of_lines.push(String::new());

        vec_of_lines.push(String::from(
            "[p] previous | [u] update story | [d] delete story",
        ));

        vec_of_lines.iter().for_each(|line| println!("{}", line));

        Ok(vec_of_lines)
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        match input {
            "p" => Ok(Some(Action::NavigateToPreviousPage)),
            "u" => Ok(Some(Action::UpdateStoryStatus {
                story_id: self.story_id,
            })),
            "d" => Ok(Some(Action::DeleteStory {
                story_id: self.story_id,
            })),
            _ => Ok(None),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::test_utils::MockDB;
    use crate::models::{Epic, Story};

    mod home_page {
        use super::*;

        #[test]
        fn draw_page_should_not_throw_error() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });

            let page = HomePage { db };
            assert_eq!(page.draw_page().is_ok(), true);
        }

        #[test]
        fn handle_input_should_not_throw_error() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });

            let page = HomePage { db };
            assert_eq!(page.handle_input("").is_ok(), true);
        }

        #[test]
        fn handle_input_should_return_the_correct_actions() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });

            let epic = Epic::new("".to_owned(), "".to_owned());

            let epic_id = db.create_epic(epic).unwrap();

            let page = HomePage { db };

            let q = "q";
            let c = "c";
            let valid_epic_id = epic_id.to_string();
            let invalid_epic_id = "999";
            let junk_input = "j983f2j";
            let junk_input_with_valid_prefix = "q983f2j";
            let input_with_trailing_white_spaces = "q\n";

            assert_eq!(page.handle_input(q).unwrap(), Some(Action::Exit));
            assert_eq!(page.handle_input(c).unwrap(), Some(Action::CreateEpic));
            assert_eq!(
                page.handle_input(&valid_epic_id).unwrap(),
                Some(Action::NavigateToEpicDetail { epic_id: 1 })
            );
            assert_eq!(page.handle_input(invalid_epic_id).unwrap(), None);
            assert_eq!(page.handle_input(junk_input).unwrap(), None);
            assert_eq!(
                page.handle_input(junk_input_with_valid_prefix).unwrap(),
                None
            );
            assert_eq!(
                page.handle_input(input_with_trailing_white_spaces).unwrap(),
                None
            );
        }
    }

    mod epic_detail_page {
        use super::*;

        #[test]
        fn draw_page_should_not_throw_error() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });
            let epic_id = db
                .create_epic(Epic::new("".to_owned(), "".to_owned()))
                .unwrap();

            let page = EpicDetail { epic_id, db };
            assert_eq!(page.draw_page().is_ok(), true);
        }

        #[test]
        fn handle_input_should_not_throw_error() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });
            let epic_id = db
                .create_epic(Epic::new("".to_owned(), "".to_owned()))
                .unwrap();

            let page = EpicDetail { epic_id, db };
            assert_eq!(page.handle_input("").is_ok(), true);
        }

        #[test]
        fn draw_page_should_throw_error_for_invalid_epic_id() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });

            let page = EpicDetail { epic_id: 999, db };
            assert_eq!(page.draw_page().is_err(), true);
        }

        #[test]
        fn handle_input_should_return_the_correct_actions() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });

            let epic_id = db
                .create_epic(Epic::new("".to_owned(), "".to_owned()))
                .unwrap();
            let story_id = db
                .create_story(Story::new("".to_owned(), "".to_owned()), epic_id)
                .unwrap();

            let page = EpicDetail { epic_id, db };

            let p = "p";
            let u = "u";
            let d = "d";
            let c = "c";
            let invalid_story_id = "999";
            let junk_input = "j983f2j";
            let junk_input_with_valid_prefix = "p983f2j";
            let input_with_trailing_white_spaces = "p\n";

            assert_eq!(
                page.handle_input(p).unwrap(),
                Some(Action::NavigateToPreviousPage)
            );
            assert_eq!(
                page.handle_input(u).unwrap(),
                Some(Action::UpdateEpicStatus { epic_id: 1 })
            );
            assert_eq!(
                page.handle_input(d).unwrap(),
                Some(Action::DeleteEpic { epic_id: 1 })
            );
            assert_eq!(
                page.handle_input(c).unwrap(),
                Some(Action::CreateStory { epic_id: 1 })
            );
            assert_eq!(
                page.handle_input(&story_id.to_string()).unwrap(),
                Some(Action::NavigateToStoryDetail { story_id: 2 })
            );
            assert_eq!(page.handle_input(invalid_story_id).unwrap(), None);
            assert_eq!(page.handle_input(junk_input).unwrap(), None);
            assert_eq!(
                page.handle_input(junk_input_with_valid_prefix).unwrap(),
                None
            );
            assert_eq!(
                page.handle_input(input_with_trailing_white_spaces).unwrap(),
                None
            );
        }
    }

    mod story_detail_page {
        use super::*;

        #[test]
        fn draw_page_should_not_throw_error() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });

            let epic_id = db
                .create_epic(Epic::new("".to_owned(), "".to_owned()))
                .unwrap();
            let story_id = db
                .create_story(Story::new("".to_owned(), "".to_owned()), epic_id)
                .unwrap();

            let page = StoryDetail {
                story_id,
                db,
            };
            assert_eq!(page.draw_page().is_ok(), true);
        }

        #[test]
        fn handle_input_should_not_throw_error() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });

            let epic_id = db
                .create_epic(Epic::new("".to_owned(), "".to_owned()))
                .unwrap();
            let story_id = db
                .create_story(Story::new("".to_owned(), "".to_owned()), epic_id)
                .unwrap();

            let page = StoryDetail {
                story_id,
                db,
            };
            assert_eq!(page.handle_input("").is_ok(), true);
        }

        #[test]
        fn draw_page_should_throw_error_for_invalid_story_id() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });

            let epic_id = db
                .create_epic(Epic::new("".to_owned(), "".to_owned()))
                .unwrap();
            let _ = db
                .create_story(Story::new("".to_owned(), "".to_owned()), epic_id)
                .unwrap();

            let page = StoryDetail {
                story_id: 999,
                db,
            };
            assert_eq!(page.draw_page().is_err(), true);
        }

        #[test]
        fn handle_input_should_return_the_correct_actions() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });

            let epic_id = db
                .create_epic(Epic::new("".to_owned(), "".to_owned()))
                .unwrap();
            let story_id = db
                .create_story(Story::new("".to_owned(), "".to_owned()), epic_id)
                .unwrap();

            let page = StoryDetail {
                story_id,
                db,
            };

            let p = "p";
            let u = "u";
            let d = "d";
            let some_number = "1";
            let junk_input = "j983f2j";
            let junk_input_with_valid_prefix = "p983f2j";
            let input_with_trailing_white_spaces = "p\n";

            assert_eq!(
                page.handle_input(p).unwrap(),
                Some(Action::NavigateToPreviousPage)
            );
            assert_eq!(
                page.handle_input(u).unwrap(),
                Some(Action::UpdateStoryStatus { story_id })
            );
            assert_eq!(
                page.handle_input(d).unwrap(),
                Some(Action::DeleteStory { story_id })
            );
            assert_eq!(page.handle_input(some_number).unwrap(), None);
            assert_eq!(page.handle_input(junk_input).unwrap(), None);
            assert_eq!(
                page.handle_input(junk_input_with_valid_prefix).unwrap(),
                None
            );
            assert_eq!(
                page.handle_input(input_with_trailing_white_spaces).unwrap(),
                None
            );
        }
    }
}
