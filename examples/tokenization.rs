use std::collections::BTreeSet;

fn split_include_delimeter<'a>(txt: &'a str, re: &regex::Regex) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut last = 0;

    for m in re.find_iter(txt) {
        let idx = m.start();
        if last != idx {
            result.push(&txt[last..idx]);
        }

        let matched_str = m.as_str();
        if !matched_str.trim().is_empty() {
            result.push(m.as_str());
        }

        last = idx + matched_str.len();
    }

    if last < txt.len() {
        result.push(&txt[last..]);
    }

    result
}

struct SimpleTokenizer<'a> {
    vocab: BTreeSet<&'a str>,
}

impl<'a> SimpleTokenizer<'a> {
    pub fn new(txt: &'a str) -> Self {
        let mut preprocessed = split_include_delimeter(
            txt,
            &regex::Regex::new(r#"([,.:;?_!"()\']|--|\s)"#).expect("Failed to create regex"),
        );
        preprocessed.sort();
        Self {
            vocab: BTreeSet::from_iter(preprocessed),
        }
    }
}

fn main() {
    let txt = std::fs::read_to_string("./examples/the-verdict.txt").expect("Failed reading the verdict txt file");

    let _tokenizer = SimpleTokenizer::new(&txt);
}
