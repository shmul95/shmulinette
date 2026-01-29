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
            let exit_status = out.status.code().unwrap_or(-1) as u32;
            
            // Check both result and status
            let result_matches = actual == test.result.trim();
            let status_matches = exit_status == test.status;
            
            if result_matches && status_matches {
                println!("\x1b[32m[ OK ]\x1b[0m {}", test.name);
                None
            } else {
                let stderr = String::from_utf8_lossy(&out.stderr);
                let mut msg = format!("test '{}' failed:\n", test.name);
                
                if !result_matches {
                    msg.push_str(&format!("  Expected output: '{}'\n  Actual output:   '{}'\n", 
                        test.result, actual));
                }
                
                if !status_matches {
                    msg.push_str(&format!("  Expected status: {}\n  Actual status:   {}\n", 
                        test.status, exit_status));
                }
                
                if !stderr.trim().is_empty() {
                    msg.push_str(&format!("  Stderr: '{}'\n", stderr.trim()));
                }
                
                println!("\x1b[31m[ KO ]\x1b[0m {}", test.name);
                Some(msg)
            }
        }
        Err(e) => Some(format!("test '{}' could not run: {}", test.name, e)),
    }
}
