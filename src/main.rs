extern crate regex;
extern crate tokrust;

fn main() {
    let s = String::from("我非 said:\"hello, world!\"");
    let tokens: Vec<String> = tokrust::treebank::tokenize(&s, false);
    for tok in tokens {
        println!("{}", tok);
    }
}
