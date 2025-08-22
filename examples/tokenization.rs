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

fn replace_with_trimed_delimeter(txt: &str, re: &regex::Regex) -> String {
    let mut result = Vec::new();
    let mut last = 0;
    for m in re.find_iter(txt) {
        let idx = m.start();
        if last != idx {
            result.push(&txt[last..idx]);
        }

        let matched_str = m.as_str();
        if !matched_str.trim().is_empty() {
            result.push(m.as_str().trim());
        }

        last = idx + matched_str.len();
    }

    if last < txt.len() {
        result.push(&txt[last..]);
    }

    result.join("")
}

struct SimpleTokenizer<'a> {
    encode_regex: regex::Regex,
    decode_regex: regex::Regex,
    vocab: BTreeSet<&'a str>,
}

impl<'a> SimpleTokenizer<'a> {
    pub fn new(txt: &'a str) -> Self {
        let encode_regex = regex::Regex::new(r#"([,.:;?_!"()\']|--|\s)"#).expect("Failed to create encode regex");
        let decode_regex = regex::Regex::new(r#"\s+([,.:;?_!"()\'])"#).expect("Failed to create decode regex");

        let mut preprocessed = split_include_delimeter(txt, &encode_regex);

        preprocessed.sort();

        Self {
            encode_regex,
            decode_regex,
            vocab: BTreeSet::from_iter(preprocessed),
        }
    }

    pub fn encode(&self, txt: &str) -> Vec<usize> {
        split_include_delimeter(txt, &self.encode_regex)
            .into_iter()
            .map(|s| {
                if let Some((idx, _)) = self.vocab.iter().enumerate().find(|(_, vs)| **vs == s) {
                    idx
                } else {
                    0
                }
            })
            .collect()
    }

    pub fn decode(&self, ids: Vec<usize>) -> String {
        let decoded_str: Vec<&str> = ids
            .iter()
            .map(|idx| {
                if let Some(v) = self.vocab.iter().nth(*idx) {
                    *v
                } else {
                    ""
                }
            })
            .collect();

        replace_with_trimed_delimeter(&decoded_str.join(" "), &self.decode_regex)
    }
}

fn main() {
    let base_txt = std::fs::read_to_string("./examples/the-verdict.txt").expect("Failed reading the verdict txt file");
    let tokenizer = SimpleTokenizer::new(&base_txt);

    let txt = r#""It's the last he painted, you know," Mrs. Gisburn said with pardonable pride.""#;

    let ids = tokenizer.encode(txt);
    println!("Tokenized Ids: {:?}", ids);

    let decoded_txt = tokenizer.decode(ids);
    println!("Decoded Ids: {}", decoded_txt);
}
