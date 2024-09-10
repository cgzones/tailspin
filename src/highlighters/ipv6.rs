use std::borrow::Cow;

use crate::line_info::LineInfo;
use crate::types::Highlight;
use nu_ansi_term::Style;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};

static IPV6_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?:[0-9a-fA-F]{1,4}:{1,2}){3,}[0-9a-fA-F]{1,4}"#).expect("Invalid IPv6 regex pattern"));

pub struct Ipv6Highlighter {
    number: Style,
    letter: Style,
    separator: Style,
}

impl Ipv6Highlighter {
    pub const fn new(number: Style, letter: Style, separator: Style) -> Self {
        Self {
            number,
            letter,
            separator,
        }
    }
}

impl Highlight for Ipv6Highlighter {
    fn should_short_circuit(&self, line_info: &LineInfo) -> bool {
        line_info.colons < 4
    }

    fn only_apply_to_segments_not_already_highlighted(&self) -> bool {
        true
    }

    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        let mut b = [0; 2];

        IPV6_REGEX.replace_all(input, |caps: &Captures<'_>| {
            caps[0]
                .chars()
                .map(|c| match c {
                    '0'..='9' => self.number.paint(c.encode_utf8(&mut b) as &str).to_string(),
                    'a'..='f' | 'A'..='F' => self.letter.paint(c.encode_utf8(&mut b) as &str).to_string(),
                    ':' | '.' => self.separator.paint(c.encode_utf8(&mut b) as &str).to_string(),
                    _ => c.to_string(),
                })
                .collect::<String>()
        })
    }
}
