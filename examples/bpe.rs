use std::collections::HashSet;

fn main() {
    let tokenizer = tiktoken_rs::get_bpe_from_tokenizer(
        tiktoken_rs::tokenizer::get_tokenizer("gpt-3.5").expect("Expect tokenizer to exist for gpt 3"),
    )
    .expect("CoreBPE returned err");

    let sample_txt = std::fs::read_to_string("./examples/the-verdict.txt").expect("Failed to load the verdict txt");

    let (enc_txt, _size) = tokenizer.encode(&sample_txt, &HashSet::new());

    println!("Total encoded txt: {:?}", enc_txt.len());
}
