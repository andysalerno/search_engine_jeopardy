pub struct SearchResult {
    pub title: String,
    pub url: String,
}

pub trait SearchEngine {
    fn search(&self, query: impl AsRef<str>) -> Vec<SearchResult>;
}
