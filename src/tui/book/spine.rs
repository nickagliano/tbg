use crate::models::book::book::Book;
use std::io::{self, Write};

pub struct Spine {
    pub book: Book,
}

/// The skinny, spiny, binding, backbone of a book
/// Useful for rendering a collection of books, maybe
/// in a library, or when choosing a book during tutorial
impl Spine {
    pub fn new(book: Book) -> Self {
        Spine { book }
    }

    pub fn render(&self) -> io::Result<()> {
        let mut stdout = io::stdout();

        stdout.flush()
    }
}
