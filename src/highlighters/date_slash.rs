use std::borrow::Cow;

use crate::line_info::LineInfo;
use crate::types::Highlight;
use nu_ansi_term::Style;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};

static DATE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?ix)
                   \b(?P<yearA>20\d{2})(?P<separatorA1>/)(?P<monthA>(?:0[1-9]|1[0-2]))(?P<separatorA2>/)(?P<dayA>(?:0[1-9]|[12][0-9]|3[01]))\b
                   |
                   \b(?P<dayB>(?:0[1-9]|[12][0-9]|3[01]))(?P<separatorB1>/)(?P<monthB>(?:Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec))(?P<separatorB2>/)(?P<yearB>20\d{2})\b")
    .expect("Invalid regex pattern")
});

pub struct DateSlashHighlighter {
    number: Style,
    separator: Style,
}

impl DateSlashHighlighter {
    pub const fn new(number: Style, separator: Style) -> Self {
        Self { number, separator }
    }
}

impl Highlight for DateSlashHighlighter {
    fn should_short_circuit(&self, line_info: &LineInfo) -> bool {
        line_info.slashes < 2
    }

    fn only_apply_to_segments_not_already_highlighted(&self) -> bool {
        true
    }

    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        DATE_REGEX.replace_all(input, |caps: &Captures<'_>| {
            let year = caps
                .name("yearA")
                .or_else(|| caps.name("yearB"))
                .expect("Either regex branch must have been matched")
                .as_str();
            let month = caps
                .name("monthA")
                .or_else(|| caps.name("monthB"))
                .expect("Either regex branch must have been matched")
                .as_str();
            let day = caps
                .name("dayA")
                .or_else(|| caps.name("dayB"))
                .expect("Either regex branch must have been matched")
                .as_str();
            let separator1 = caps
                .name("separatorA1")
                .or_else(|| caps.name("separatorB1"))
                .expect("Either regex branch must have been matched")
                .as_str();
            let separator2 = caps
                .name("separatorA2")
                .or_else(|| caps.name("separatorB2"))
                .expect("Either regex branch must have been matched")
                .as_str();

            format!(
                "{}{}{}{}{}",
                self.number.paint(year),
                self.separator.paint(separator1),
                self.number.paint(month),
                self.separator.paint(separator2),
                self.number.paint(day)
            )
        })
    }
}
