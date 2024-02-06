/// missing test cases
///
/// Change log:
/// entery of name and description is on same line (still remove unwrap?)
/// removed bugs in Y/n and input of name and description
use std::io;
use std::io::Write;

use crate::{
    io_utils::{get_user_input, DFT, RED},
    models::{Epic, Status, Story},
};

const QUERY_COLOR: &str = RED;
const SEPERATOR_LINE_WIDTH: usize = 100;

pub struct Prompts {
    pub create_epic: Box<dyn Fn() -> Epic>,
    pub create_story: Box<dyn Fn() -> Story>,
    pub delete_epic: Box<dyn Fn() -> bool>,
    pub delete_story: Box<dyn Fn() -> bool>,
    pub update_status: Box<dyn Fn() -> Option<Status>>,
}

impl Prompts {
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

fn get_keyboard_input(message: &str) -> String {
    let text = message.to_string();

    let before: String = format!("[{QUERY_COLOR}");
    let after = format!("{DFT}]");

    let text = text.replace('[', &before);
    let text = text.replace(']', &after);

    print!("{text}");
    io::stdout().flush().unwrap();
    get_user_input().trim().to_string()
}

fn create_epic_prompt() -> Epic {
    println!(
        "{QUERY_COLOR}{:-<width$}{DFT}",
        "",
        width = SEPERATOR_LINE_WIDTH
    );
    let name = get_keyboard_input("Epic Name       :");
    let description = get_keyboard_input("Epic Description:");
    Epic::new(name, description)
}

fn create_story_prompt() -> Story {
    println!(
        "{QUERY_COLOR}{:-<width$}{DFT}",
        "",
        width = SEPERATOR_LINE_WIDTH
    );
    let name = get_keyboard_input("Story Name       :");
    let description = get_keyboard_input("Story Description:");
    Story::new(name, description)
}

fn delete_epic_prompt() -> bool {
    println!(
        "{QUERY_COLOR}{:-<width$}{DFT}",
        "",
        width = SEPERATOR_LINE_WIDTH
    );
    get_keyboard_input("Are you sure you want to delete this epic? All stories in this epic will also be deleted [Y/n]:")== "Y"
}

fn delete_story_prompt() -> bool {
    println!(
        "{QUERY_COLOR}{:-<width$}{DFT}",
        "",
        width = SEPERATOR_LINE_WIDTH
    );
    get_keyboard_input("Are you sure you want to delete this story? [Y/n]:") == "Y"
}

fn update_status_prompt() -> Option<Status> {
    println!(
        "{QUERY_COLOR}{:-<width$}{DFT}",
        "",
        width = SEPERATOR_LINE_WIDTH
    );
    match get_keyboard_input(
        "New Status ([1] - OPEN, [2] - IN-PROGRESS, [3] - RESOLVED, [4] - CLOSED):",
    )
    .as_str()
    {
        "1" => Some(Status::Open),
        "2" => Some(Status::InProgress),
        "3" => Some(Status::Resolved),
        "4" => Some(Status::Closed),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn dummy_test() {
        todo!("make tests to overtake the keyboard");
    }
}
