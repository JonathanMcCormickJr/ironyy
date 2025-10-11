#![forbid(unsafe_code)]
use ironyy::{ db, navigator};
use std::rc::Rc;

fn main() {
    let jira_db = db::JiraDatabase::from("../data/db.json".to_string()).expect("Failed to initialize database");
    
    let mut navigator = navigator::Navigator::new(Rc::new(jira_db));
    
    loop {
        clearscreen::clear().unwrap();

        // Get current data from database
        let db_state = navigator.db.read_db().expect("Failed to read database");

        // TODO: implement the following functionality:
        // 1. get current page from navigator. If there is no current page exit the loop.
        let current_page = match navigator.get_current_page() {
            Some(page) => page,
            None => break,
        };
        // 2. render page
        if let Ok(page) = current_page.draw_page() {
            for line in page {
                println!("{}", line);
            }
        } else {
            eprintln!("Failed to render page");
            break;
        };
        // 3. get user input
        // 4. pass input to page's input handler
        // 5. if the page's input handler returns an action let the navigator process the action

        break // TODO: remove this line when implementing the loop functionality
    }
}
