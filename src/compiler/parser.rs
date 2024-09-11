use super::{deserialization::Deck, error::DeserializationError};

pub fn deserialize_json(deck_json: String) -> Result<Deck, DeserializationError> {
    let result: Result<Deck, serde_json::Error> = serde_json::from_str(&deck_json);

    match result {
        Ok(deck) => return Ok(deck),
        Err(_) => Err(DeserializationError::FailedToDeserialize),
    }
}
