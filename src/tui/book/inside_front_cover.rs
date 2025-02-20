use std::io::{self, Write};

pub struct InsideFrontCover<'a> {
    pub content: &'a str,
}

impl<'a> InsideFrontCover<'a> {
    pub fn new(content: &'a str) -> Self {
        InsideFrontCover { content }
    }

    pub fn render(&self) -> io::Result<()> {
        let mut stdout = io::stdout();

        stdout.flush()
    }
}
