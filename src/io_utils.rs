use std::io;

pub const DFT: &str = "\x1B[0m";
pub const GREEN: &str = "\x1B[38;2;20;198;13m";
pub const RED: &str = "\x1B[38;2;255;0;0m";

//pub const BLUE: &str = "\x1B[38;2;58;150;221m";
//pub const TEAL: &str = "\x1B[38;2;0;214;214m";

pub fn get_user_input() -> String {
    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).unwrap();

    user_input
}

pub fn wait_for_key_press() {
    io::stdin().read_line(&mut String::new()).unwrap();
}
