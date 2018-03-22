extern crate regex;

use self::regex::Regex;

type Substitution = (Regex, &'static str);

lazy_static! {
    static ref STARTING_QUOTES: Vec<Substitution> = vec![
        (Regex::new("^\"").unwrap(), "``"),
        (Regex::new("(``)").unwrap(), " $1 "),
        (Regex::new("([ (\\[{<])(\"|\'{2})").unwrap(), "$1 `` "),
    ];
    static ref PUNCTUATION: Vec<Substitution> = vec![
        (Regex::new("([:,])([^\\d])").unwrap(), " $1 $2"),
        (Regex::new("([:,])$").unwrap(), " $1 "),
        (Regex::new("\\.\\.\\.").unwrap(), " ... "),
        (Regex::new("[;@#$%&]").unwrap(), " $0 "),
        (
            Regex::new("([^\\.])(\\.)([\\]\\)}>\"\']*)\\s*$").unwrap(),
            "$1 $2$3 ",
        ),
        (Regex::new("[?!]").unwrap(), " $0 "),
        (Regex::new("([^'])' ").unwrap(), "$1 ' "),
    ];
    static ref PARENS_BRACKETS: Vec<Substitution> =
        vec![(Regex::new(r"[\]\[\(\)\{\}<>]").unwrap(), " $0 ")];
    static ref CONVERT_PARENTHESES: Vec<Substitution> = vec![
        (Regex::new("\\(").unwrap(), "-LRB-"),
        (Regex::new("\\)").unwrap(), "-RRB-"),
        (Regex::new("\\[").unwrap(), "-LSB-"),
        (Regex::new("\\]").unwrap(), "-RSB-"),
        (Regex::new("\\{").unwrap(), "-LCB-"),
        (Regex::new("\\}").unwrap(), "-RCB-"),
    ];
    static ref DOUBLE_DASHES: Vec<Substitution> = vec![(Regex::new("--").unwrap(), " -- ")];
    static ref ENDING_QUOTES: Vec<Substitution> = vec![
        (Regex::new("\"").unwrap(), " '' "),
        (Regex::new("(\\S)(\'\')").unwrap(), "$1 $2 "),
        (
            Regex::new("([^' ])('[sS]|'[mM]|'[dD]|') ").unwrap(),
            "$1 $2 ",
        ),
        (
            Regex::new("([^' ])('ll|'LL|'re|'RE|'ve|'VE|n't|N'T) ").unwrap(),
            "$1 $2 ",
        ),
    ];
}

pub fn tokenize(text: &str, convert_parentheses: bool) -> Vec<String> {
    let mut text = text.to_string();
    for &(ref regexp, substitution) in (&STARTING_QUOTES).iter() {
        text = regexp.replace_all(&text, substitution).to_string();
    }
    for &(ref regexp, substitution) in (&PUNCTUATION).iter() {
        text = regexp.replace_all(&text, substitution).to_string();
    }

    // Handles parentheses.
    let &(ref regexp, substitution) = &PARENS_BRACKETS[0];
    text = regexp.replace_all(&text, substitution).to_string();
    // Optionally convert parentheses
    if convert_parentheses {
        for &(ref regexp, substitution) in (&CONVERT_PARENTHESES).iter() {
            text = regexp.replace_all(&text, substitution).to_string();
        }
    }

    // Handles double dash.
    let &(ref regexp, substitution) = &DOUBLE_DASHES[0];
    text = regexp.replace_all(&text, substitution).to_string();

    // add extra space to make things easier
    text.insert(0, ' ');
    text.push(' ');

    for &(ref regexp, substitution) in (&ENDING_QUOTES).iter() {
        text = regexp.replace_all(&text, substitution).to_string();
    }

    text.split_whitespace().map(|x| x.to_string()).collect()
}
