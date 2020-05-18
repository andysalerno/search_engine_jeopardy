mod jeopardy_answer;
mod search_engine;
mod searchers;
mod stopword_sanitizer;

use jeopardy_answer::Answer;
use search_engine::SearchEngine;
use searchers::bing::Bing;
use searchers::google::Google;
use std::path::Path;

const JEOPARDY_ANSWERS_PATH: &str = "/home/andy/Desktop/JEOPARDY_QUESTIONS1.json";

fn main() {
    let answers = jeopardy_answer::from_file(Path::new(JEOPARDY_ANSWERS_PATH));
    // let search = Google;
    let search = Google;

    for answer in answers.into_iter().take(10) {
        println!();
        println!();

        println!("Category: {}", &answer.category);
        println!("Prize: {}", &answer.value.unwrap_or("<none>".into()));
        println!("The answer is: {}", &answer.question);

        let x = &answer.question;
        let query_str = stopword_sanitizer::sanitize_stopwords(x);
        println!("Sanitized a to be:\na: {}\nb: {}", x, query_str);
        let search_result = search.search(x);

        let url = &search_result.first().expect("No search result").url;

        println!("{}", url);

        println!(
            "Checking first result.\n\turl: {}\n\tanswer: {}",
            url, &answer.answer
        );

        let site_content = if let Ok(sc) = get_url_content(&url) {
            sc
        } else {
            println!("Skipping this question, due to a ssl/tsl issue I haven't fixed yet.");
            continue;
        };

        if site_content.contains(&answer.question) {
            println!("\tSite contains the exact question... suspicious.");
            continue;
        }

        if let Some((matching_line, matching_offset)) = test_site_for_answer(&site_content, &answer.answer) {
            println!("\tCorrect! Answer is: {}", &answer.answer);

            let start = if matching_offset > 20 {
                matching_offset - 20
            } else {
                0
            };

            let stop = if matching_line.len() - matching_offset > 20 {
                matching_offset + 20
            } else {
                matching_line.len()
            };

            println!("\tEngine said:\n\t\t{}", &matching_line[start..stop]);
        } else {
            // println!("Didn't see answer anywhere in response text:\n{}", site_content);
            println!("\tWrong! Answer is: {}", &answer.answer);
        }
    }
}

fn get_url_content(url: &str) -> Result<String, ()> {
    reqwest::blocking::get(url)
        .map_err(|e| ())?
        .text()
        .map_err(|e| ())
}

fn test_site_for_answer<'a>(site_content: &'a str, answer: &str) -> Option<(String, usize)> {
    let answer_sanitized = stopword_sanitizer::sanitize_text(answer);

    site_content
        .lines()
        .map(|l| stopword_sanitizer::sanitize_text(l))
        .filter_map(|l| l.find(&answer_sanitized).map(|offset| (l, offset)))
        .next()
}
