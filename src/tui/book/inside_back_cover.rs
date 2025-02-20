use std::io::{self, Write};

pub struct InsideBackCover<'a> {
    pub content: &'a str,
}

impl<'a> InsideBackCover<'a> {
    pub fn new(content: &'a str) -> Self {
        InsideBackCover { content }
    }

    pub fn render(&self) -> io::Result<()> {
        let mut stdout = io::stdout();

        stdout.flush()
    }
}
