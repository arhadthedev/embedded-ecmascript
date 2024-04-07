//! Tokenizers of names in `.js` and `.mjs` files.
//!
//! Third party conditions
//! ======================
//!
//! This file cites and implements ECMA-262 14th edition also known as
//! ECMAScript 2023 (<https://262.ecma-international.org/14.0/>).
//!
//! Terminology and citations are provided under the following conditions listed
//! in section I Copyright & Software License:
//!
//! > Copyright Notice
//! >
//! > © 2023 Ecma International
//! >
//! > By obtaining and/or copying this work, you (the licensee) agree that you
//! > have read, understood, and will comply with the following terms
//! > and conditions.
//! >
//! > Permission under Ecma’s copyright to copy, modify, prepare derivative
//! > works of, and distribute this work, with or without modification, for any
//! > purpose and without fee or royalty is hereby granted, provided that you
//! > include the following on ALL copies of the work or portions thereof,
//! > including modifications:
//! >
//! > (i) The full text of this COPYRIGHT NOTICE AND COPYRIGHT LICENSE
//! > in a location viewable to users of the redistributed or derivative work.
//! >
//! > (ii) Any pre-existing intellectual property disclaimers, notices, or
//! > terms and conditions. If none exist, the Ecma alternative copyright notice
//! > should be included.
//! >
//! > (iii) Notice of any changes or modifications, through a copyright
//! > statement on the document such as “This document includes material copied
//! > from or derived from [title and URI of the Ecma document]. Copyright
//! > © Ecma International.”
//! >
//! > Disclaimers
//! >
//! > THIS WORK IS PROVIDED “AS IS,” AND COPYRIGHT HOLDERS MAKE NO
//! > REPRESENTATIONS OR WARRANTIES, EXPRESS OR IMPLIED, INCLUDING
//! > BUT NOT LIMITED TO, WARRANTIES OF MERCHANTABILITY OR FITNESS FOR ANY
//! > PARTICULAR PURPOSE OR THAT THE USE OF THE DOCUMENT WILL NOT INFRINGE ANY
//! > THIRD PARTY PATENTS, COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS.
//! >
//! > COPYRIGHT HOLDERS WILL NOT BE LIABLE FOR ANY DIRECT, INDIRECT, SPECIAL
//! > OR CONSEQUENTIAL DAMAGES ARISING OUT OF ANY USE OF THE DOCUMENT.
//! >
//! > The name and trademarks of copyright holders may NOT be used in
//! > advertising or publicity pertaining to the work without specific, written
//! > prior permission. Title to copyright in this work will at all times remain
//! > with copyright holders.

/// Try to match start of a string against `<ZWNJ>` entry of Table 34:
/// Format-Control Code Point Usage:
///
/// > | Code Point | Name                      | Abbreviation |
/// > |------------|---------------------------|--------------|
/// > | U+200C     | ZERO WIDTH NON-JOINER     | <ZWNJ>       |
///
/// Returns a tuple of an object created from the matched part and an unparsed
/// tail after the matched part.
///
/// Implements <https://262.ecma-international.org/14.0/#sec-unicode-format-control-characters>.
pub fn match_zwnj(text: &str) -> Option<((), &str)> {
    text.strip_prefix('\u{200C}').map(|tail| ((), tail))
}

/// Try to match start of a string against `<ZWJ>` entry of Table 34:
/// Format-Control Code Point Usage:
///
/// > | Code Point | Name                      | Abbreviation |
/// > |------------|---------------------------|--------------|
/// > | U+200D     | ZERO WIDTH JOINER         | <ZWJ>        |
///
/// Returns a tuple of an object created from the matched part and an unparsed
/// tail after the matched part.
///
/// Implements <https://262.ecma-international.org/14.0/#sec-unicode-format-control-characters>.
pub fn match_zwj(text: &str) -> Option<((), &str)> {
    text.strip_prefix('\u{200D}').map(|tail| ((), tail))
}

#[cfg(test)]
mod tests {
    use crate::_tokenizer::tests::{return_none, with_term};
    use rstest::rstest;
    use std::str::FromStr;

    /// A test case for a parser, creatable from a literal the parser
    /// is documented to process.
    ///
    /// The creation is performed in [`TerminalCase.from_str`] and invoked
    /// by the `#[values("\u{...}, ...)]` macro provided by rstest.
    struct TerminalCase {
        token: String,
        parser: fn(&str) -> Option<((), &str)>
    }

    struct CaseParameterError;

    impl FromStr for TerminalCase {
        type Err = CaseParameterError;

        fn from_str(text: &str) -> Result<Self, Self::Err> {
            let tested_parser = match text {
                "\u{200C}" => super::match_zwnj,
                "\u{200D}" => super::match_zwj,
                _ => return_none
            };
            Ok(Self {
                token: text.to_string(),
                parser: tested_parser
            })
        }
    }

    #[rstest]
    fn match_space(
        #[values(
            "\u{200C}", "\u{200D}"
        )]
        tested: TerminalCase,
        #[values("foo", " ")]
        separator: &str
    ) {
        with_term(tested.parser, tested.token.as_ref(), separator);
    }
}
