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
    _tokenizer::space::match_zwnj("");
    _tokenizer::space::match_zwj("");
    _tokenizer::space::match_whitespace("");
    _tokenizer::space::match_line_terminator("");
}

#[cfg(test)]
mod tests {
    pub fn with_token(tok: &str, sep: &str, parser: fn(&str) -> Option<((), &str)>) {
        // Empty strings do not match
        assert_eq!(parser(""), None);

        // Skip false match when the function recognizes a separator.
        if parser(sep) != Some(((), "")) {
            // Non-matching strings do not match
            assert_eq!(parser(sep), None);

            // Catch arbitrary (regex-like) match of a necessary symbol
            assert_eq!(parser(format!("{sep}{tok}").as_ref()), None);
        }

        // Test EOF match
        assert_eq!(parser(tok), Some(((), "")));

        // Test non-EOF match
        assert_eq!(
            parser(format!("{tok}{sep}").as_ref()),
            Some(((), sep))
        );

        // Test repetitions
        assert_eq!(
            parser(format!("{tok}{tok}").as_ref()),
            Some(((), tok))
        );

        // Test separated repetitions
        assert_eq!(
            parser(format!("{tok}{sep}{tok}").as_ref()),
            Some(((), format!("{sep}{tok}").as_ref()))
        );
    }
}
