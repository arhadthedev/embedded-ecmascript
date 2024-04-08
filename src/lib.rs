//! Embed ECMAScript into desktop and automotive programs.

mod _tokenizer;

use std::cmp::{Eq, PartialEq};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result};
use std::ops::Range;

/// An error message that can be attributed to a certain piece of source code.
#[derive(Debug, Eq, PartialEq)]
pub struct SourceCodeError {
    /// A non-inclusive, zero-based range of source code UTF-8 characters.
    ///
    /// To convert offsets into line and column numbers, use
    /// `calculate_location`.
    pub location: Range<u64>,

    /// An arbitrary text describing what happened.
    ///
    /// No need to prepend `error: ` in front of the message.
    pub message: String,
}

impl Error for SourceCodeError {
}

impl Display for SourceCodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "error in characters #{}-#{}: {}",
            self.location.start + 1,
            self.location.end,
            &self.message
        )
    }
}

pub fn dummy() {
    _tokenizer::names::match_zwnj("");
    _tokenizer::names::match_zwj("");
    _tokenizer::space::match_whitespace("");
    _tokenizer::space::match_line_terminator("");
    _tokenizer::space::match_line_terminator_sequence("");
    _tokenizer::punctuators::match_other_punctuator("");
    _tokenizer::punctuators::match_div_punctuator("");
    _tokenizer::punctuators::match_right_brace_punctuator("");
}
