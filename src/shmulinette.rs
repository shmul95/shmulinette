use std::process::{Command, ExitStatus, Output, exit};
use std::os::unix::process::ExitStatusExt;
use std::io;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::models::{Shmuli,TestCase};


pub fn shmulinette(shmuli: Shmuli, tests: Vec<TestCase>)
{
    build(shmuli.builder);

    match run_tests(tests) {
        Ok(_) => println!("\x1b[32mAll tests pass\x1b[0m"),
        Err(e) => println!("\x1b[31mSome issues with your code\x1b[0m\n{}", e),
    };
}

fn get_shell() -> (&'static str, &'static str)
{
    if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    }
}

fn build(build_cmd: Option<String>)
{
    let output = match build_cmd {
        Some(cmd) => {
            let (shell, arg) = get_shell();
            Command::new(shell)
                .arg(arg)
                .arg(&cmd)
                .output()
        },
        None => Ok(Output {
            status: ExitStatus::from_raw(0),
            stdout: Vec::new(),
            stderr: Vec::new()
        })
    }.expect("Error happend in the build");
    if !output.status.success() {
        eprintln!("Build Failed: {}", String::from_utf8_lossy(&output.stderr));
        exit(1);
    }
}

fn run_tests(tests: Vec<TestCase>) -> Result<(), String>
{
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

fn run_test(test: &TestCase) -> Option<String>
{
    let (shell, arg) = get_shell();
    handle_result(
        test,
        Command::new(shell)
            .arg(arg)
            .arg(&test.command)
            .output()
    )
}

fn handle_result(test: &TestCase, output: io::Result<Output>) -> Option<String> {
    match output {
        Ok(out) => {
            let actual = String::from_utf8_lossy(&out.stdout).trim().to_string();
            let exit_status = out.status.code().unwrap_or(-1) as u32;
            
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
