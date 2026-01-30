mod models;
mod parse;
mod shmulinette;

use models::{CLIArgs,TestCase,Shmuli};
use parse::{parse_args,parse_json,parse_shmuli};
use shmulinette::shmulinette;

fn main() {
    let args: CLIArgs = parse_args();

    let tests: Vec<TestCase> = parse_json(args.clone());
    let shmuli: Shmuli = parse_shmuli();

    dbg!(&args);
    dbg!(&shmuli);

    shmulinette(shmuli, tests);
}
