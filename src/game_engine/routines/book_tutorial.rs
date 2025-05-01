// use crate::game_engine::interactions;
use crate::models::player::player::Player;
// use crate::models:: // TODO: Fill in book model(s), grab Book, PlayerBook
use crate::terminal_utils;
// use crate::tui; // FIXME: Use TUI for rendering books during
use crossterm::terminal;

pub struct BookTutorialRoutine {
    player: Player,
}

impl BookTutorialRoutine {
    pub fn new(player: Player) -> Self {
        BookTutorialRoutine { player }
    }

    // Runs the book tutorial, in which the player will learn how to
    // use books, culminating in them choosing their first book.
    pub fn run(&mut self) {
        terminal::enable_raw_mode().unwrap();

        terminal_utils::simulate_typing(&format!("Okay, {}.", self.player.name));
        terminal_utils::prompt_enter_to_continue();

        terminal_utils::simulate_typing("You might want to sit down for this one.");
        terminal_utils::prompt_enter_to_continue();

        terminal_utils::simulate_typing("Let me ask...\n\nAnd I want you to really think about it—\n\nWhat do stories mean to you?");
        terminal_utils::prompt_enter_to_continue();

        terminal_utils::simulate_typing("Have you ever read a story that *changed* you?\n\nThat shaped your life, and, maybe, continues to shape it every day?");
        terminal_utils::prompt_enter_to_continue();

        terminal_utils::simulate_typing(
            "Stories you've read, or perhaps, those you've created for yourself?",
        );
        terminal_utils::prompt_enter_to_continue();

        terminal_utils::simulate_typing(
            "Well, since you're here, you probably know this already...",
        );
        terminal_utils::prompt_enter_to_continue();

        terminal_utils::simulate_typing(
            "But it's my job to make sure you understand that words are powerful.",
        );
        terminal_utils::prompt_enter_to_continue();

        terminal_utils::simulate_typing(
            "Words hold the power of life and death. Creation and destruction.",
        );
        terminal_utils::prompt_enter_to_continue();

        let options = vec!["Yes", "No"];
        let _ = terminal_utils::menu_select("Does that make sense?:", options);

        terminal_utils::simulate_typing("And now let me ask you this—");
        terminal_utils::prompt_enter_to_continue();

        terminal_utils::simulate_typing(
            "Have you ever held an old, well-loved book in your hands?",
        );
        terminal_utils::prompt_enter_to_continue();

        terminal_utils::simulate_typing(
            "The worn leather. The faded ink.\n\nThe smell — oh the wonderful smell of the pages.",
        );
        terminal_utils::prompt_enter_to_continue();

        terminal_utils::simulate_typing("Like dust and ink and memory.");
        terminal_utils::prompt_enter_to_continue();

        terminal_utils::simulate_typing(
            "Have you ever read a story...\n\nand felt the world fall away?",
        );
        terminal_utils::prompt_enter_to_continue();

        terminal_utils::simulate_typing("Most people think they're just that—\n\nstories.");
        terminal_utils::prompt_enter_to_continue();

        terminal_utils::simulate_typing(
            "Words on a page.\n\nLies we tell children before the world hardens them.",
        );
        terminal_utils::prompt_enter_to_continue();

        terminal_utils::simulate_typing("But stories are older than kings. Older than kingdoms.\n\nAnd some stories which are written just right...\n\nthey, well...");
        terminal_utils::prompt_enter_to_continue();

        terminal_utils::simulate_typing("Let's just say they can grow to be \"larger than life\".");
        terminal_utils::prompt_enter_to_continue();

        terminal_utils::simulate_typing("What does that really mean, though?");
        terminal_utils::prompt_enter_to_continue();

        terminal_utils::simulate_typing(
            "Well, some stories are not just real — they are *more* real than you and I.",
        );
        terminal_utils::prompt_enter_to_continue();

        terminal_utils::simulate_typing("More real than you can comprehend. At least, right now.");
        terminal_utils::prompt_enter_to_continue();

        terminal_utils::simulate_typing("Some pages carry this power.\n\nRead them, *understand* them.\n\nAnd you might feel a lion’s courage...\n\nor a thief’s quick fingers.");
        terminal_utils::prompt_enter_to_continue();

        terminal_utils::simulate_typing(
            "With the right tale, you might grow strong; or become as light as a feather.",
        );
        terminal_utils::prompt_enter_to_continue();

        terminal_utils::simulate_typing("But don’t get any ideas — not just anyone can use them, read them, understand them.\n\nThe power of books is... somewhat rare. And maybe even a bit selective.");
        terminal_utils::prompt_enter_to_continue();

        terminal_utils::simulate_typing(
            "It chooses. It waits.\n\nAnd when it finds someone it likes —\n\nit listens.",
        );
        terminal_utils::prompt_enter_to_continue();

        terminal_utils::simulate_typing("...");
        terminal_utils::prompt_enter_to_continue();

        terminal_utils::simulate_typing("They say everyone has a vocation — a calling.");
        terminal_utils::prompt_enter_to_continue();

        terminal_utils::simulate_typing(
            "And that every book, and every page, was written with someone in mind...",
        );
        terminal_utils::prompt_enter_to_continue();

        terminal_utils::simulate_typing("And now, here we are.\n\nWaiting. Heavy with silence.");
        terminal_utils::prompt_enter_to_continue();

        terminal_utils::simulate_typing("I don't know where you came from.");
        terminal_utils::prompt_enter_to_continue();

        terminal_utils::simulate_typing("This is a rare thing... A rare thing indeed.");
        terminal_utils::prompt_enter_to_continue();

        terminal_utils::simulate_typing("Rarer still is someone like you.");
        terminal_utils::prompt_enter_to_continue();

        terminal_utils::simulate_typing("So go on.\n\nTake a breath.");
        terminal_utils::prompt_enter_to_continue();

        terminal_utils::simulate_typing("This is your story now.");
        terminal_utils::prompt_enter_to_continue();

        // TODO: Show a book, closed.

        loop {
            break;
        }

        // Create the player's book after choosing
        // PlayerBook::new(player, book)... something

        terminal::disable_raw_mode().unwrap();
    }
}
