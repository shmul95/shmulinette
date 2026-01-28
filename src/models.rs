use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TestCase {
    pub name: String,
    pub command: String,
    pub result: String,
}

