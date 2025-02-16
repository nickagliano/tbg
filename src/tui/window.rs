// TODO: Use this
struct Window<'a, T: std::fmt::Display> {
    content: &'a str,
}

impl Window {
    pub fn draw_window(&self) -> io::Result<()> {
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
        let content_lines: Vec<&str> = self.content.split('\n').collect();
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
}
