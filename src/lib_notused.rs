pub mod db;
pub mod io_utils;
pub mod models;
pub mod navigator;
pub mod ui;

pub fn run() {
    println!("run");
}
ccc
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
