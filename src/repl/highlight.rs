use nu_ansi_term::Style;
use reedline::Highlighter;
use reedline::StyledText;

use crate::parser::*;

impl Highlighter for Localization {
    fn highlight(&self, line: &str, _pos: usize) -> StyledText {
        let mut styled_text = StyledText::new();
        match self.parse_highlight(line) {
            Ok(pairs) => {
                for (text, style) in pairs.into_iter() {
                    styled_text.push((style.into(), text));
                }
                styled_text
            }
            Err(_) => {
                styled_text.push((Style::new(), line.to_string()));
                styled_text
            }
        }
    }
}
