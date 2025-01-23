use serde_json::json;
use std::collections::HashMap;
use tera::{Result as TeraResult, Value};

pub fn human_readable_size(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    let size_in_bytes = value
        .as_u64()
        .ok_or_else(|| tera::Error::msg("Filter `human_readable_size` expected a number."))?;
    let units = ["B", "K", "M", "G", "T"];
    let mut size = size_in_bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < units.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    Ok(json!(format!("{:.1}{}", size, units[unit_index])))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_human_readable_size() {
        let value = json!(1024);
        let result = human_readable_size(&value, &HashMap::new()).unwrap();
        assert_eq!(result, json!("1.0K"));

        let value = json!(1024 * 1024);
        let result = human_readable_size(&value, &HashMap::new()).unwrap();
        assert_eq!(result, json!("1.0M"));

        let value = json!(1024 * 1024 * 1024);
        let result = human_readable_size(&value, &HashMap::new()).unwrap();
        assert_eq!(result, json!("1.0G"));
    }
}
