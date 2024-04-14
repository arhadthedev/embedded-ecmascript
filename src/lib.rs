//! Embed ECMAScript into desktop and automotive programs.
//!
//! This library works with ECMA-262 grammar definition, both for parsing and
//! execution of static&dynamic semantics. So here is a reminder on grammar
//! terminology used in the specification.
//!
//! Each grammar rule looks like `Production :: ProductionDefinition`. Each
//! production has an algorithm for each static and dynamic semantics.

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
    _tokenizer::names::match_reserved_word("");
    _tokenizer::names::match_identifier_part_char("");
    _tokenizer::names::match_ascii_letter("");
    _tokenizer::names::match_unicode_id_start("");
    _tokenizer::space::match_whitespace("");
    _tokenizer::space::match_line_terminator("");
    _tokenizer::space::match_line_terminator_sequence("");
    if let Some(
        (
            _tokenizer::punctuators::Punctuator::Other(_tokenizer::punctuators::OtherPunctuator::And),
            _
        )
    ) = _tokenizer::punctuators::match_punctuator("") {
        print!("");
    }
    _tokenizer::punctuators::match_div_punctuator("");
    _tokenizer::punctuators::match_right_brace_punctuator("");
}
