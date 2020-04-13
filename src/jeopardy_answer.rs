use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Answer {
    pub category: String,
    pub value: Option<String>,
    pub question: String,
    pub answer: String,
    pub round: String,
    pub show_number: String,
    pub air_date: String,
}

pub fn from_file(path: &Path) -> Vec<Answer> {
    let json_str = std::fs::read_to_string(path).expect("Couldn't read from path.");

    serde_json::from_str(&json_str).expect("Couldn't parse file content to json")
}

pub fn from_json_list(json: &str) -> Vec<Answer> {
    let x: Vec<Answer> = serde_json::from_str(json).unwrap();
    println!("{:?}", x.first().unwrap());
    todo!()
}
