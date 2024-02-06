///
/// Notes:
/// Refactor: duplicate code in handle inputs.
/// Refactor: tests for screen output, they don't generate error when not doing anything.
mod page_helpers;

use crate::db::JiraDatabase;
use crate::io_utils::{DFT, GREEN, RED};
use crate::models::Action;
use anyhow::anyhow;
use anyhow::Result;
use itertools::Itertools;
use page_helpers::get_column_string;
use std::any::Any;
use std::rc::Rc;

const HEADER_COLOR: &str = GREEN;
const QUERY_COLOR: &str = RED;

const TERMINAL_WIDTH: usize = 100;
const ID_WIDTH: usize = 8;
const NAME_WIDTH: usize = 20;
const STATUS_WIDTH: usize = 12;

pub trait Page {
    fn draw_page(&self) -> Result<()>;
    fn handle_input(&self, input: &str) -> Result<Option<Action>>;
    fn as_any(&self) -> &dyn Any;
}

pub struct HomePage {
    pub db: Rc<JiraDatabase>,
}

fn print_query(text: &str) -> Result<()> {
    let text = text.to_string();

    let before: String = format!("[{QUERY_COLOR}");
    let after = format!("{DFT}]");

    let text = text.replace('[', &before);
    let text = text.replace(']', &after);

    println!("{text}");

    Ok(())
}

