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

    for answer in answers.into_iter().take(10) {
        println!();
        println!();

        println!("Category: {}", &answer.category);
        println!("Prize: {}", &answer.value.unwrap_or("<none>".into()));
        println!("The answer is: {}", &answer.question);

        let x = &answer.question;
        let search_result = search.search(x);

        let url = &search_result.first().expect("No search result").url;

        println!(
            "Checking first result.\n\turl: {}\n\tanswer: {}",
            url, &answer.answer
        );

        let site_content = get_url_content(&url);

        if site_content.contains(&answer.question) {
            println!("\tSite contains the exact question... suspicious.");
            continue;
        }

        if let Some(result_line) = test_site_for_answer(&site_content, &answer.answer) {
            println!("\tCorrect! Answer is: {}", &answer.answer);
            println!("\tEngine said:\n\t\t{}", result_line);
        } else {
            println!("\tWrong! Answer is: {}", &answer.answer);
        }
    }
}

fn get_url_content(url: &str) -> String {
    reqwest::blocking::get(url).unwrap().text().unwrap()
}

fn test_site_for_answer<'a>(site_content: &'a str, answer: &str) -> Option<&'a str> {
    site_content.lines().filter(|l| l.contains(answer)).next()
}
