use std::env;

pub struct GameArgs {
    pub new_game: bool,
    pub dev: bool,
}

// Original function that uses env::args()
pub fn parse_args() -> GameArgs {
    let args: Vec<String> = env::args().collect();
    GameArgs {
        new_game: args.contains(&"--new-game".to_string()),
        dev: args.contains(&"--dev".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test utility function that takes a Vec<String> for testing
    pub fn parse_args_test(args: Vec<String>) -> GameArgs {
        GameArgs {
            new_game: args.contains(&"--new-game".to_string()),
            dev: args.contains(&"--dev".to_string()),
        }
    }

    #[test]
    fn test_parse_args_no_flags() {
        // Test with no flags provided
        let args = vec![];
        let parsed_args = parse_args_test(args);

        assert!(!parsed_args.new_game);
        assert!(!parsed_args.dev);
    }

    #[test]
    fn test_parse_args_new_game_only() {
        // Test with the `--new-game` flag
        let args = vec!["--new-game".to_string()];
        let parsed_args = parse_args_test(args);

        assert!(parsed_args.new_game);
        assert!(!parsed_args.dev);
    }

    #[test]
    fn test_parse_args_dev_only() {
        // Test with the `--dev` flag
        let args = vec!["--dev".to_string()];
        let parsed_args = parse_args_test(args);

        assert!(!parsed_args.new_game);
        assert!(parsed_args.dev);
    }

    #[test]
    fn test_parse_args_both_flags() {
        // Test with both `--new-game` and `--dev` flags
        let args = vec!["--new-game".to_string(), "--dev".to_string()];
        let parsed_args = parse_args_test(args);

        assert!(parsed_args.new_game);
        assert!(parsed_args.dev);
    }
}