impl Page for HomePage {
    fn draw_page(&self) -> Result<()> {
        let name_width: usize = TERMINAL_WIDTH - ID_WIDTH - STATUS_WIDTH - 3;

        println!(
            "{}{:-^width$}",
            HEADER_COLOR,
            " EPICS ",
            width = TERMINAL_WIDTH
        );
        println!(
            "{: ^id_width$}|{: ^name_width$}| {: ^status_width$}{dft}",
            "id",
            "name",
            "status",
            id_width = ID_WIDTH,
            name_width = name_width,
            status_width = STATUS_WIDTH,
            dft = DFT
        );

        // print out epics using get_column_string(). also make sure the epics are sorted by id
        let db_state = self.db.database.read_db()?;
        let epics_iter = db_state.epics.iter().sorted_by_key(|x| x.0);

        for (&id, epic) in epics_iter {
            println!(
                "{:<id_width$}{HEADER_COLOR}|{DFT}{}{HEADER_COLOR}|{DFT} {}",
                id,
                get_column_string(&epic.name, name_width),
                get_column_string(format!("{}", epic.status).as_str(), STATUS_WIDTH),
                id_width = ID_WIDTH,
            );
        }

        println!();
        println!();

        print_query("[q] quit | [c] create epic | [:id:] navigate to epic")
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        // match against the user input and return the corresponding action. If the user input was invalid return None.
        match input {
            "" => Ok(None),
            "q" => Ok(Some(Action::Exit)),
            "c" => Ok(Some(Action::CreateEpic)),
            _ => {
                if input.parse::<u32>().is_err() {
                    return Ok(None);
                }

                let epic_id = input.parse::<u32>()?;
                let db_state = self.db.database.read_db()?;

                if db_state.epics.get(&epic_id).is_none() {
                    return Ok(None);
                }
                Ok(Some(Action::NavigateToEpicDetail { epic_id }))
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct EpicDetail {
    pub epic_id: u32,
    pub db: Rc<JiraDatabase>,
}

impl Page for EpicDetail {
    fn draw_page(&self) -> Result<()> {
        let name_width: usize = TERMINAL_WIDTH - ID_WIDTH - STATUS_WIDTH - 3;
        let description_width = TERMINAL_WIDTH - ID_WIDTH - STATUS_WIDTH - NAME_WIDTH - 4;

        let db_state = self.db.read_db()?;
        let epic = db_state
            .epics
            .get(&self.epic_id)
            .ok_or_else(|| anyhow!("could not find epic!"))?;

        println!(
            "{}{:-^width$}",
            HEADER_COLOR,
            " EPIC ",
            width = TERMINAL_WIDTH,
        );
        println!(
            "{: ^id_width$}|{: ^name_width$}|{: ^description_width$}| {: ^status_width$}{dft}",
            "id",
            "name",
            "description",
            "status",
            id_width = ID_WIDTH,
            name_width = NAME_WIDTH,
            description_width = description_width,
            status_width = STATUS_WIDTH,
            dft = DFT
        );

        // print out epic details using get_column_string()
        println!(
            "{:<id_width$}{HEADER_COLOR}|{DFT}{}{HEADER_COLOR}|{DFT}{}{HEADER_COLOR}|{DFT} {}",
            &self.epic_id,
            get_column_string(&epic.name, NAME_WIDTH),
            get_column_string(&epic.description, description_width),
            get_column_string(format!("{}", epic.status).as_str(), STATUS_WIDTH),
            id_width = ID_WIDTH
        );

        println!();

        println!(
            "{}{:-^width$}",
            HEADER_COLOR,
            " STORIES ",
            width = TERMINAL_WIDTH
        );
        println!(
            "{: ^id_width$}|{: ^name_width$}|{: ^status_width$} {dft}",
            "id",
            "name",
            "status",
            id_width = ID_WIDTH,
            name_width = name_width,
            status_width = STATUS_WIDTH,
            dft = DFT
        );

        let stories = &db_state.stories;
        // print out stories using get_column_string(). also make sure the stories are sorted by id
        let stories_iter = stories.iter().sorted_by_key(|x| x.0);
        for (&id, story) in stories_iter {
            // filter out the items for the given epic
            if epic.stories.contains(&id) {
                println!(
                    "{:<id_width$}{HEADER_COLOR}|{DFT}{}{HEADER_COLOR}|{DFT} {}",
                    id,
                    get_column_string(&story.name, name_width),
                    get_column_string(format!("{}", story.status).as_str(), STATUS_WIDTH),
                    id_width = ID_WIDTH,
                );
            }
        }

        println!();
        println!();

        print_query("[p] previous | [u] update epic | [d] delete epic | [c] create story | [:id:] navigate to story")
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        // match against the user input and return the corresponding action. If the user input was invalid return None.
        let epic_id = self.epic_id;

        match input {
            "" => Ok(None),
            "p" => Ok(Some(Action::NavigateToPreviousPage)),
            "u" => Ok(Some(Action::UpdateEpicStatus { epic_id })),
            "d" => Ok(Some(Action::DeleteEpic { epic_id })),
            "c" => Ok(Some(Action::CreateStory { epic_id })),
            _ => {
                if input.parse::<u32>().is_err() {
                    return Ok(None);
                }

                let story_id = input.parse::<u32>()?;
                let db_state = self.db.database.read_db()?;

                if db_state.stories.get(&story_id).is_none() {
                    return Ok(None);
                }

                Ok(Some(Action::NavigateToStoryDetail { epic_id, story_id }))
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct StoryDetail {
    pub epic_id: u32,
    pub story_id: u32,
    pub db: Rc<JiraDatabase>,
}

impl Page for StoryDetail {
    fn draw_page(&self) -> Result<()> {
        let description_width = TERMINAL_WIDTH - ID_WIDTH - STATUS_WIDTH - NAME_WIDTH - 4;
        let db_state = self.db.read_db()?;
        let story = db_state
            .stories
            .get(&self.story_id)
            .ok_or_else(|| anyhow!("could not find story!"))?;

        println!(
            "{}{:-^width$}",
            HEADER_COLOR,
            " STORY ",
            width = TERMINAL_WIDTH
        );
        println!(
            "{: ^id_width$}|{: ^name_width$}|{: ^description_width$}| {: ^status_width$}{dft}",
            "id",
            "name",
            "description",
            "status",
            id_width = ID_WIDTH,
            name_width = NAME_WIDTH,
            description_width = description_width,
            status_width = STATUS_WIDTH,
            dft = DFT
        );

        // print out story details using get_column_string()
        println!(
            "{:<id_width$}{HEADER_COLOR}|{DFT}{}{HEADER_COLOR}|{DFT}{}{HEADER_COLOR}|{DFT} {}",
            &self.story_id,
            get_column_string(&story.name, NAME_WIDTH),
            get_column_string(&story.description, description_width),
            get_column_string(format!("{}", story.status).as_str(), STATUS_WIDTH),
            id_width = ID_WIDTH,
        );

        println!();
        println!();

        print_query("[p] previous | [u] update story | [d] delete story")
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        // match against the user input and return the corresponding action. If the user input was invalid return None.
        let epic_id = self.epic_id;
        let story_id = self.story_id;

        match input {
            "" => Ok(None),
            "p" => Ok(Some(Action::NavigateToPreviousPage)),
            "u" => Ok(Some(Action::UpdateStoryStatus { story_id })),
            "d" => Ok(Some(Action::DeleteStory { epic_id, story_id })),
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
            let db: Rc<JiraDatabase> = Rc::new(JiraDatabase {
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
                Some(Action::NavigateToStoryDetail {
                    epic_id: 1,
                    story_id: 2
                })
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
                epic_id,
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
                epic_id,
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
                epic_id,
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
                epic_id,
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
                Some(Action::DeleteStory { epic_id, story_id })
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
