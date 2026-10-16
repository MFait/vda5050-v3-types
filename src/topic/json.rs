use super::{Message, Topic, TopicKind, TopicMessage};

impl TopicKind {
    /// Decode exactly the body type selected by this kind from JSON text.
    ///
    /// Uses the concrete body's wire checks and returns the original JSON error.
    /// Does not try other body types, unwrap an envelope or validate header
    /// identity/version against a topic. Unknown fields retain the concrete
    /// body's behavior, so success does not prove the payload's intended kind.
    pub fn decode_json(self, payload: &str) -> Result<Message, serde_json::Error> {
        match self {
            Self::Order => crate::Order::from_json(payload).map(Message::from),
            Self::InstantActions => crate::InstantActions::from_json(payload).map(Message::from),
            Self::State => crate::State::from_json(payload).map(Message::from),
            Self::Visualization => crate::Visualization::from_json(payload).map(Message::from),
            Self::Connection => crate::Connection::from_json(payload).map(Message::from),
            Self::Factsheet => crate::Factsheet::from_json(payload).map(Message::from),
            Self::ZoneSet => crate::ZoneSet::from_json(payload).map(Message::from),
            Self::Responses => crate::Responses::from_json(payload).map(Message::from),
        }
    }

    /// Decode the selected body from UTF-8 JSON bytes, without header validation.
    ///
    /// Invalid UTF-8 and wire errors are returned unchanged from the concrete
    /// body's decoder. No other body type is attempted.
    pub fn decode_json_slice(self, payload: &[u8]) -> Result<Message, serde_json::Error> {
        match self {
            Self::Order => crate::Order::from_json_slice(payload).map(Message::from),
            Self::InstantActions => {
                crate::InstantActions::from_json_slice(payload).map(Message::from)
            }
            Self::State => crate::State::from_json_slice(payload).map(Message::from),
            Self::Visualization => {
                crate::Visualization::from_json_slice(payload).map(Message::from)
            }
            Self::Connection => crate::Connection::from_json_slice(payload).map(Message::from),
            Self::Factsheet => crate::Factsheet::from_json_slice(payload).map(Message::from),
            Self::ZoneSet => crate::ZoneSet::from_json_slice(payload).map(Message::from),
            Self::Responses => crate::Responses::from_json_slice(payload).map(Message::from),
        }
    }
}

impl Topic {
    /// Decode a JSON body using this topic's kind, without checking its header.
    ///
    /// Only the kind is used: manufacturer, serial number and header version
    /// are not compared. Parse raw topic strings separately to distinguish
    /// topic syntax errors from the original JSON errors returned here.
    pub fn decode_json(&self, payload: &str) -> Result<Message, serde_json::Error> {
        self.kind().decode_json(payload)
    }

    /// Decode UTF-8 JSON bytes using this topic's kind, without header checks.
    pub fn decode_json_slice(&self, payload: &[u8]) -> Result<Message, serde_json::Error> {
        self.kind().decode_json_slice(payload)
    }
}

impl TopicMessage {
    /// Encode as separate topic text and JSON body text, without an envelope.
    ///
    /// Pair consistency is preserved by construction; encoding still checks the
    /// concrete body's wire constraints and returns its original JSON error.
    pub fn to_json(&self) -> Result<(String, String), serde_json::Error> {
        let payload = match self.message() {
            Message::Order(body) => body.to_json(),
            Message::InstantActions(body) => body.to_json(),
            Message::State(body) => body.to_json(),
            Message::Visualization(body) => body.to_json(),
            Message::Connection(body) => body.to_json(),
            Message::Factsheet(body) => body.to_json(),
            Message::ZoneSet(body) => body.to_json(),
            Message::Responses(body) => body.to_json(),
        }?;
        Ok((self.topic().to_string(), payload))
    }

    /// Encode as separate topic text and UTF-8 JSON body bytes.
    ///
    /// No MQTT publish operation is performed. Unrepresentable body fields
    /// return the concrete body's original JSON error.
    pub fn to_json_vec(&self) -> Result<(String, Vec<u8>), serde_json::Error> {
        let payload = match self.message() {
            Message::Order(body) => body.to_json_vec(),
            Message::InstantActions(body) => body.to_json_vec(),
            Message::State(body) => body.to_json_vec(),
            Message::Visualization(body) => body.to_json_vec(),
            Message::Connection(body) => body.to_json_vec(),
            Message::Factsheet(body) => body.to_json_vec(),
            Message::ZoneSet(body) => body.to_json_vec(),
            Message::Responses(body) => body.to_json_vec(),
        }?;
        Ok((self.topic().to_string(), payload))
    }
}
