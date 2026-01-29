use crate::models::TestCase;

/// Filters test cases based on the provided flag and test names
/// 
/// # Arguments
/// * `test_cases` - Vector of test cases to filter
/// * `flag` - Filter flag ("--only"/"-o" or "--exclude"/"-e")
/// * `names` - Vector of test names to filter by
/// 
/// # Returns
/// Filtered vector of test cases
pub fn filter_tests(mut test_cases: Vec<TestCase>, flag: String, names: Vec<String>) -> Vec<TestCase> {
    if names.is_empty() {
        return test_cases;
    }
    
    match flag.as_str() {
        "--only" | "-o" => {
            test_cases.retain(|case| names.contains(&case.name));
            if !names.is_empty() && test_cases.is_empty() {
                eprintln!("\x1b[33m[ ! ] Warning: No tests matched the --only filter. Available tests:\x1b[0m");
                // This won't work since test_cases is already filtered, but we'll handle this in main
            }
        },
        "--exclude" | "-e" => {
            test_cases.retain(|case| !names.contains(&case.name));
        },
        _ => {
            eprintln!("\x1b[31m[ ! ] Unknown filter flag: {}\x1b[0m", flag);
        }
    }
    test_cases
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::TestCase;

    fn create_test_case(name: &str) -> TestCase {
        TestCase {
            name: name.to_string(),
            command: "echo test".to_string(),
            result: "test".to_string(),
            status: 0,
        }
    }

    #[test]
    fn test_filter_only() {
        let test_cases = vec![
            create_test_case("test1"),
            create_test_case("test2"),
            create_test_case("test3"),
        ];
        
        let filtered = filter_tests(
            test_cases,
            "--only".to_string(),
            vec!["test1".to_string(), "test3".to_string()]
        );
        
        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0].name, "test1");
        assert_eq!(filtered[1].name, "test3");
    }

    #[test]
    fn test_filter_exclude() {
        let test_cases = vec![
            create_test_case("test1"),
            create_test_case("test2"),
            create_test_case("test3"),
        ];
        
        let filtered = filter_tests(
            test_cases,
            "--exclude".to_string(),
            vec!["test2".to_string()]
        );
        
        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0].name, "test1");
        assert_eq!(filtered[1].name, "test3");
    }

    #[test]
    fn test_empty_names() {
        let test_cases = vec![
            create_test_case("test1"),
            create_test_case("test2"),
        ];
        
        let filtered = filter_tests(
            test_cases.clone(),
            "--only".to_string(),
            vec![]
        );
        
        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0].name, test_cases[0].name);
        assert_eq!(filtered[1].name, test_cases[1].name);
    }
}
