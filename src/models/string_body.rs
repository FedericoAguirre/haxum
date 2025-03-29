use serde::{Deserialize, Serialize};

// Add to Cargo.toml file serde and serde-json to use Serialize and Deserialize
// Add Debug, Clone, PartialEq, Eq to derive Debug, Clone, PartialEq, Eq
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
/// A struct that represents a key-value pair
pub struct StringBody {
    key: String,
    value: String,
}

impl StringBody {
    /// Creates a new `StringBody` instance with the given key and value.
    ///
    /// # Arguments
    ///
    /// * `key` - A string representing the key.
    /// * `value` - A string representing the value.
    ///
    /// # Returns
    ///
    /// A new instance of `StringBody`.
    pub fn new(key: String, value: String) -> Self {
        StringBody { key, value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_body_creation() {
        let key = "example_key".to_string();
        let value = "example_value".to_string();
        let string_body = StringBody {
            key: key.clone(),
            value: value.clone(),
        };

        assert_eq!(string_body.key, key);
        assert_eq!(string_body.value, value);
    }

    #[test]
    fn test_string_body_serialization() {
        let string_body = StringBody {
            key: "example_key".to_string(),
            value: "example_value".to_string(),
        };

        let serialized = serde_json::to_string(&string_body).unwrap();
        let expected = r#"{"key":"example_key","value":"example_value"}"#;

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_string_body_deserialization() {
        let json_data = r#"{"key":"example_key","value":"example_value"}"#;
        let deserialized: StringBody = serde_json::from_str(json_data).unwrap();

        assert_eq!(deserialized.key, "example_key");
        assert_eq!(deserialized.value, "example_value");
    }

    #[test]
    fn test_string_body_equality() {
        let string_body1 = StringBody::new("key1".to_string(), "value1".to_string());
        let string_body2 = StringBody::new("key1".to_string(), "value1".to_string());
        let string_body3 = StringBody::new("key2".to_string(), "value2".to_string());

        assert_eq!(string_body1, string_body2);
        assert_ne!(string_body1, string_body3);
    }
}
