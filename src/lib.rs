#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

//! # Ironyy
//! A simple command-line tool for managing epics and stories in a project.
//! It allows you to create, update, and view epics and stories, as well as track their
//! status. The data is stored in a JSON file for easy access and modification.

/// Database operations and interactions.
pub mod db;

/// Input/output utility functions.
pub mod io_utils;

/// Data models for epics, stories, and their statuses.
pub mod models;

/// Navigation and page management.
pub mod navigator;

/// User interface components and pages.
pub mod ui;
