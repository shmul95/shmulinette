mod models;
mod parse;
mod run_tests;

use std::env::{current_dir, args, current_exe};
use std::path::PathBuf;
use run_tests::run_parallel_tests;

fn main() {
    let argv: Vec<String> = args().collect();
    
    let file_path: PathBuf = match argv.len() {
        1 => {
            current_dir().unwrap_or_default().join("tests.json")
        },
        2 => {
            let mut exe_path = current_exe().expect("Failed to get exe path");
            exe_path.pop();
            exe_path.pop();
            exe_path.join("cases").join(format!("{}.json", argv[1]))
        },
        _ => {
            eprintln!("Usage: <exe> [case_name]");
            std::process::exit(1);
        }
    };

    let file_name = file_path.display().to_string();

    let test_cases = parse::validate_test_json(&file_name).expect("Invalid bundled JSON");

    if test_cases.is_empty() {
        println!("\x1b[31m[ ! ] No tests found matching that filter.\x1b[0m");
        return;
    }

    let result = run_parallel_tests(test_cases);

    match result {
        Ok(_) => {
            println!("\n\x1b[1;32mALL TESTS PASSED!\x1b[0m");
        }
        Err(error_summary) => {
            println!("\n\x1b[1;31m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\x1b[0m");
            println!("\x1b[1;31m               TEST SUITE FAILED                \x1b[0m");
            println!("\x1b[1;31m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\x1b[0m");
            
            println!("{}", error_summary);
            
            println!("\x1b[1;31m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\x1b[0m");
            
            std::process::exit(1);
        }
    }
}
