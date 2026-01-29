mod models;
mod parse;
mod run_tests;
mod filter_tests;

use std::env::{current_dir, args, current_exe};
use std::path::PathBuf;
use run_tests::run_parallel_tests;
use parse::validate_test_json;
use filter_tests::filter_tests;

#[derive(Default)]
struct Args {
    file_path: PathBuf,
    filter_flag: Option<String>,
    filter_names: Vec<String>,
}

fn parse_args() -> Args {
    let argv: Vec<String> = args().collect();
    let mut args = Args::default();
    let mut i = 1;
    
    // Parse file argument first
    if argv.len() > 1 && !argv[1].starts_with('-') {
        let mut exe_path = current_exe().expect("Failed to get exe path");
        exe_path.pop();
        exe_path.pop();
        args.file_path = exe_path.join("cases").join(format!("{}.json", argv[1]));
        i = 2;
    } else {
        args.file_path = current_dir().unwrap_or_default().join("cases").join("tests.json");
    }
    
    // Parse filter flags
    while i < argv.len() {
        match argv[i].as_str() {
            "--only" | "-o" | "--exclude" | "-e" => {
                if i + 1 < argv.len() {
                    args.filter_flag = Some(argv[i].clone());
                    i += 1;
                    // Collect comma-separated test names
                    args.filter_names = argv[i].split(',').map(|s| s.trim().to_string()).collect();
                    i += 1;
                } else {
                    eprintln!("Error: {} requires test names", argv[i]);
                    print_usage();
                    std::process::exit(1);
                }
            },
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            },
            _ => {
                eprintln!("Unknown argument: {}", argv[i]);
                print_usage();
                std::process::exit(1);
            }
        }
    }
    
    args
}

fn print_usage() {
    println!("Usage: shmulinette [case_name] [OPTIONS]");
    println!();
    println!("Arguments:");
    println!("  case_name           JSON test case file (without .json extension)");
    println!();
    println!("Options:");
    println!("  -o, --only <names>     Run only the specified tests (comma-separated)");
    println!("  -e, --exclude <names>  Exclude the specified tests (comma-separated)");
    println!("  -h, --help            Show this help message");
    println!();
    println!("Examples:");
    println!("  shmulinette                           # Run all tests from tests.json");
    println!("  shmulinette 1_test                    # Run tests from 1_test.json");
    println!("  shmulinette -o \"Simple Arithmetic\"    # Run only the Simple Arithmetic test");
    println!("  shmulinette -e \"Date Formatting\"      # Run all except Date Formatting test");
}

fn main() {
    let args = parse_args();
    let file_name = args.file_path.display().to_string();

    let test_cases = validate_test_json(&file_name).expect("Invalid bundled JSON");
    
    // Show available tests for debugging if requested
    if args.filter_flag.is_some() && args.filter_names.is_empty() {
        println!("\x1b[33m[ ! ] No test names provided. Available tests:\x1b[0m");
        for case in &test_cases {
            println!("  - {}", case.name);
        }
        return;
    }
    
    let original_count = test_cases.len();
    let test_cases = if let Some(flag) = args.filter_flag {
        filter_tests(test_cases, flag, args.filter_names)
    } else {
        test_cases
    };

    if test_cases.is_empty() {
        if original_count > 0 {
            println!("\x1b[31m[ ! ] No tests found matching that filter. Available tests:\x1b[0m");
            let all_test_cases = validate_test_json(&file_name).expect("Invalid bundled JSON");
            for case in &all_test_cases {
                println!("  - {}", case.name);
            }
        } else {
            println!("\x1b[31m[ ! ] No tests found in the test file.\x1b[0m");
        }
        return;
    }

    println!("\x1b[36m[ * ] Running {} test(s)...\x1b[0m", test_cases.len());

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
