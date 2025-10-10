use anyhow::Ok;

use crate::{
    io_utils::get_user_input,
    models::{Epic, Status, Story},
};

#[allow(missing_docs)]
/// Struct containing various user prompts for interacting with epics and stories.
pub struct Prompts {
    pub create_epic: Box<dyn Fn() -> Result<Epic, anyhow::Error>>,
    pub create_story: Box<dyn Fn() -> Result<Story, anyhow::Error>>,
    pub delete_epic: Box<dyn Fn() -> Result<bool, anyhow::Error>>,
    pub delete_story: Box<dyn Fn() -> Result<bool, anyhow::Error>>,
    pub update_status: Box<dyn Fn() -> Result<Option<Status>, anyhow::Error>>,
}

impl Prompts {
    /// Creates a new instance of `Prompts` with default prompt functions.
    pub fn new() -> Self {
        Self {
            create_epic: Box::new(create_epic_prompt),
            create_story: Box::new(create_story_prompt),
            delete_epic: Box::new(delete_epic_prompt),
            delete_story: Box::new(delete_story_prompt),
            update_status: Box::new(update_status_prompt),
        }
    }
}

fn create_epic_prompt() -> Result<Epic, anyhow::Error> {
    println!("----------------------------");
    println!("Epic Name:");
    let name: String = get_user_input()?;

    println!("Epic Description:");
    let description: String = get_user_input()?;

    Ok(Epic::new(name, description))
}

fn create_story_prompt() -> Result<Story, anyhow::Error> {
    println!("----------------------------");
    println!("Story Name:");
    let name: String = get_user_input()?;

    println!("Story Description:");
    let description: String = get_user_input()?;

    Ok(Story::new(name, description))
}

fn delete_epic_prompt() -> Result<bool, anyhow::Error> {
    println!("----------------------------");
    println!(
        "Are you sure you want to delete this epic? All stories in this epic will also be deleted [Y/n]:"
    );
    let confirmation: String = get_user_input()?;

    Ok(confirmation.to_lowercase() == "y" || confirmation.is_empty())
}

fn delete_story_prompt() -> Result<bool, anyhow::Error> {
    println!("----------------------------");
    println!("Are you sure you want to delete this story? [Y/n]: Y");
    let confirmation: String = get_user_input()?;

    Ok(confirmation.to_lowercase() == "y" || confirmation.is_empty())
}

fn update_status_prompt() -> Result<Option<Status>, anyhow::Error> {
    println!("----------------------------");
    println!("New Status (1 - OPEN, 2 - IN-PROGRESS, 3 - RESOLVED, 4 - CLOSED):");
    let status_input = get_user_input()?;
    let numeric = status_input.parse::<u8>().ok().unwrap_or(0);

    let status = match numeric {
        1 => Status::Open,
        2 => Status::InProgress,
        3 => Status::Resolved,
        4 => Status::Closed,
        _ => return Ok(None),
    };
    Ok(Some(status))
}
