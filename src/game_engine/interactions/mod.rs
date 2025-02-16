/// Interactions are sort of a weird term, but these are basically
/// TUI rendering loops, with the intent of getting some input
/// from the user.
///
/// Interactions wrap terminal-listening logic (via termion), with TUI
/// component rendering, and package them into reusable components.
///
/// E.g., the game engine might use an interaction to get a user's move selection
/// during a battle, or to select their gender during character creation.
///
/// Interactions help to keep the TUI components "dumb", and the game engine clean.
///
/// This isn't a perfect analogy, but it might help to think of the test pyramid--
/// - TUI components are the unit tests
/// - game_engine::interactions are the integration tests
///
pub mod battle;
pub mod book_builder;
pub mod character_creation;
pub mod methods;
