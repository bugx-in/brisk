// src/message.rs

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    pub name: String,
    pub display: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: u64,
    pub summary: String,
    pub body: String,
    pub icon: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Action>>
}

impl Message {
    /// Load message from json string.
    pub fn from_json(message_json: &String) -> Result<Message, serde_json::Error> {
        let message: Message = serde_json::from_str(message_json)?;
        Ok(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bad_message() {
        let message_json: String = String::from("{\"id\": 1, \"summary\": \"hello\"");
        assert!(Message::from_json(&message_json).is_err(), "Error is supposed to happen here.");
    }

    #[test]
    fn test_good_message() {
        let message_json: String = String::from("{\"id\": 1, \"summary\": \"hello\",
            \"body\": \"test\", \"icon\": \"testicon\", \"actions\": [{\"name\": \"hello\", \"display\": \"nonono\"}]}");
        assert!(Message::from_json(&message_json).is_ok(), "Message could not be correctly parsed.");
    }
}
