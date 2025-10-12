//! Navigation module for Ironyy.
//! 
//! This module handles the navigation logic within the Ironyy application.
//! 
//! Each page in the application implements the `Page` trait, allowing for a consistent
//! interface for navigation actions. The `Navigator` struct manages the stack of pages
//! and provides methods to navigate between them.

use std::{rc::Rc, cell::RefCell};

use crate::{cli::PageType, db::DBState};

pub struct NavState {
    pages: Vec<PageType>,
    pub db: Rc<RefCell<crate::db::DBState>>,
}
