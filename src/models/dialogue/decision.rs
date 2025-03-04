// use rusqlite::types::ToSql;
use serde::{Deserialize, Serialize};
use std::time::Duration;

// Represents a player's decision in the dialogue system
#[derive(Debug, Serialize, Deserialize)]
pub struct Decision {
    id: u32,
    player_id: u32,           // Tracks the player making the decision
    dialogue_id: u32,         // FK to the dialogue
    response_id: Option<u32>, // FK to the chosen response (nullable for input-based)
    deliberation_time: Duration,
    // FIXME: Add created at, updated at
}

// FIXME: This should create a new Decision
// - Need to figure out how to record
pub fn capture_decision(dialogue_id: u32, response_id: Option<u32>, args: Vec<String>) {
    println!("{}", dialogue_id);
    println!("{:?}", response_id);
    println!("{:?}", args);
}

// Test function to verify decision logging
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::dialogue::character_type::CharacterType;
    use crate::models::dialogue::dialogue::Dialogue;
    use crate::models::dialogue::dialogue_response::DialogueResponse;
    use std::time::Duration;

    fn create_mock_decision(
        player_id: u32,
        dialogue_id: u32,
        response_id: Option<u32>,
        deliberation_time: Duration,
    ) -> Decision {
        Decision {
            id: 1, // Mock ID for the decision
            player_id,
            dialogue_id,
            response_id,
            deliberation_time,
        }
    }

    #[test]
    fn test_decision_logging() {
        // Create a mock dialogue using the new constructor
        let dialogue = Dialogue::new(
            1,                                     // ID
            1,                                     // character_id (mock)
            CharacterType::NonPlayerCharacter,     // character_type
            Some(false),                           // Root node flag
            None,                                  // Root node name
            "What do you want to do?".to_string(), // text
            Some(vec![
                DialogueResponse {
                    id: 1,
                    dialogue_id: 1,
                    text: "Fight".to_string(),
                    next_id: 2,
                },
                DialogueResponse {
                    id: 2,
                    dialogue_id: 1,
                    text: "Talk".to_string(),
                    next_id: 3,
                },
            ]), // responses
            Some(false),                           // no input required
            None,                                  // no input type
            Some(2),                               // next_id
        );

        // Create a mock decision
        let decision = create_mock_decision(1, dialogue.id, Some(1), Duration::new(2, 0));

        // Verify decision logging
        println!("{:?}", decision);

        // Test assertions
        assert_eq!(decision.player_id, 1);
        assert_eq!(decision.dialogue_id, dialogue.id);
        assert_eq!(decision.response_id, Some(1));
        assert_eq!(decision.deliberation_time, Duration::new(2, 0));

        // Check if the dialogue is an "end node"
        assert_eq!(dialogue.is_end_node(), false); // Since it has a next_id, it's not an end node
    }
}
