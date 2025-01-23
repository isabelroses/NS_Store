use serde_json::json;
use std::collections::HashMap;
use tera::{Result as TeraResult, Value};

const STORE_DIRECTORY: &str = "/nix/store";
const DIGEST_SIZE: usize = 32;

pub fn name_from_store_path(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    let start_index = STORE_DIRECTORY.len() + 1 + DIGEST_SIZE + 1;
    let val = &value.as_str().unwrap()[start_index..];
    Ok(json!(val))
}

pub fn strip_nix_store(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    let val = &value.as_str().unwrap()[STORE_DIRECTORY.len() + 1..];
    Ok(json!(val))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_from_store_path() {
        let value = json!("/nix/store/bzsw1khhjvg4kw1x36ajnrnxhfp94hx4-name");
        let result = name_from_store_path(&value, &HashMap::new()).unwrap();
        assert_eq!(result, json!("name"));
    }

    #[test]
    fn test_strip_nix_store() {
        let value = json!("/nix/store/bzsw1khhjvg4kw1x36ajnrnxhfp94hx4-name");
        let result = strip_nix_store(&value, &HashMap::new()).unwrap();
        assert_eq!(result, json!("bzsw1khhjvg4kw1x36ajnrnxhfp94hx4-name"));
    }
}
