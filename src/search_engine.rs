pub struct SearchResult {
    title: String,
    url: String,
}

pub trait SearchEngine {
    fn search(&self, query: impl AsRef<str>) -> Vec<SearchResult>;
}
