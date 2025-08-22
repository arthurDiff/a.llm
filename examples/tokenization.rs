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
    #[allow(dead_code)]
    eot_id: usize,
    unk_id: usize,
    vocab: BTreeSet<&'a str>,
}

impl<'a> SimpleTokenizer<'a> {
    pub fn new(txt: &'a str) -> Self {
        let encode_regex = regex::Regex::new(r#"([,.:;?_!"()\']|--|\s)"#).expect("Failed to create encode regex");
        let decode_regex = regex::Regex::new(r#"\s+([,.:;?_!"()\'])"#).expect("Failed to create decode regex");

        let mut vocab_vec = split_include_delimeter(txt, &encode_regex);
        vocab_vec.push("<|endoftext|>");
        vocab_vec.push("<|unk|>");

        let vocab = BTreeSet::from_iter(vocab_vec);

        Self {
            encode_regex,
            decode_regex,
            eot_id: vocab
                .iter()
                .enumerate()
                .find(|(_, vs)| **vs == "<|endoftext|>")
                .expect("End of text token is missing")
                .0,
            unk_id: vocab
                .iter()
                .enumerate()
                .find(|(_, vs)| **vs == "<|unk|>")
                .expect("Unknown token is missing")
                .0,
            vocab,
        }
    }

    pub fn encode(&self, txt: &str) -> Vec<usize> {
        split_include_delimeter(txt, &self.encode_regex)
            .into_iter()
            .map(|s| {
                if let Some((idx, _)) = self.vocab.iter().enumerate().find(|(_, vs)| **vs == s) {
                    idx
                } else {
                    self.unk_id
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

    // Unknown check

    let txt_1 = "Hello, do you like tea?";
    let txt_2 = "Inthe sunlit something of the palace.";
    let new_txt = format!("{txt_1} <|endoftext|> {txt_2}");
    println!("predefined token check txt: {new_txt}");

    let new_ids = tokenizer.encode(&new_txt);
    println!("Some unknow token ids: {:?}", new_ids);

    let new_decoded = tokenizer.decode(new_ids);
    println!("Some unknow token decoded: {:?}", new_decoded);
}
