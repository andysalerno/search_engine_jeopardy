use lazy_static::*;
use std::collections::HashSet;

lazy_static! {
    static ref STOP_WORDS: HashSet<&'static str> = {
        let mut stop_words = HashSet::new();

        for word in _STOP_WORDS.iter() {
            stop_words.insert(*word);
        }

        stop_words
    };
}

pub(crate) fn remove_stopwords(input: &str) -> String {
    let input = input.to_lowercase();

    let filtered_words = input
        .split_whitespace()
        .filter(|w| !STOP_WORDS.contains(w))
        .map(|w| sanitize_characters(w));

    filtered_words.collect::<Vec<String>>().join(" ").into()
}

pub(crate) fn sanitize_text(text: impl AsRef<str>) -> String {
    sanitize_characters(text.as_ref().to_lowercase())
}

fn sanitize_characters(text: impl AsRef<str>) -> String {
    text.as_ref()
        // ... sanitize HTML encoding for the single-quote... huge hack, obviously need to handle them all
        .replace("&#39;", "")
        .chars()
        .filter(|&c| c != '\\')
        .filter(|&c| c != '\'')
        .filter(|&c| c != ',')
        .map(|c| {
            if c.is_ascii_alphanumeric() || c.is_ascii_whitespace() {
                c
            } else {
                ' '
            }
        })
        .collect()
}

const _STOP_WORDS: [&str; 127] = [
    "i",
    "me",
    "my",
    "myself",
    "we",
    "our",
    "ours",
    "ourselves",
    "you",
    "your",
    "yours",
    "yourself",
    "yourselves",
    "he",
    "him",
    "his",
    "himself",
    "she",
    "her",
    "hers",
    "herself",
    "it",
    "its",
    "itself",
    "they",
    "them",
    "their",
    "theirs",
    "themselves",
    "what",
    "which",
    "who",
    "whom",
    "this",
    "that",
    "these",
    "those",
    "am",
    "is",
    "are",
    "was",
    "were",
    "be",
    "been",
    "being",
    "have",
    "has",
    "had",
    "having",
    "do",
    "does",
    "did",
    "doing",
    "a",
    "an",
    "the",
    "and",
    "but",
    "if",
    "or",
    "because",
    "as",
    "until",
    "while",
    "of",
    "at",
    "by",
    "for",
    "with",
    "about",
    "against",
    "between",
    "into",
    "through",
    "during",
    "before",
    "after",
    "above",
    "below",
    "to",
    "from",
    "up",
    "down",
    "in",
    "out",
    "on",
    "off",
    "over",
    "under",
    "again",
    "further",
    "then",
    "once",
    "here",
    "there",
    "when",
    "where",
    "why",
    "how",
    "all",
    "any",
    "both",
    "each",
    "few",
    "more",
    "most",
    "other",
    "some",
    "such",
    "no",
    "nor",
    "not",
    "only",
    "own",
    "same",
    "so",
    "than",
    "too",
    "very",
    "s",
    "t",
    "can",
    "will",
    "just",
    "don",
    "should",
    "now",
];
