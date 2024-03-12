use crate::{parser::*, session::Session};
use reedline::{ValidationResult, Validator};

impl Validator for Localization {
    fn validate(&self, line: &str) -> ValidationResult {
        let res = self.parse_input(line);
        match res {
            Ok(_) => ValidationResult::Complete,
            Err(_) => ValidationResult::Incomplete,
        }
    }
}

impl Validator for Session {
    fn validate(&self, line: &str) -> ValidationResult {
        let res = self.locale.parse_input_with(line, self);
        match res {
            Ok(_) => ValidationResult::Complete,
            Err(_) => ValidationResult::Incomplete,
        }
    }
}
