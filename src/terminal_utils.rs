use std::io::{self, Stdout, Write};
use std::{thread, time::Duration};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor};

struct Colors;
impl Colors {
    const ACTION_COLOR: color::Fg<color::Blue> = color::Fg(color::Blue);
    const TEXT_COLOR: color::Fg<color::Yellow> = color::Fg(color::Yellow);
    const RESET: color::Fg<color::Reset> = color::Fg(color::Reset);
}

pub fn get_input() -> String {
    let mut stdout = io::stdout();
    let mut user_input = String::new();

    // Show the input prompt
    write!(
        stdout,
        "\n{}>{} {}",
        Colors::ACTION_COLOR,
        Colors::ACTION_COLOR,
        Colors::RESET
    )
    .unwrap();
    stdout.flush().unwrap();

    std::io::stdin().read_line(&mut user_input).unwrap();

    write!(stdout, "{}", color::Fg(color::Reset)).unwrap();

    clear_console(None);
    return user_input.trim().to_string();
}

// NOTE: The stdout parameter is only used for tests, in order simulate the terminal
pub fn clear_console(stdout: Option<&mut dyn Write>) {
    let mut stdout: Box<dyn Write> = match stdout {
        Some(s) => Box::new(s), // If a custom writer is passed, wrap it in a Box
        None => Box::new(io::stdout()), // If no custom writer is passed, use io::stdout
    };

    // Write the clear sequence
    write!(stdout, "{}", clear::All).unwrap();
    stdout.flush().unwrap();

    // Reset cursor to the top and hide it
    write!(stdout, "{}{}", cursor::Goto(1, 1), cursor::Hide).unwrap();
}

pub fn prompt_enter_to_continue() {
    let prompt = "Press any key to continue... ";
    action_required(&format!("\n{}", prompt));

    // Ensure prompt is printed before waiting for input
    io::stdout().flush().unwrap();

    // Try to set terminal to raw mode
    let stdin = io::stdin();
    match io::stdout().into_raw_mode() {
        Ok(stdout) => stdout,
        Err(e) => {
            eprintln!("Error entering raw mode: {}", e);
            return;
        }
    };

    stdin.keys().next().unwrap().unwrap();

    clear_console(None);
}

pub fn p(message: &str) {
    let mut stdout = io::stdout();

    write!(stdout, "{}{}{}", Colors::TEXT_COLOR, message, Colors::RESET).unwrap();
}

pub fn action_required(message: &str) {
    let mut stdout = io::stdout();

    write!(
        stdout,
        "{}{}{}",
        Colors::ACTION_COLOR,
        message,
        Colors::RESET
    )
    .unwrap();
}

pub fn simulate_typing(message: &str) {
    // TODO: Let user select their typing speed
    let typing_speed = 25; // Adjust typing speed here

    let mut stdout = io::stdout();

    // Set the color before starting to type
    write!(stdout, "{}", color::Fg(color::Yellow)).unwrap();

    // Flush to apply the color change to the whole message
    stdout.flush().unwrap();

    // Simulate typing one character at a time
    for c in message.chars() {
        write!(stdout, "{}", c).unwrap();
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(typing_speed));
    }

    // Reset color after typing is complete
    write!(stdout, "{}", Colors::RESET).unwrap();

    println!(); // Move to the next line after typing is complete
}

pub fn title_screen() {
    let mut stdout = io::stdout();

    clear_console(None);

    let message = r"
        ___                                               __         __                         __ ,    ___
       -   ---___- _-_-        ,- _~,       _-_ _,,     ,-||-,     ,-||-,   _-_-,             ,-| ~    -   -_,   /\\,/\\,   ,- _~,
          (' ||      /,       (' /| /          -/  )   ('|||  )   ('|||  )    // ,           ('||/__, (  ~/||   /| || ||   (' /| /
         ((  ||      || __   ((  ||/=         ~||_<   (( |||--)) (( |||--))   ||/\\         (( |||  | (  / ||   || || ||  ((  ||/=
        ((   ||     ~||-  -  ((  ||            || \\  (( |||--)) (( |||--))  ~|| <          (( |||==|  \/==||   ||=|= ||  ((  ||
         (( //       ||===||  ( / |            ,/--||  ( / |  )   ( / |  )    ||/\\          ( / |  ,  /_ _||  ~|| || ||   ( / |
           -____-   ( \_, |    -____-         _--_-'    -____-     -____-    _-__,\\,         -____/  (  - \\,  |, \\,\\,   -____-
                          `                  (                                                                 _-
        ";

    write!(stdout, "{}", message).unwrap();
}

pub fn reset_cursor(mut stdout: Stdout) -> Stdout {
    write!(stdout, "{}\n", cursor::Goto(1, 1)).unwrap();
    return stdout;
}
