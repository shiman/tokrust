extern crate regex;

use self::regex::Regex;

type Substitution = (Regex, &'static str);

pub struct TreebankTokenizer {
    starting_quotes: Vec<Substitution>,
    punctuation: Vec<Substitution>,
    parens_brackets: Vec<Substitution>,
    convert_parentheses: Vec<Substitution>,
    double_dashes: Vec<Substitution>,
    ending_quotes: Vec<Substitution>,
//    CONTRACTIONS: Vec<Regex>,
}

impl TreebankTokenizer {
    pub fn new() -> TreebankTokenizer {
        TreebankTokenizer {
            starting_quotes: vec![
                (Regex::new("^\"").unwrap(), "``"),
                (Regex::new("(``)").unwrap(), " $1 "),
                (Regex::new("([ (\\[{<])(\"|\'{2})").unwrap(), "$1 `` "),
            ],
            punctuation: vec![
                (Regex::new("([:,])([^\\d])").unwrap(), " $1 $2"),
                (Regex::new("([:,])$").unwrap(), " $1 "),
                (Regex::new("\\.\\.\\.").unwrap(), " ... "),
                (Regex::new("[;@#$%&]").unwrap(), " $0 "),
                (Regex::new("([^\\.])(\\.)([\\]\\)}>\"\']*)\\s*$").unwrap(), "$1 $2$3 "),  // Handles the final period.
                (Regex::new("[?!]").unwrap(), " $0 "),
                (Regex::new("([^'])' ").unwrap(), "$1 ' "),
            ],
            parens_brackets: vec![
                (Regex::new(r"[\]\[\(\)\{\}<>]").unwrap(), " $0 ")
            ],
            convert_parentheses: vec![
                (Regex::new("\\(").unwrap(), "-LRB-"),
                (Regex::new("\\)").unwrap(), "-RRB-"),
                (Regex::new("\\[").unwrap(), "-LSB-"),
                (Regex::new("\\]").unwrap(), "-RSB-"),
                (Regex::new("\\{").unwrap(), "-LCB-"),
                (Regex::new("\\}").unwrap(), "-RCB-")
            ],
            double_dashes: vec![
                (Regex::new("--").unwrap(), " -- ")
            ],
            ending_quotes: vec![
                (Regex::new("\"").unwrap(), " '' "),
                (Regex::new("(\\S)(\'\')").unwrap(), "$1 $2 "),
                (Regex::new("([^' ])('[sS]|'[mM]|'[dD]|') ").unwrap(), "$1 $2 "),
                (Regex::new("([^' ])('ll|'LL|'re|'RE|'ve|'VE|n't|N'T) ").unwrap(), "$1 $2 "),
            ],
//            CONTRACTIONS: vec![
//                Regex::new(r"(?i)\b(can)(?#X)(not)\b").unwrap(),
//                Regex::new(r"(?i)\b(d)(?#X)('ye)\b").unwrap(),
//                Regex::new(r"(?i)\b(gim)(?#X)(me)\b").unwrap(),
//                Regex::new(r"(?i)\b(gon)(?#X)(na)\b").unwrap(),
//                Regex::new(r"(?i)\b(got)(?#X)(ta)\b").unwrap(),
//                Regex::new(r"(?i)\b(lem)(?#X)(me)\b").unwrap(),
//                Regex::new(r"(?i)\b(mor)(?#X)('n)\b").unwrap(),
//                Regex::new(r"(?i)\b(wan)(?#X)(na)\s").unwrap(),
//                Regex::new(r"(?i) ('t)(?#X)(is)\b").unwrap(),
//                Regex::new(r"(?i) ('t)(?#X)(was)\b").unwrap(),
//            ]
        }
    }

    pub fn tokenize(&self, text: &str, convert_parentheses: bool) -> Vec<String> {
        let mut text = text.to_string();
        for &(ref regexp, substitution) in &self.starting_quotes {
            text = regexp.replace_all(&text, substitution).to_string();
        }
        for &(ref regexp, substitution) in &self.punctuation {
            text = regexp.replace_all(&text, substitution).to_string();
        }

        // Handles parentheses.
        let &(ref regexp, substitution) = &self.parens_brackets[0];
        text = regexp.replace_all(&text, substitution).to_string();
        // Optionally convert parentheses
        if convert_parentheses {
            for &(ref regexp, substitution) in &self.convert_parentheses {
                text = regexp.replace_all(&text, substitution).to_string();
            }
        }

        // Handles double dash.
        let &(ref regexp, substitution) = &self.double_dashes[0];
        text = regexp.replace_all(&text, substitution).to_string();

        // add extra space to make things easier
        text.insert(0, ' ');
        text.push(' ');

        for &(ref regexp, substitution) in &self.ending_quotes {
            text = regexp.replace_all(&text, substitution).to_string();
        }

//        for regexp in self.CONTRACTIONS {
//            text = regexp.replace_all(&text, " $1 $2 ").to_string();
//        }
        text.split_whitespace().map(|x| x.to_string()).collect()
    }
}
