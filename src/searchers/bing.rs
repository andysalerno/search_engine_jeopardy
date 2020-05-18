use crate::search_engine::{SearchEngine, SearchResult};
use scraper::{Html, Selector};

pub struct Bing;

impl SearchEngine for Bing {
    fn search(&self, query: impl AsRef<str>) -> Vec<SearchResult> {
        let url = format!(
            "https://www.bing.com/search?q={} NOT jeopardy",
            query.as_ref()
        );

        let resp = reqwest::blocking::get(&url).unwrap().text().unwrap();

        extract_result(resp)
    }
}

fn extract_result(html: String) -> Vec<SearchResult> {
    let document = Html::parse_document(&html);

    let mut results = Vec::new();

    let results_div = {
        let selector = Selector::parse(r#"ol[id="b_results"]"#).unwrap();
        document
            .select(&selector)
            .next()
            .expect("Could not find results div")
    };

    let links = {
        let selector = Selector::parse("h2 a").unwrap();
        results_div.select(&selector).collect::<Vec<_>>()
    };

    for link in links {
        let href = link
            .value()
            .attr("href")
            .expect("Encountered a link tag <a ...> without the 'href' attribute.");

        let url = href.to_string();

        // Filter out ads, self-indulgent cross-linking, etc
        if !url.starts_with("http") || url.contains("bing.com") {
            continue;
        }

        let title = link.inner_html();

        results.push(SearchResult { title, url });
    }

    results
}
