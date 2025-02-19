use crate::terminal_utils::prompt_enter_to_continue;
use crossterm::terminal;

pub struct DialogueRoutine;

// FIXME: Not sure how useful this routine is in particular
//        - Sometimes dialogue is very complex, has menus, etc. Character Creation...?
//        - How to structure different dialogue and have it be callable by something like
//          DialogueRoutine, while allowing for the flexibility of returning values, etc.?
//
impl DialogueRoutine {
    pub fn new(&self) -> Self {
        DialogueRoutine
    }

    // Runs the dialogue routine
    pub fn run(&mut self) {
        terminal::enable_raw_mode().unwrap();

        loop {
            println!("Do something");
            prompt_enter_to_continue();
            break;
        }

        terminal::disable_raw_mode().unwrap();
    }
}
