extern crate tokrust;
extern crate regex;


fn main() {
    let s = String::from("我非 said:\"hello, world!\"");
    let tokenizer = tokrust::TreebankTokenizer::new();
    let tokens: Vec<String> = tokenizer.tokenize(&s, false);
    for tok in tokens {
        println!("{}", tok);
    }
}
