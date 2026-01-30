use std::str::FromStr;
use serde::Deserialize;


// --- json test ---

#[derive(Deserialize,Debug)]
pub struct TestCase {
    pub name : String,
    pub command : String,
    pub result : String,
    pub status : u32,
}

// --- shmulifile ---

#[derive(Debug)]
pub struct Shmuli {
    pub bin : String,
    pub builder : Option<String>,
    pub separator: bool,
}

#[derive(Debug)]
pub struct ShmulError;

impl FromStr for Shmuli {
    type Err = ShmulError;
    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        const VAL: [&str; 3] = ["BUILD", "BIN", "SEPARATOR" ];

        let key_val: Vec<(&str, &str)> = s.lines()
            .filter(|&l| l.contains('='))
            .filter(|&l| !l.trim().is_empty())
            .map(|x| x.split_once('=').unwrap())
            .map(|(k,v)| (k.trim(), v.trim()))
            .filter(|(k,_)| VAL.contains(k))
            .collect();

        let bin = key_val.iter()
            .find(|&&(k,_)| k == "BIN")
            .map(|&(_,v)| v.to_string())
            .ok_or(ShmulError)?;

        let builder = key_val.iter()
            .find(|&&(k,_)| k == "BUILD")
            .map(|&(_,v)| v.to_string());

        let separator = key_val.iter()
            .find(|&&(k,_)| k == "SEPARATOR")
            .map(|&(_,v)| v == "true" || v == "1")
            .unwrap_or(false);

        Ok(Shmuli { bin, builder, separator })
    }
}

// --- args parsing ---

#[derive(Debug,Clone)]
pub enum CLIArgsOption {
    Only(Vec<String>),
    Exclude(Vec<String>),
}

#[derive(Debug,Clone)]
pub struct CLIArgs {
    pub path: Option<String>,
    pub option: Option<CLIArgsOption>,
}

impl CLIArgsOption {
    pub fn should_keep(&self, test_name: &str) -> bool
    {
        match self {
            CLIArgsOption::Only(names) => names.is_empty() || names.contains(&test_name.to_string()),
            CLIArgsOption::Exclude(names) => !names.contains(&test_name.to_string()),
        }
    }
}
