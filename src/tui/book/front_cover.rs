use std::io::{self, Write};

pub struct FrontCover<'a> {
    pub content: &'a str,
}

impl<'a> FrontCover<'a> {
    pub fn new(content: &'a str) -> Self {
        FrontCover { content }
    }

    pub fn render(&self) -> io::Result<()> {
        let mut stdout = io::stdout();

        stdout.flush()
    }
}
