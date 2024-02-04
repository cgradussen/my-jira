use std::rc::Rc;

mod models;

mod db;
use db::*;

mod ui;

mod io_utils;
use io_utils::*;

mod navigator;
use navigator::*;

fn main() {
    let db = Rc::new(JiraDatabase::new("./data/db.json".to_string()));
    let mut nav = Navigator::new(db);

    clearscreen::clear().unwrap();

    loop {
        // 1. get current page from navigator. If there is no current page exit the loop.
        let page = match nav.get_current_page() {
            None => break,
            Some(page) => page,
        };
        // 2. render page
        if let Err(error) = page.draw_page() {
            println!(
                "Error rendering page: {}\nPress any key to continue...",
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
            Ok(action) => match action {
                Some(action) => {
                    let _ = nav.handle_action(action);
                }
                None => {}
            },
            Err(_e) => {
                println!("Input doesn't lead to any action.");
            }
        }
    }
}
