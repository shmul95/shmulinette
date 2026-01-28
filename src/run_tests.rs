use rayon::prelude::*;
use std::process::Command;
use std::process::Output;
use std::io;

use crate::models::TestCase;

pub fn run_parallel_tests(tests: Vec<TestCase>) -> Result<(), String> {
    let failures: Vec<String> = tests
        .into_par_iter()
        .filter_map(|test| run_test(&test))
        .collect();

    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures.join("\n\n"))
    }
}

fn run_test(test: &TestCase) -> Option<String> {
    handle_result(
        test,
        Command::new("sh")
            .arg("-c")
            .arg(&test.command)
            .output()
    )
}

fn handle_result(test: &TestCase, output: io::Result<Output>) -> Option<String> {
    match output {
        Ok(out) => {
            let actual = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if actual == test.result.trim() {
                println!("\x1b[32m[ OK ]\x1b[0m {}", test.name);
                None
            } else {
                let stderr = String::from_utf8_lossy(&out.stderr);
                let msg = format!(
                    "test '{}' failed:\n  Expected: '{}'\n  Actual:   '{}'\n  Stderr:   '{}'",
                    test.name, test.result, actual, stderr.trim()
                );
                println!("\x1b[31m[ KO ]\x1b[0m {}", test.name);
                Some(msg)
            }
        }
        Err(e) => Some(format!("test '{}' could not run: {}", test.name, e)),
    }
}
