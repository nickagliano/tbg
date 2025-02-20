use crate::models::book::page::Page;
use std::io::{self, Write};

pub struct Spread {
    pub left: Page,
    pub right: Page,
}

/// A spread is an open book, with a left page and a right page
/// - If on page 1, the "left page" will be the inside-front cover
/// - If on last page, the "right page" will be the inside-back cover
/// FIXME: Model PageOrInsideCover?
impl Spread {
    pub fn new(left: Page, right: Page) -> Self {
        Spread { left, right }
    }

    pub fn render(&self) -> io::Result<()> {
        let mut stdout = io::stdout();

        stdout.flush()
    }
}
