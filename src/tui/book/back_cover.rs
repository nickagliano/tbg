use std::io::{self, Write};

pub struct BackCover<'a> {
    pub content: &'a str,
}

impl<'a> BackCover<'a> {
    pub fn new(content: &'a str) -> Self {
        BackCover { content }
    }

    pub fn render(&self) -> io::Result<()> {
        let mut stdout = io::stdout();

        stdout.flush()
    }
}
