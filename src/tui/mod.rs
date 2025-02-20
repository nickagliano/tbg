//! The TUI (terminal user interface) module contains all of the UI components
//! that make up TBG.
//!
//! It should __just render stuff__, and should not handle any data, do any transformations,
//! contain any complex logic, computations, or do anything "smart".
//!
//! If a TUI component is getting too complicated, it might have logic that needs
//! to be extracted into a tbg::game_engine::interactions.
//!
//! TODO: All or almost all TUI components should have a trait. Should have render() function.
//!

pub mod book;
pub mod menu;
pub mod window;
