use crate::terminal_utils::{prompt_enter_to_continue, title_screen};
use crossterm::terminal;

pub struct TitleMenuRoutine;

impl TitleMenuRoutine {
    pub fn new(&self) -> Self {
        TitleMenuRoutine
    }

    pub fn run(&mut self) {
        terminal::enable_raw_mode().unwrap();
        title_screen();
        prompt_enter_to_continue();
        terminal::disable_raw_mode().unwrap();
    }
}
