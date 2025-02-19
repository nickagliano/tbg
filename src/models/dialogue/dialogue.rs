use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

// Define the root path for dialogues
const DIALOGUE_ROOT: &str = "src/db/seeds/dialogue/";

#[derive(Debug, Serialize, Deserialize)]
pub struct Dialogue {
    id: u32,
    speaker: String,
    text: String,
    responses: Option<Vec<DialogueResponse>>,
    has_input: Option<bool>,
    input_type: Option<String>,
    next_id: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DialogueResponse {
    text: String,
    next_id: u32,
}

/// Function to load a YAML dialogue file into a <Vec<Dialogue>
pub fn load_dialogue<P: AsRef<Path>>(
    filename: P,
) -> Result<Vec<Dialogue>, Box<dyn std::error::Error>> {
    // Combine the root path with the filename
    let full_path = Path::new(DIALOGUE_ROOT)
        .join(filename)
        .with_extension("yaml");

    let yaml_str = fs::read_to_string(full_path)?;
    let dialogues: Vec<Dialogue> = serde_yaml::from_str(&yaml_str)?;
    Ok(dialogues)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_dialogue() {
        let filename = "test_dialogue";
        let result = load_dialogue(filename);

        assert!(result.is_ok(), "Failed to load dialogue from YAML.");
        let dialogues = result.unwrap();
        assert!(!dialogues.is_empty(), "Dialogue list is empty.");
    }

    #[test]
    fn test_dialogue_structure() {
        let filename = "test_dialogue";
        let dialogues = load_dialogue(filename).expect("Failed to load test dialogue");

        assert_eq!(dialogues[0].id, 1);
        assert_eq!(dialogues[0].speaker, "Test Narrator");
        assert_eq!(dialogues[0].text, "Hello, traveler!");
        assert!(dialogues[0]
            .responses
            .as_ref()
            .map_or(true, |r| r.is_empty()));
    }

    #[test]
    fn test_dialogue_with_input() {
        let filename = "test_dialogue";
        let dialogues = load_dialogue(filename).expect("Failed to load test dialogue");

        let input_dialogue = dialogues
            .iter()
            .find(|d| d.id == 2)
            .expect("Missing input dialogue");

        assert_eq!(input_dialogue.text, "What is your name?");
        assert_eq!(input_dialogue.has_input, Some(true));
        assert_eq!(input_dialogue.input_type, Some("string".to_string()));
        assert_eq!(input_dialogue.next_id, Some(3));
    }

    #[test]
    fn test_dialogue_with_placeholder() {
        let filename = "test_dialogue";
        let dialogues = load_dialogue(filename).expect("Failed to load test dialogue");

        let personalized_dialogue = dialogues
            .iter()
            .find(|d| d.id == 3)
            .expect("Missing personalized dialogue");

        assert!(
            personalized_dialogue.text.contains("{player_name}"),
            "Placeholder {{player_name}} not found in text"
        );
    }

    #[test]
    fn test_dialogue_with_responses() {
        let filename = "test_dialogue";
        let dialogues = load_dialogue(filename).expect("Failed to load test dialogue");

        // Find the dialogue with responses (with id 5)
        let response_dialogue = dialogues
            .iter()
            .find(|d| d.id == 5)
            .expect("Dialogue with ID 5 not found");

        assert!(
            response_dialogue.responses.is_some(),
            "Expected responses to be Some(Vec), but found None."
        );
        let responses = response_dialogue.responses.as_ref().unwrap();
        assert_eq!(
            responses.len(),
            2,
            "Expected 2 responses, but found {}",
            responses.len()
        );
        assert_eq!(
            responses[0].text, "Go left",
            "First response text doesn't match."
        );
        assert_eq!(
            responses[0].next_id, 5,
            "First response next_id doesn't match."
        );
        assert_eq!(
            responses[1].text, "Go right",
            "Second response text doesn't match."
        );
        assert_eq!(
            responses[1].next_id, 6,
            "Second response next_id doesn't match."
        );
    }
}
