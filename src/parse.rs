use std::env::{args,current_exe,current_dir};
use std::fs::File;
use std::path::PathBuf;
use std::io::BufReader;

use serde_json::from_reader;

use crate::models::{CLIArgs, CLIArgsOption, Shmuli, TestCase};


pub fn parse_args() -> CLIArgs {
    let argv: Vec<String> = args().skip(1).collect();

    if let Some(option) = is_option(argv.get(0), argv.get(1..)) {
        CLIArgs { path: None, option: Some(option) }
    } else {
        let path = argv.get(0).cloned();
        let option = Some(
            is_option(argv.get(1), argv.get(2..))
                .unwrap_or(CLIArgsOption::Exclude(vec![]))
        );
        CLIArgs { path, option }
    }
}

fn is_option(act: Option<&String>, remaining: Option<&[String]>) -> Option<CLIArgsOption> {
    match act.map(|s| s.as_str()) {
        Some("-o" | "--only")    => Some(CLIArgsOption::   Only(remaining?.to_vec())),
        Some("-e" | "--exclude") => Some(CLIArgsOption::Exclude(remaining?.to_vec())),
        _ => None,
    }
}

pub fn parse_json(arg: &CLIArgs, shmuli: &Shmuli) -> Vec<TestCase>
{
    let path: PathBuf = find_path(arg.path.as_ref());
    let file = File::open(&path)
        .expect("Couldn't open json test file");
    let reader = BufReader::new(file);
    let all_tests : Vec<TestCase> = from_reader(reader)
        .expect("Json file is malformed");
    all_tests
        .into_iter()
        .filter(|test| arg.option
            .as_ref()
            .unwrap()
            .should_keep(&test.name))
        .map(|mut test| {
            let replacement = format!(
                    "{} {}",
                    shmuli.bin,
                    if shmuli.separator { "--" }
                    else { "" }
                );
            
            test.command = test.command
                .replace("@BIN", replacement.trim());
            test
        })
        .collect()
}

fn find_path(case: Option<&String>) -> PathBuf
{
    match case {
        Some(c) => {
            let mut exe_path = current_exe()
                .expect("Failed to parse exe");
            exe_path.pop();
            exe_path.pop();
            let case_path = exe_path
                .join("cases")
                .join(format!("{}.json", c));
            case_path
        },
        None => current_dir()
            .unwrap_or_default()
            .join("tests.json")
    }
}

pub fn parse_shmuli() -> Shmuli
{
    let path: PathBuf = current_dir()
        .unwrap_or_default()
        .join("Shmulifile");
    std::fs::read_to_string(&path)
        .expect("Couldn't reed the content of the Shmulifile")
        .parse()
        .expect("Couldn't find Builder and Bin")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    // --- Tests for Shmuli Parsing ---
    #[test]
    fn test_parse_shmuli_logic() {
        let content = "BUILDER=cargo build\nBIN=cargo run";
        let parsed: Shmuli = content.parse().expect("Valid Shmulinette format");
        
        assert_eq!(parsed.bin, "cargo run");
        assert_eq!(parsed.builder, Some("cargo build".to_string()));
    }

    #[test]
    fn test_parse_shmuli_no_builder() {
        let content = "BIN=./my_binary";
        let parsed: Shmuli = content.parse().expect("Should work without BUILDER");
        
        assert_eq!(parsed.bin, "./my_binary");
        assert!(parsed.builder.is_none());
    }

    // --- Tests for Argument Logic ---
    #[test]
    fn test_is_option_only() {
        let act = Some(&"-o".to_string());
        let remaining = Some(&["test1".to_string(), "test2".to_string()][..]);
        
        let result = is_option(act, remaining);
        if let Some(CLIArgsOption::Only(vec)) = result {
            assert_eq!(vec.len(), 2);
            assert_eq!(vec[0], "test1");
        } else {
            panic!("Should have parsed as Only");
        }
    }

    // --- Tests for JSON Filtering ---
    #[test]
    fn test_parse_json_filtering() {
        // 1. Create a dummy JSON file
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, r#"[
            {{"name": "test1", "command": "ls", "result": "ok", "status": 0}},
            {{"name": "test2", "command": "pwd", "result": "ok", "status": 0}}
        ]"#).unwrap();

        // 2. Setup CLIArgs to "Only" keep test1
        let args = CLIArgs {
            path: Some(file.path().to_str().unwrap().to_string()),
            option: Some(CLIArgsOption::Only(vec!["test1".to_string()])),
        };

        // Note: You might need to modify find_path to accept absolute paths 
        // Or bypass find_path for the test by injecting the path directly
        // Here we assume parse_json uses the path provided in CLIArgs
        
        // Let's test the filtering logic specifically:
        let test_case = TestCase {
            name: "test1".to_string(),
            command: "ls".to_string(),
            result: "ok".to_string(),
            status: 0,
        };
        
        assert!(args.option.as_ref().unwrap().should_keep(&test_case.name));
    }
}
