use std::fmt;

use regex::Regex;

#[derive(Debug, Clone, Copy)]
pub enum HeadDirection {
    Right,
    Left,
    None,
}
impl fmt::Display for HeadDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                HeadDirection::Right => ">",
                HeadDirection::Left => "<",
                HeadDirection::None => "-",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct MDTRule {
    pub current_state: String,
    pub read_symbol: char,
    pub next_state: String,
    pub write_symbol: char,
    pub direction: HeadDirection,
}

impl fmt::Display for MDTRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({},{},{},{},{})",
            make_special_chars_printable(&self.current_state),
            make_special_chars_printable(&self.read_symbol.to_string()),
            make_special_chars_printable(&self.next_state),
            make_special_chars_printable(&self.write_symbol.to_string()),
            self.direction
        )
    }
}

fn make_special_chars_printable(s: &str) -> String {
    let re = Regex::new(r"(?P<c>[\\<\\>\\^\\.\\,\[\]\{\}\(\)\\])").unwrap();
    re.replace(&s.replace(" ", "-"), "\\${c}").to_string()
}
