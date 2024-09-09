use crate::line_info::LineInfo;
use crate::types::Highlight;
use nu_ansi_term::Style;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};

static DATE_REGEX_1: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?P<year>20\d{2})(?P<separator1>/)(?P<month>(?:0[1-9]|1[0-2]))(?P<separator2>/)(?P<day>(?:0[1-9]|[12][0-9]|3[01]))")
        .expect("Invalid regex pattern")
});

static DATE_REGEX_2: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?P<day>(?:0[1-9]|[12][0-9]|3[01]))(?P<separator1>/)(?P<month>(?:Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec))(?P<separator2>/)(?P<year>20\d{2})")
    .expect("Regex is valid")
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

    fn apply(&self, input: &str) -> String {
        let step1 = DATE_REGEX_1.replace_all(input, |caps: &Captures<'_>| {
            let year = &caps["year"];
            let month = &caps["month"];
            let day = &caps["day"];
            let separator1 = &caps["separator1"];
            let separator2 = &caps["separator2"];

            format!(
                "{}{}{}{}{}",
                self.number.paint(year),
                self.separator.paint(separator1),
                self.number.paint(month),
                self.separator.paint(separator2),
                self.number.paint(day)
            )
        });

        let step2 = DATE_REGEX_2.replace_all(&step1, |caps: &Captures<'_>| {
            let day = &caps["day"];
            let month = &caps["month"];
            let year = &caps["year"];
            let separator1 = &caps["separator1"];
            let separator2 = &caps["separator2"];

            format!(
                "{}{}{}{}{}",
                self.number.paint(day),
                self.separator.paint(separator1),
                self.number.paint(month),
                self.separator.paint(separator2),
                self.number.paint(year)
            )
        });

        step2.to_string()
    }
}
