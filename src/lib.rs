#![forbid(missing_docs)]
#![forbid(unsafe_code)]


//! # Ironyy
//! 
//! Ironyy is a command-line tool for project management.
//! 

mod cli;
mod db;
mod models;
mod nav;

/// Run the Ironyy application.
pub fn run() {
    use std::path::PathBuf;
    use crate::db::DBState;
    
    // TODO: Implement the main application logic here.
    
    // Load the database state
    let path: PathBuf = "data.json".into();
    let db_state = DBState::from(path);

    // Initialize the CLI and begin program loop

}