use std::fmt;
use std::str::FromStr;

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Metadata {
    pub name: String,
    pub length: usize,
    pub hash: String,
}

impl Metadata {
    pub fn new(name: String, length: usize, hash: String) -> Self {
        Self { name, length, hash }
    }
}

impl fmt::Display for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "METADATA:{},{},{}", self.name, self.length, self.hash)
    }
}

impl FromStr for Metadata {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("METADATA:") {
            return Err("Metadata must start with 'METADATA:'".to_string());
        }

        let data = &s[9..];
        let parts: Vec<&str> = data.split(',').collect();

        if parts.len() != 3 {
            return Err(format!("Expected 3 parts, got {}", parts.len()));
        }

        let length = parts[1]
            .parse::<usize>()
            .map_err(|e| format!("Failed to parse length: {}", e))?;

        Ok(Metadata {
            name: parts[0].to_string(),
            length,
            hash: parts[2].to_string(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Message {
    Metadata(Metadata),
    Piece { index: usize, data: String },
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Message::Metadata(metadata) => write!(f, "{}", metadata),
            Message::Piece { index, data } => write!(f, "{}:{}", index, data),
        }
    }
}

impl FromStr for Message {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("METADATA:") {
            let metadata = Metadata::from_str(s)?;
            Ok(Message::Metadata(metadata))
        } else {
            let parts: Vec<&str> = s.split(':').collect();
            if parts.len() < 2 {
                return Err("Message must contain at least one ':'".to_string());
            }

            let index = parts[0]
                .parse::<usize>()
                .map_err(|e| format!("Failed to parse index: {}", e))?;

            Ok(Message::Piece {
                index,
                data: parts[1].to_string(),
            })
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Payload {
    pub metadata: Metadata,
    pub pieces: Vec<(usize, String)>,
}

impl Payload {
    pub fn new(metadata: Metadata, pieces: Vec<(usize, String)>) -> Self {
        Self { metadata, pieces }
    }

    pub fn len(&self) -> usize {
        1 + self.pieces.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pieces.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_to_string() {
        let metadata = Metadata::new("test_name".to_string(), 42, "abc123".to_string());
        assert_eq!(metadata.to_string(), "METADATA:test_name,42,abc123");
    }

    #[test]
    fn test_metadata_from_str() {
        let s = "METADATA:test_name,42,abc123";
        let metadata = Metadata::from_str(s).unwrap();
        assert_eq!(metadata.name, "test_name");
        assert_eq!(metadata.length, 42);
        assert_eq!(metadata.hash, "abc123");
    }

    #[test]
    fn test_message_metadata_roundtrip() {
        let msg = Message::Metadata(Metadata::new(
            "filename".to_string(),
            10,
            "hash123".to_string(),
        ));
        let s = msg.to_string();
        let parsed = Message::from_str(&s).unwrap();
        assert_eq!(msg, parsed);
    }

    #[test]
    fn test_message_piece_roundtrip() {
        let msg = Message::Piece {
            index: 5,
            data: "somedata".to_string(),
        };
        let s = msg.to_string();
        let parsed = Message::from_str(&s).unwrap();
        assert_eq!(msg, parsed);
    }

    #[test]
    fn test_payload_creation() {
        let metadata = Metadata::new("testfile".to_string(), 2, "hash123".to_string());
        let pieces = vec![(1, "data1".to_string()), (2, "data2".to_string())];
        let payload = Payload::new(metadata.clone(), pieces.clone());

        assert_eq!(payload.metadata, metadata);
        assert_eq!(payload.pieces, pieces);
        assert_eq!(payload.len(), 3);
        assert!(!payload.is_empty());
    }
}
