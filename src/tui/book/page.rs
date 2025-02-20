use crate::models::book::page::Page as PageModel;
use crate::models::book::page::PageSide;
use std::io::{self, Write};

pub struct Page {
    pub source_page: PageModel,
    pub side: PageSide, // Front or back
}

/// A page has two sides -- a front and a back
/// Use side to denote what should be rendered
impl Page {
    pub fn new(source_page: PageModel, side: PageSide) -> Self {
        Page { source_page, side }
    }

    pub fn render(&self) -> io::Result<()> {
        let mut stdout = io::stdout();

        stdout.flush()
    }
}
