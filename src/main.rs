mod models;
mod parse;

use models::{Args,TestCase,Shmuli};
use parse::{parse_args,parse_json,parse_shmuli};

fn main() {
    let args: Args = parse_args();
    let tests: Vec<TestCase> = parse_json(args.clone());
    let shmuli: Shmuli = parse_shmuli();

    dbg!(args);
    dbg!(shmuli);
}
