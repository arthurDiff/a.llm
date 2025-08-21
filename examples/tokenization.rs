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

fn main() {
    let txt = std::fs::read_to_string("./examples/the-verdict.txt").expect("Failed reading the verdict txt file");

    // the verdict continas 20479 characters
    println!("Total num of chars: {:?}", txt.chars().count());

    let preprocessed = split_include_delimeter(
        &txt,
        &regex::Regex::new(r#"([,.:;?_!"()\']|--|\s)"#).expect("Failed to create regex"),
    );

    println!("Preprocessed len: {}", preprocessed.len());
    println!("{:?}", &preprocessed[..30]);
}
