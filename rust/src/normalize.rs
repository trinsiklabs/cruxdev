//! Input normalization — coerce tool inputs regardless of model quirks.

use serde_json::Value;

/// Ensure a value is a list of strings (from comma-sep, JSON array, or actual vec).
pub fn to_string_list(value: &Value) -> Vec<String> {
    match value {
        Value::Null => vec![],
        Value::String(s) if s.is_empty() => vec![],
        Value::String(s) => {
            // Try JSON array first
            if let Some(stripped) = s.trim_start().strip_prefix('[') {
                let _ = stripped; // just checking prefix
                if let Ok(Value::Array(arr)) = serde_json::from_str(s) {
                    return arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.trim().to_string()))
                        .filter(|s| !s.is_empty())
                        .collect();
                }
            }
            // Fall back to comma-split
            s.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
        }
        Value::Array(arr) => arr.iter()
            .map(|v| match v {
                Value::String(s) => s.trim().to_string(),
                _ => v.to_string(),
            })
            .filter(|s| !s.is_empty())
            .collect(),
        _ => vec![value.to_string()],
    }
}

/// Ensure a value is a list of objects.
pub fn to_dict_list(value: &Value) -> Vec<Value> {
    match value {
        Value::Null => vec![],
        Value::String(s) => {
            match serde_json::from_str(s) {
                Ok(Value::Array(arr)) => arr.into_iter().filter(|v| v.is_object()).collect(),
                Ok(Value::Object(_)) => vec![serde_json::from_str(s).unwrap()],
                _ => vec![],
            }
        }
        Value::Array(arr) => arr.iter().filter(|v| v.is_object()).cloned().collect(),
        Value::Object(_) => vec![value.clone()],
        _ => vec![],
    }
}

/// Parse pipe-separated values.
pub fn to_pipe_list(value: &Value) -> Vec<String> {
    match value {
        Value::Null => vec![],
        Value::String(s) if s.is_empty() => vec![],
        Value::String(s) => s.split('|').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect(),
        Value::Array(arr) => arr.iter()
            .filter_map(|v| v.as_str().map(|s| s.trim().to_string()))
            .filter(|s| !s.is_empty())
            .collect(),
        _ => vec![],
    }
}

/// Ensure a value is an integer.
pub fn to_int(value: &Value, default: i64) -> i64 {
    match value {
        Value::Number(n) => n.as_i64().unwrap_or(default),
        Value::String(s) => s.parse().unwrap_or(default),
        Value::Null => default,
        _ => default,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn string_list_comma() {
        assert_eq!(to_string_list(&json!("a, b, c")), vec!["a", "b", "c"]);
    }

    #[test]
    fn string_list_json_array() {
        assert_eq!(to_string_list(&json!("[\"x\", \"y\"]")), vec!["x", "y"]);
    }

    #[test]
    fn string_list_actual_array() {
        assert_eq!(to_string_list(&json!(["a", "b"])), vec!["a", "b"]);
    }

    #[test]
    fn string_list_null() {
        assert!(to_string_list(&json!(null)).is_empty());
    }

    #[test]
    fn string_list_empty() {
        assert!(to_string_list(&json!("")).is_empty());
    }

    #[test]
    fn dict_list_json_string() {
        let result = to_dict_list(&json!("[{\"name\": \"x\"}]"));
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn dict_list_actual_array() {
        let result = to_dict_list(&json!([{"a": 1}, "not_dict"]));
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn dict_list_null() {
        assert!(to_dict_list(&json!(null)).is_empty());
    }

    #[test]
    fn dict_list_single_object() {
        let result = to_dict_list(&json!({"name": "x"}));
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn pipe_list() {
        assert_eq!(to_pipe_list(&json!("a|b|c")), vec!["a", "b", "c"]);
    }

    #[test]
    fn pipe_list_null() {
        assert!(to_pipe_list(&json!(null)).is_empty());
    }

    #[test]
    fn to_int_number() {
        assert_eq!(to_int(&json!(42), 0), 42);
    }

    #[test]
    fn to_int_string() {
        assert_eq!(to_int(&json!("5"), 0), 5);
    }

    #[test]
    fn to_int_null() {
        assert_eq!(to_int(&json!(null), 10), 10);
    }

    #[test]
    fn to_int_invalid() {
        assert_eq!(to_int(&json!("abc"), 7), 7);
    }
}
