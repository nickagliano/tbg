use crossterm::{
    cursor, execute,
    style::{Color as TermionColor, SetForegroundColor},
    terminal::{self, ClearType},
};
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    style::ResetColor,
    terminal::Clear,
    ExecutableCommand,
};
use regex::Regex;
use std::{
    io::{self, Write},
    thread,
    time::Duration,
};
// For calculating visual width of multibyte unicode chars
// TODO: I'd like to get rid of this dependency if possible
//       - Could just hardcode a map of multibyte chars I'm using
use unicode_width::UnicodeWidthStr;

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
        draw_window(&colored_message).expect("Failed to draw window");

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
    draw_window(&formatted_message).unwrap();
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
    draw_window(&content)?;

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
            draw_window(&format!(
                "{}\n{}", // typed message + remaining content
                colored_message,
                content.split_once('\n').unwrap().1
            ))?;

            thread::sleep(Duration::from_millis(25));
        }
    } else {
        draw_window(&content)?;
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
    draw_window(&final_content)?;

    Ok(())
}

/// This is one of the core functions of TBG!
/// Use-cases:
///  - During dialogue/narration mode (TODO: Come up with these modes, naming conventions)
///  - *Not* during world/navigation mode (that uses Viewport::render())
///  - The Viewport stuct's render fn and this draw_window fn are very similar
///    - Might need to share some abstractions in the future
///
/// Responsibilities:
///  - Take a string slice (usually a snippet of dialogue or narration)
///  - Draw a frame around the edges of the user's terminal
///  - Center the message in the frame
///
/// Other notes:
///  - The real complexity of this method is in calculating the padding
///     - The string slice is split into lines by the new-line char (\n)
///     - The lines may contain ANSI colors, or multi-byte unicode chars--
///       the padding calculations need to take into account these
///       invisible or non-standard chars
///  - Users can choose the border styling in the settings
///  - The draw_window method can be used in tandem with the simulate_typing method
///
// FIXME: add a FrameType setting, use instead of hard-coding "NORMAL" borders
//        - DO NOT want to query this from the GameState every time.
//        - Want to load this at start of game, and only fetch if settings are updated.
// FIXME: Can we validate that the content parameter actually fits inside of the terminal?
//        - If it doesn't fit, how do we handle?
pub fn draw_window(content: &str) -> io::Result<()> {
    let mut stdout = io::stdout();

    // Get the terminal size
    let (width, height) = terminal::size()?;
    let width = width.max(10);
    let height = height.max(5);

    // Create borders
    // let top_border = format!("┏{}┓", "━".repeat((width - 2) as usize));
    // let bottom_border = format!("┗{}┛", "━".repeat((width - 2) as usize));
    // let empty_line = format!("┃{}┃", " ".repeat((width - 2) as usize));
    // TODO: Maybe use these "fantasy" style borders
    let repeat_count = (width - 2) / 3; // Required because the fantasy border is 3 chars long
    let remainder = (width - 2) % 3; // Required because the fantasy border is 3 chars long
    let top_border = format!(
        "╭{}{}╮",
        "╼◈╾".repeat(repeat_count as usize),
        "━".repeat(remainder as usize)
    );
    let bottom_border = format!(
        "╰{}{}╯",
        "╼◈╾".repeat(repeat_count as usize),
        "━".repeat(remainder as usize)
    );
    let empty_line = format!("║{}║", " ".repeat((width - 2) as usize));

    // Regex to remove ANSI escape codes (including color codes and resets)
    let color_code_re = Regex::new(r"\x1b\[[0-9;]*m").unwrap();

    // Move the cursor to the top-left corner and clear the screen
    execute!(stdout, cursor::MoveTo(0, 0), Clear(ClearType::All))?;
    writeln!(stdout, "{}\r", top_border)?;

    // Split content into lines and calculate padding
    let content_lines: Vec<&str> = content.split('\n').collect();
    let content_height = content_lines.len();
    let padding_top = (height as usize - content_height - 2).max(0) / 2;
    let padding_bottom = (height as usize - content_height - padding_top - 2).max(0);

    // Pad top empty lines
    for _ in 0..padding_top {
        writeln!(stdout, "{}\r", empty_line)?;
    }

    // Pad and print each line of content
    for line in content_lines {
        // Remove color codes to calculate the padding based on the actual length
        let clean_line = color_code_re.replace_all(line, "");
        // Correct visual width, accounting for multibyte unicode characters!
        let line_len = UnicodeWidthStr::width(clean_line.as_ref());
        let extra_padding = width as usize - 2 - line_len;

        // Split padding between left and right equally
        let padding_left = extra_padding / 2;
        let padding_right = extra_padding - padding_left;

        // Pad the line and print it, with color codes intact
        let padded_line = format!(
            "┃{}{}{}┃",
            " ".repeat(padding_left),
            line,
            " ".repeat(padding_right)
        );

        writeln!(stdout, "{}\r", padded_line)?;
    }

    // Pad bottom empty lines
    for _ in 0..(padding_bottom.saturating_sub(1)) {
        writeln!(stdout, "{}\r", empty_line)?;
    }

    // Print the bottom border
    write!(stdout, "{}\r", bottom_border)?;
    stdout.flush()
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
