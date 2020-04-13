use crate::search_engine::{SearchEngine, SearchResult};
use std::collections::HashMap;

use scraper::{Html, Selector};

pub struct Google;

impl SearchEngine for Google {
    fn search(&self, query: impl AsRef<str>) -> Vec<SearchResult> {
        println!("Searching for: {}", query.as_ref());
        let url = format!(
            "https://www.google.com/search?q={}&gws_rd=ssl",
            query.as_ref()
        );

        let resp = reqwest::blocking::get(&url).unwrap().text().unwrap();

        extract_result(resp);

        todo!()
    }
}

fn extract_result(html: String) {
    let document = Html::parse_document(&html);

    let main_div = {
        let selector = Selector::parse(r#"div[id="main"]"#).unwrap();
        document.select(&selector).next().expect("Could not find main div")
    };

    let links = {
        let selector = Selector::parse("a").unwrap();
        document.select(&selector).collect::<Vec<_>>()
    };

    for link in links {
        let href = link.value().attr("href").unwrap();

        // Google result links will always start with '/url?' because google has a sneaky redirect service
        if !href.starts_with("/url?") {
            continue;
        }

        // Now we have good evidence this is a result link,
        // which should have two child divs.
        let child_divs = {
            let selector = Selector::parse("div").unwrap();
            link.select(&selector).collect::<Vec<_>>()
        };

        if child_divs.len() != 2 {
            continue;
        }

        let title = child_divs.first().unwrap().inner_html();

        println!("Found link: {} {:?}", title, href);
    }
    
    println!("Main div: {:?}", main_div.value());
}