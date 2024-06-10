//! Tokenizers of spaces in `.js` and `.mjs` files.
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

/// Try to match start of a string against `<LF>` entry of Table 36:
/// Line Terminator Code Points:
///
/// > | Code Point | Name                      | Abbreviation |
/// > |------------|---------------------------|--------------|
/// > | U+000A     | LINE FEED (LF)            | <LF>         |
///
/// Returns a tuple of an object created from the matched part and an unparsed
/// tail after the matched part.
///
/// Implements <https://262.ecma-international.org/14.0/#table-line-terminator-code-points>.
pub fn match_lf(text: &str) -> Option<((), &str)> {
    text.strip_prefix('\u{000A}').map(|tail| ((), tail))
}

/// Try to match start of a string against `<CR>` entry of Table 36:
/// Line Terminator Code Points:
///
/// > | Code Point | Name                      | Abbreviation |
/// > |------------|---------------------------|--------------|
/// > | U+000D     | CARRIAGE RETURN (CR)      | <CR>         |
///
/// Returns a tuple of an object created from the matched part and an unparsed
/// tail after the matched part.
///
/// Implements <https://262.ecma-international.org/14.0/#table-line-terminator-code-points>.
pub fn match_cr(text: &str) -> Option<((), &str)> {
    text.strip_prefix('\u{000D}').map(|tail| ((), tail))
}

/// Try to match start of a string against `<LS>` entry of Table 36:
/// Line Terminator Code Points:
///
/// > | Code Point | Name                      | Abbreviation |
/// > |------------|---------------------------|--------------|
/// > | U+2028     | LINE SEPARATOR            | <LS>         |
///
/// Returns a tuple of an object created from the matched part and an unparsed
/// tail after the matched part.
///
/// Implements <https://262.ecma-international.org/14.0/#table-line-terminator-code-points>.
pub fn match_ls(text: &str) -> Option<((), &str)> {
    text.strip_prefix('\u{2028}').map(|tail| ((), tail))
}

/// Try to match start of a string against `<PS>` entry of Table 36:
/// Line Terminator Code Points:
///
/// > | Code Point | Name                      | Abbreviation |
/// > |------------|---------------------------|--------------|
/// > | U+2029     | PARAGRAPH SEPARATOR       | <PS>         |
///
/// Returns a tuple of an object created from the matched part and an unparsed
/// tail after the matched part.
///
/// Implements <https://262.ecma-international.org/14.0/#table-line-terminator-code-points>.
pub fn match_ps(text: &str) -> Option<((), &str)> {
    text.strip_prefix('\u{2029}').map(|tail| ((), tail))
}

/// Try to match start of a string against `LineTerminator` production:
///
/// ```plain
/// LineTerminator ::
///     <LF>
///     <CR>
///     <LS>
///     <PS>
/// ```
///
/// Implements <https://262.ecma-international.org/14.0/#prod-LineTerminator>.
pub fn match_line_terminator(text: &str) -> Option<((), &str)> {
    match_lf(text) 
        .or_else(|| match_cr(text))
        .or_else(|| match_ls(text))
        .or_else(|| match_ps(text))
}

fn match_crlf(text: &str) -> Option<((), &str)> {
    text.strip_prefix("\u{000D}\u{000A}").map(|tail| ((), tail))
}

/// Try to match start of a string against `LineTerminatorSequence` production:
///
/// ```plain
/// LineTerminatorSequence ::
///     <LF>
///     <CR> [lookahead ≠ <LF>]
///     <LS>
///     <PS>
//      <CR> <LF>
/// ```
///
/// Implements <https://262.ecma-international.org/14.0/#prod-LineTerminatorSequence>.
pub fn match_line_terminator_sequence(text: &str) -> Option<((), &str)> {
    match_lf(text)
        .or_else(|| match_crlf(text)) // Try greedy match_crlf before match_cr
        .or_else(|| match_cr(text))
        .or_else(|| match_ls(text))
        .or_else(|| match_ps(text))
}

#[cfg(test)]
mod tests {
    use crate::_tokenizer::tests::{generate_cases, TerminalCase, unwrap_tail};
    use rstest::rstest;

    #[rstest]
    fn match_line_terminator(
        #[values(
            "\u{000A}", "\u{000D}", "\u{2028}", "\u{2029}"
        )]
        tested: TerminalCase,
        #[values("foo", " ")]
        separator: &str
    ) {
        // This test is a subset of match_space used to separately test
        // match_line_terminator and match_line_terminator_sequence. Thus,
        // tested.parser is touched in match_space but deliberately unused here.
        for case in generate_cases(&tested.terminal, separator) {
            assert!(
                unwrap_tail(super::match_line_terminator(&case.input)) ==
                case.expected_tail
            );
            assert!(
                unwrap_tail(super::match_line_terminator_sequence(&case.input)) ==
                case.expected_tail
            );
        }
    }

    #[rstest]
    fn match_line_terminator_sequence_crlf(
        #[values("foo", " ")]
        separator: &str
    ) {
        for case in generate_cases("\r\n", separator) {
            assert!(
                unwrap_tail(super::match_line_terminator_sequence(&case.input)) ==
                case.expected_tail
            );
        }
    }
}
