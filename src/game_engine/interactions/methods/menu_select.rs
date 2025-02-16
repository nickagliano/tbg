/// menu_select.rs is an impl block for GameEngine which abstracts out
/// selecting items from a menu.
///
use crate::game_engine::game_engine::GameEngine;
use crate::terminal_utils;
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io;

impl GameEngine {
    pub fn menu_select<T: Clone + std::fmt::Display>(&self, message: &str, options: Vec<T>) -> T {
        let mut stdout = io::stdout();
        enable_raw_mode().expect("Failed to enable raw mode");

        let mut selected_index = 0;
        execute!(stdout, Hide).expect("Cursor failed to hide");

        terminal_utils::print_menu(message, &options, selected_index, true) // pass a slice
            .expect("Printing menu failed");

        let selected_item = loop {
            if let Ok(Event::Key(key_event)) = event::read() {
                match key_event.code {
                    KeyCode::Up => {
                        if selected_index > 0 {
                            selected_index -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if selected_index < options.len() - 1 {
                            selected_index += 1;
                        }
                    }
                    KeyCode::Enter => break options[selected_index].clone(),
                    _ => break options[0].clone(), // Default to first option on unexpected input
                }

                terminal_utils::print_menu(message, &options, selected_index, false)
                    .expect("Printing menu failed");
            }
        };

        disable_raw_mode().expect("Failed to disable raw mode");
        execute!(stdout, Show).expect("Cursor failed to show");
        terminal_utils::clear_console(None);

        selected_item
    }
}
