use crate::models::TestCase;
use std::fs;

pub fn validate_test_json(file_path: &String) -> Result<Vec<TestCase>, String> {
    let json_str = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file {}: {}", file_path, e))?;
    
    let tests: Vec<TestCase> = serde_json::from_str(&json_str)
        .map_err(|e| format!("Invalid JSON structure: {}", e))?;

    if tests.is_empty() {
        return Err("The JSON file contains an empty list.".to_string());
    }

    for (i, test) in tests.iter().enumerate() {
        if test.name.trim().is_empty() {
            return Err(format!("Test case at index {} is missing a name.", i));
        }
    }

    Ok(tests)
}

