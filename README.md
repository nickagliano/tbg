# The Book Game (tbg)

## Overview
**The Book Game (tbg)** is an interactive, text-based, adventure game written in Rust. It features persistent player data stored in an SQLite database, creating an immersive and evolving gameplay experience.

At the core of The Book Game is a turned-based strategy game, where players must skillfully navigate through pages of a book in order to defeat their opponent.

## Features
- Interactive text-based gameplay
- Persistent player profiles stored in an SQLite database
- Console interface with clear prompts and user input handling
- Modular design with a focus on reusability and maintainability

## Installation
### Prerequisites
- Rust (latest stable version recommended)
- ttyd (to play in browser)

### Installing ttyd (for playing in the browser)
```shell
brew install ttyd  # macOS
sudo apt install ttyd  # Debian/Ubuntu
```

### Clone the Repository
```sh
git clone https://github.com/nickagliano/tbg.git
cd tbg
```

### Build the Project
```sh
cargo build --release
```

### Run the Game

#### In terminal
```sh
cargo run  # start game, continuing from last save
cargo run -- --new-game # useful for dev if you want to start afresh
```

#### In browser
```sh
./bin/web.sh  # start game, continuing from last save
./bin/web_new_game.sh  # useful for dev i f you want to start afresh
```

## Usage
Upon launching the game, you will be prompted to either continue an existing adventure or create a new character.

- If you are a returning player, your saved data will be loaded.
- If you are a new player, you will be prompted to enter your name before beginning your journey.

## Project Structure
```
tbg/
├── Cargo.toml                # Rust package configuration
├── README.md                 # Project documentation
├── TODO.md                   # Nick's TODO list (with ordered priorities)
├── bin/                      # Wrappers around helpful commands
├── docs/                     # Documentation for developers (probably going to move this to a GitHub project wiki)
├── saves/                    # For storing save files (sqlite databases)
├── src
│   ├── args.rs                # For parsing command-line input (mostly for development purposes)
│   ├── db                     # Database connection and management
│   │   ├── [REDACTED]         # various db module functions
│   │   └── seeds/             # Seed data, fixtures, etc.
│   ├── dev/                   # Directory of specific development scenarios (useful for workshopping without requiring playing through the whole game)
│   ├── game_engine
│   │   ├── game_engine.rs     # Core engine/driver of game
│   │   ├── game_event.rs
│   │   ├── interactions/      # Resuable, composable rendering loops, usually to get some input from the user
│   │   ├── mod.rs
│   │   └── routines/          # Reusable, composable, often madeof many "interactions" (e.g., Battle, Dialogue, etc.)
│   ├── lib.rs                # lib
│   ├── main.rs               # Entry point of the game
│   ├── models/               # All of the models (almost always with SQL representations) and their functions
│   ├── music/                # The music module, for music and sound effects
│   ├── terminal_utils.rs     # For all things manipulating the terminal (being deprecated, everything moving to tui/)
│   ├── test_utils.rs         # Test utilities
│   ├── tui/                  # Terminal User Interface module, for all things rendering
│   └── world/                # For all things manupulating the world, world map (during the Navigation routine)
└── tests                     # Tests
```

## Highlighted Dependencies
This project relies heavily on the following Rust crates:
- `crossterm` – for handling terminal interactions
- `rusqlite` – for SQLite database support
- For a full list of dependencies and why they're in this project, see the Cargo.toml and the comments there.

## Contributing
Contributions are welcome! Feel free to fork the repository and submit pull requests.

CONTRIBUTING.md coming soon...

## License
This project is licensed under the MIT License. See `LICENSE` for details.

## Author
Developed by **Nick Agliano** – [GitHub Profile](https://github.com/nickagliano)
