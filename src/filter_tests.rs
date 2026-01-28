use models::TestCase;

pub fn filter_tests(test_cases: TestCase, flag: String, names: Vec<String>) -> TestCase {
    match flag {
        "--only"    | "-o" => test_cases.retain(|case|  names.contains(&case.name)),
        "--exclude" | "-e" => test_cases.retain(|case| !names.contains(&case.name)),
    }
}
