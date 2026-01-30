use std::env::{args,current_exe,current_dir};
use std::fs::File;
use std::path::PathBuf;
use std::io::BufReader;

use serde_json::from_reader;

use crate::models::{Args, ArgsOption, Shmuli, TestCase};


pub fn parse_args() -> Args {
    let argv: Vec<String> = args().skip(1).collect();

    if let Some(option) = is_option(
        argv.get(0),
        argv.get(1..)
    ) {
        Args { path: None, option: Some(option) }
    } else {
        Args {
            path: argv.get(0).cloned(),
            option: Some(
                is_option(
                    argv.get(1),
                    argv.get(2..)
                ).unwrap_or(ArgsOption::Exclude(vec![]))
            ),
        }
    }
}

fn is_option(act: Option<&String>, remaining: Option<&[String]>) -> Option<ArgsOption> {
    match act.map(|s| s.as_str()) {
        Some("-o" | "--only")    => Some(ArgsOption::   Only(remaining?.to_vec())),
        Some("-e" | "--exclude") => Some(ArgsOption::Exclude(remaining?.to_vec())),
        _ => None,
    }
}

pub fn parse_json(arg: Args) -> Vec<TestCase>
{
    let path: PathBuf = find_path(arg.path.as_ref());
    let file = File::open(&path)
        .expect("Couldn't open json test file");
    let reader = BufReader::new(file);
    from_reader(reader)
        .expect("Json file is malformed")
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

