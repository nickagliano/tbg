use crate::tui;
use crossterm::{
    cursor::{self, Hide, Show},
    event::{self, Event, KeyCode},
    execute,
    style::{Color as TermionColor, ResetColor, SetForegroundColor},
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

// TODO: Set FrameType in settings
// pub enum FrameType {
//     Normal,
//     Fantasy,
// }

/// TBGColors, not to be confused with termion::Color (aliased to TermionColor),
/// are a subset of ANSI colors that are used to style the TUI
// TODO: store these in settings!
struct TBGColors;
impl TBGColors {
    const ACTION_COLOR: TermionColor = TermionColor::DarkCyan;
    const TEXT_COLOR: TermionColor = TermionColor::DarkYellow;

    // A single method to get the ANSI escape code for any color
    fn fg_string(color: TermionColor) -> String {
        match color {
            TermionColor::DarkCyan => "\x1b[36m".to_string(), // ANSI code for DarkCyan
            TermionColor::DarkYellow => "\x1b[33m".to_string(), // ANSI code for DarkYellow
            _ => "\x1b[39m".to_string(),                      // Default color if no match
        }
    }

    // Method to reset the color
    fn fg_str_reset() -> String {
        "\x1b[39m".to_string() // ANSI reset code
    }
}

/// The standard method to get free-form user input, e.g., when
/// they input their name during character creation
pub fn get_input() -> String {
    let mut stdout = io::stdout();
    let mut user_input = String::new();

    // Disable raw mode while we get user input
    terminal::disable_raw_mode().unwrap();

    // Print input prompt using action required color
    stdout
        .execute(SetForegroundColor(TBGColors::ACTION_COLOR))
        .unwrap();
    write!(stdout, "\n> ").unwrap();
    stdout.execute(ResetColor).unwrap();
    stdout.flush().unwrap();

    // Read input
    io::stdin().read_line(&mut user_input).unwrap();

    // Reset terminal colors and clear screen
    stdout.execute(ResetColor).unwrap();
    clear_console(None);
    stdout.flush().unwrap();

    // Renable raw mode after input
    terminal::enable_raw_mode().unwrap();

    // Return trimmed user input
    user_input.trim().to_string()
}

/// Helper method which mostly just wraps termion's clear terminal function
pub fn clear_console(stdout: Option<&mut dyn Write>) {
    // This conditional lets use use clear_console in testing scenarios
    // In real-game usage, clear_console will always be called with None
    let stdout = match stdout {
        Some(s) => s,              // If a custom writer is passed, use it directly
        None => &mut io::stdout(), // If no custom writer is passed, use io::stdout()
    };

    // Clear the console and reset the cursor to the top left corner
    stdout.execute(terminal::Clear(ClearType::All)).unwrap();
    stdout.execute(cursor::MoveTo(0, 0)).unwrap();
}

/// Utility method, helpful for moving the user through dialogue/narration
/// - Prints a prompt using the "action-required" styling
/// - Polls the UI, blocking execution until the player presses enter
/// - Clears the terminal
// NOTE: Should this be responsible for clearing the terminal?
//       - Seems like a leaky abstraction
//       - Might want to let the rendering loop clear the terminal
//       - It's not really hurting anything right now, but this
//       - Might be double-clearing the terminal
pub fn prompt_enter_to_continue() {
    let mut stdout = io::stdout();
    let prompt = "\rPress enter to continue... ";

    write!(stdout, "{}", action_required(&format!("\n{}", prompt)))
        .expect("Failed to print prompt to continue");
    stdout.flush().unwrap();

    // Block and wait for Enter key press
    loop {
        if let Ok(true) = event::poll(Duration::from_millis(100)) {
            if let Ok(Event::Key(key_event)) = event::read() {
                if key_event.code == KeyCode::Enter {
                    break; // Exit loop when Enter is pressed
                }
            }
        }
    }

    clear_console(None);
}

/// Simple utility method to print dialogue/narration using the text styling
pub fn p(message: &str) {
    let mut stdout = io::stdout();
    stdout
        .execute(SetForegroundColor(TBGColors::TEXT_COLOR))
        .unwrap();
    write!(stdout, "{}", message).unwrap();
    stdout.execute(ResetColor).unwrap();
}

/// Simple utility method to print a message using the "action-required" styling
pub fn action_required(message: &str) -> String {
    // Write the colored message directly without adding a newline
    let formatted_message = format!(
        "{}{}{}",
        TBGColors::fg_string(TBGColors::ACTION_COLOR), // Apply color
        message,
        TBGColors::fg_str_reset() // Reset the color
    );

    return formatted_message;
}

/// One of the most commonly used methods in the app!
///
/// Prints a message (usually dialogue or narration) to the user,
/// one char at a time. Helps to add some style/life to the game,
/// and encourages them to actually read the storyline.
pub fn simulate_typing(message: &str) {
    let mut stdout = io::stdout();

    // TODO: Move typing speed to settings!
    let typing_speed = 25;
    let mut displayed_message = String::new();

    // Hide the cursor before typing starts
    execute!(stdout, Hide).expect("Failed to hide cursor");

    for c in message.chars() {
        displayed_message.push(c);

        // Apply TEXT_COLOR to the message as it is typed
        let colored_message = format!(
            "{}{}{}",
            TBGColors::fg_string(TBGColors::TEXT_COLOR), // Apply the color
            displayed_message,
            TBGColors::fg_str_reset() // Reset the color after message
        );

        // Draw the window with the colored message
        tui::window::Window::new(&colored_message)
            .render()
            .expect("Render window failed");

        thread::sleep(Duration::from_millis(typing_speed));
    }

    // Show the cursor again after typing is done
    execute!(stdout, Show).expect("Failed to show cursor");
}

/// The main title screen! Just for fun.
/// This will definitely be tweaked, but right now it's using a fun
/// gradient. Not really in the perfect style of TBG, but I wanted
/// to experiment with what I could do with color.
pub fn title_screen() {
    clear_console(None);

    let message = r"
    ___                                               __         __                         __ ,    ___
    -   ---___- _-_-        ,- _~,       _-_ _,,     ,-||-,     ,-||-,   _-_-,             ,-| ~    -   -_,   /\\,/\\,   ,- _~,
      (' ||      /,       (' /| /          -/  )   ('|||  )   ('|||  )    // ,           ('||/__, (  ~/||   /| || ||   (' /| /
     ((  ||      || __   ((  ||/=         ~||_<   (( |||--)) (( |||--))   ||/\\         (( |||  | (  / ||   || || ||  ((  ||/=
     ((   ||     ~||-  -  ((  ||            || \\  (( |||--)) (( |||--))  ~|| <          (( |||==|  \/==||   ||=|= ||  ((  ||
     (( //       ||===||  ( / |            ,/--||  ( / |  )   ( / |  )    ||/\\          ( / |  ,  /_ _||  ~|| || ||   ( / |
       -____-   ( \_, |    -____-         _--_-'    -____-     -____-    _-__,\\,         -____/  (  - \\,  |, \\,\\,   -____-
                                            (                                                                 _-
    ";

    // Get the formatted message with gradient applied
    let formatted_message = draw_title_with_gradient(message);

    // Now pass that formatted message to draw_window
    tui::window::Window::new(&formatted_message)
        .render()
        .expect("Render window failed");
}

/// Only called by the title_screen fn
pub fn draw_title_with_gradient(message: &str) -> String {
    let lines: Vec<&str> = message.split('\n').collect();
    let mut output = String::new();

    for (i, line) in lines.iter().enumerate() {
        let gradient_color = get_gradient_color(i, lines.len());
        let color_code = format!(
            "\x1b[38;2;{};{};{}m",
            gradient_color.0, gradient_color.1, gradient_color.2
        );
        output.push_str(&format!("{}{}\x1b[0m\n", color_code, line));
    }

    output
}

/// Returns an RGB tuple
/// Currently only used in the title screen rendering
fn get_gradient_color(index: usize, total_lines: usize) -> (u8, u8, u8) {
    let red = (index as f32 / total_lines as f32 * 255.0).round() as u8;
    let green = ((total_lines as f32 - index as f32) / total_lines as f32 * 255.0).round() as u8;

    (red, green, 128) // Return an RGB tuple
}

/// Syntactic sugar method to make termion's reset_cursor message more readable
pub fn reset_cursor(stdout: &mut dyn Write) {
    write!(stdout, "{}", cursor::MoveTo(0, 0)).unwrap();
}

/// Parameters:
/// - message (usually a prompt, providing details on the choice the user is going to make)
/// - options (which will be printed out below the message)
/// - selected_index (the caller is responsible for setting this)
/// - use_simulate_typing (usually true during first render, then false in subsequent renders)
///
/// Output:
/// - Should be a &str, which can then be rendered in some rendering loop
///
/// Usage:
/// - The caller (the Menu struct), will loop, and print the menu until an option is selected
/// - The print_menu fn is "dumb", in that it just displays the content--the caller
///   is responsible for handling input and updating the display with the selected_index, etc.
///
// TODO: When the menu struct is implemented, move this to its impl block
// FIXME: This method should not be calling draw_window!
//        - This method should return a string slice, and that should
//          be passed to the rendering loop, which can call draw_window!
pub fn print_menu<T: std::fmt::Display>(
    message: &str,
    options: &Vec<T>,
    selected_index: usize,
    use_simulate_typing: bool,
) -> io::Result<()> {
    let mut content = String::new();

    // Pre-fill the content with spaces to prevent jumping
    content.push_str(&format!("{}\n", message));
    for _ in options.iter() {
        content.push_str("\n");
    }

    // Draw initial empty window before typing starts
    // FIXME: Is this really the right place to call draw_window?
    tui::window::Window::new(&content).render()?;

    if use_simulate_typing {
        let mut typed_message = String::new();
        for c in message.chars() {
            typed_message.push(c);

            // Apply the text color using fg_string
            let colored_message = format!(
                "{}{}{}", // Text color + typed message + reset color
                TBGColors::fg_string(TBGColors::TEXT_COLOR),
                typed_message,
                TBGColors::fg_str_reset()
            );

            // Ensure the rest of the content stays intact, just adding the typed message
            // FIXME: Is this really the right place to call draw_window?
            tui::window::Window::new(&format!(
                "{}\n{}", // typed message + remaining content
                colored_message,
                content.split_once('\n').unwrap().1
            ))
            .render()?;

            thread::sleep(Duration::from_millis(25));
        }
    } else {
        tui::window::Window::new(&content).render()?;
    }

    // Replace placeholder spaces with actual menu options
    let mut final_content = String::new();

    // Apply color to the message and reset it
    let colored_message = format!(
        "{}{}{}",
        TBGColors::fg_string(TBGColors::TEXT_COLOR),
        message,
        TBGColors::fg_str_reset()
    );
    final_content.push_str(&format!("{}\n", colored_message));

    // Build the final content with colored message and options
    for (i, option) in options.iter().enumerate() {
        let colored_option = if i == selected_index {
            format!(
                "{}> {}{}",
                TBGColors::fg_string(TBGColors::ACTION_COLOR),
                option,
                TBGColors::fg_str_reset()
            )
        } else {
            format!(
                "  {}{}{}", // Apply ACTION_COLOR + option + reset color
                TBGColors::fg_string(TBGColors::ACTION_COLOR),
                option,
                TBGColors::fg_str_reset()
            )
        };

        final_content.push_str(&format!("{}\n", colored_option));
    }

    // Draw final stable window with full content
    tui::window::Window::new(&final_content).render()?;

    Ok(())
}

// FIXME: These should be in terminal_util_tests probably.
#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_create_player() {}

    // TODO: Add tests! For everything!
    // - TBGColors
    //   - fg_string methods
    // - get_input
    // - clear_console
    // - prompt_enter_to_continue
    // - p
    // - action_required
    // - print_menu
    // - draw_window
    // - reset_cursor
    // - title_screen
    // - draw_title_with_gradient
    // - get_gradient_color
}
