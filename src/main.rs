mod jeopardy_answer;
mod search_engine;
mod searchers;

use jeopardy_answer::Answer;
use search_engine::SearchEngine;
use searchers::google::Google;
use std::path::Path;

const JEOPARDY_ANSWERS_PATH: &str = "/home/andy/Desktop/JEOPARDY_QUESTIONS1.json";

fn main() {
    let answers = jeopardy_answer::from_file(Path::new(JEOPARDY_ANSWERS_PATH));
    let search = Google;

    for answer in answers {
        let x = answer.question;
        let search_result = search.search(x);
    }
}
