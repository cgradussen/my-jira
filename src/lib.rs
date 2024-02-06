/// The entry point for the my-jira CLI app.
///
// library modules:
mod db;
mod io_utils;
mod models;
mod navigator;
mod ui;

// namespace
use db::*;
use io_utils::*;
use navigator::*;
use std::rc::Rc;

/// entry point of application
pub fn run() {
    let db = Rc::new(JiraDatabase::new("./data/db.json".to_string()));
    let mut nav = Navigator::new(db);

    loop {
        // 1. get current page from navigator. If there is no current page exit the loop.
        clearscreen::clear().unwrap();
        let page = match nav.get_current_page() {
            None => break,
            Some(page) => page,
        };
        // 2. render page
        if let Err(error) = page.draw_page() {
            println!(
                "{RED}Error rendering page: {}\nPress enter key to continue...{DFT}",
                error
            );
            wait_for_key_press();
        };
        // 3. get user input
        let input: String = get_user_input().trim().into();
        // 4. pass input to page's input handler
        let result = page.handle_input(input.as_str());
        // 5. if the page's input handler returns an action let the navigator process the action
        match result {
            Ok(action) => {
                if let Some(action) = action {
                    if let Err(error) = nav.handle_action(action) {
                        println!(
                            "{RED}Error handle action: {}\nPress enter key to continue...{DFT}",
                            error
                        );
                        wait_for_key_press();
                    }
                }
            }
            Err(_e) => {
                println!("Input doesn't lead to any action.");
            }
        }
    }
}

// #########.#########.#########.#########.#########.#########.#########.#########.#########.#########.
#[cfg(test)]
mod tests {
    use super::*;
    use models::{DBState, Epic, Status, Story};
    use std::collections::HashMap;
    use std::fs::{File, OpenOptions};
    use std::io::BufReader;

    #[test]
    fn test_write_db_with_entries() {
        let mut epics = HashMap::new();
        let mut stories = HashMap::new();

        let mut epic = Epic::new(
            "Epic - Project 1".into(),
            "This is Project 1 for the Bootcamp".into(),
        );
        epic.status = Status::InProgress;
        epic.stories = vec![2, 3];
        epics.insert(1, epic);

        let mut story = Story::new(
            "Story - Project 1 Solution".into(),
            "Please provide full implement for Project 1".into(),
        );
        story.status = Status::Closed;
        stories.insert(2, story);

        let mut story = Story::new(
            "Story - Project 1 README".into(),
            "Please create README file for Project 1".into(),
        );
        story.status = Status::InProgress;
        stories.insert(3, story);

        let db = DBState {
            last_item_id: 3,
            epics: epics,
            stories: stories,
        };

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("./src/data/foo2.json")
            .expect("failure to open file for write");

        serde_json::to_writer_pretty(&file, &db).expect("failure to serialiaze to JSON");
    }

    #[test]
    fn test_read_db() {
        let file = File::open("./src/data/foo2.json").expect("failure to open file for read");
        let reader = BufReader::new(file);

        // Read the JSON contents of the file as an instance of `DBState`.
        let u: DBState =
            serde_json::from_reader(reader).expect("failure to deserialiaze from JSON");

        println!("{u:?}");
        //let content: Result<DBState> = serde_json::from_reader(&file); //, &db).expect("failure to serialiaze to JSON");
    }
}
