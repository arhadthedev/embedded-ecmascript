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

/// Try to match start of a string against `<ZWNBSP>` entry of Table 34:
/// Format-Control Code Point Usage:
///
/// > | Code Point | Name                      | Abbreviation |
/// > |------------|---------------------------|--------------|
/// > | U+FEFF     | ZERO WIDTH NO-BREAK SPACE | <ZWNBSP>     |
///
/// Returns a tuple of an object created from the matched part and an unparsed
/// tail after the matched part.
///
/// Implements <https://262.ecma-international.org/14.0/#sec-unicode-format-control-characters>.
pub fn match_zwnbsp(text: &str) -> Option<((), &str)> {
    text.strip_prefix('\u{FEFF}').map(|tail| ((), tail))
}
/// Try to match start of a string against `<TAB>` entry of Table 35:
/// White Space Code Points:
///
/// > | Code Point | Name                      | Abbreviation |
/// > |------------|---------------------------|--------------|
/// > | U+0009     | CHARACTER TABULATION      | <TAB>        |
///
/// Returns a tuple of an object created from the matched part and an unparsed
/// tail after the matched part.
///
/// Implements <https://262.ecma-international.org/14.0/#sec-white-space>.
pub fn match_tab(text: &str) -> Option<((), &str)> {
    text.strip_prefix('\u{0009}').map(|tail| ((), tail))
}

/// Try to match start of a string against `<VT>` entry of Table 35:
/// White Space Code Points:
///
/// > | Code Point | Name                      | Abbreviation |
/// > |------------|---------------------------|--------------|
/// > | U+000B     | LINE TABULATION           | <VT>         |
///
/// Returns a tuple of an object created from the matched part and an unparsed
/// tail after the matched part.
///
/// Implements <https://262.ecma-international.org/14.0/#sec-white-space>.
pub fn match_vt(text: &str) -> Option<((), &str)> {
    text.strip_prefix('\u{000B}').map(|tail| ((), tail))
}

/// Try to match start of a string against `<FF>` entry of Table 35:
/// White Space Code Points:
///
/// > | Code Point | Name                      | Abbreviation |
/// > |------------|---------------------------|--------------|
/// > | U+000C     | FORM FEED (FF)            | <FF>         |
///
/// Returns a tuple of an object created from the matched part and an unparsed
/// tail after the matched part.
///
/// Implements <https://262.ecma-international.org/14.0/#sec-white-space>.
pub fn match_ff(text: &str) -> Option<((), &str)> {
    text.strip_prefix('\u{000C}').map(|tail| ((), tail))
}

/// Try to match start of a string against `<USP>` entry of Table 35:
/// White Space Code Points:
///
/// > | Code Point | Name                      | Abbreviation |
/// > |------------|---------------------------|--------------|
/// > | any code point in general category     | <USP>        |
/// > | `Space_Separator`                      |              | 
///
/// `Space_Separator` (Zs) contains the following:
///
/// - U+0020 SPACE
/// - U+00A0 NO-BREAK SPACE
/// - U+1680 OHGRAM SPACE MARK
/// - U+2000 EN QUAD
/// - U+2001 EM QUAD
/// - U+2002 EN SPACE
/// - U+2003 EM SPACE
/// - U+2004 THREE-PER-EM SPACE
/// - U+2005 FOUR-PER-EM SPACE
/// - U+2006 SIX-PER-EM SPACE
/// - U+2007 FIGURE SPACE
/// - U+2008 PUNCTUATION SPACE
/// - U+2009 THIN SPACE
/// - U+200A HAIR SPACE
/// - U+202F NARROW NO-BREAK SPACE
/// - U+205F MEDIUM MATHEMATICAL SPACE
/// - U+3000 IDEOGRAPHIC SPACE
///
/// Returns a tuple of an object created from the matched part and an unparsed
/// tail after the matched part.
///
/// Implements <https://262.ecma-international.org/14.0/#sec-white-space>.
pub fn match_usp(text: &str) -> Option<((), &str)> {
    let rest = text.strip_prefix('\u{0020}') 
        .or_else(|| text.strip_prefix('\u{00A0}'))
        .or_else(|| text.strip_prefix('\u{1680}'))
        .or_else(|| text.strip_prefix('\u{2000}'))
        .or_else(|| text.strip_prefix('\u{2001}'))
        .or_else(|| text.strip_prefix('\u{2002}'))
        .or_else(|| text.strip_prefix('\u{2003}'))
        .or_else(|| text.strip_prefix('\u{2004}'))
        .or_else(|| text.strip_prefix('\u{2005}'))
        .or_else(|| text.strip_prefix('\u{2006}'))
        .or_else(|| text.strip_prefix('\u{2007}'))
        .or_else(|| text.strip_prefix('\u{2008}'))
        .or_else(|| text.strip_prefix('\u{2009}'))
        .or_else(|| text.strip_prefix('\u{200A}'))
        .or_else(|| text.strip_prefix('\u{202F}'))
        .or_else(|| text.strip_prefix('\u{205F}'))
        .or_else(|| text.strip_prefix('\u{3000}'));
    rest.map(|tail| ((), tail))
}

/// Try to match start of a string against `WhiteSpace` production:
///
/// ```plain
/// WhiteSpace ::
///     <TAB>
///     <VT>
///     <FF>
///     <ZWNBSP>
///     <USP>
/// ```
///
/// Implements <https://262.ecma-international.org/14.0/#prod-WhiteSpace>.
pub fn match_whitespace(text: &str) -> Option<((), &str)> {
    match_tab(text) 
        .or_else(|| match_vt(text))
        .or_else(|| match_ff(text))
        .or_else(|| match_zwnbsp(text))
        .or_else(|| match_usp(text))
}

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
    use crate::_tokenizer::tests::{assert_match_tail, generate_cases, TerminalCase};
    use rstest::rstest;

    #[rstest]
    fn match_space(
        #[values(
            "\u{FEFF}", "\t", "\u{000B}", "\u{000C}",
            "\u{0020}", "\u{00A0}", "\u{1680}", "\u{2000}", "\u{2001}",
            "\u{2002}", "\u{2003}", "\u{2004}", "\u{2005}", "\u{2006}",
            "\u{2007}", "\u{2008}", "\u{2009}", "\u{200A}", "\u{202F}",
            "\u{205F}", "\u{3000}", "\u{000A}", "\u{000D}", "\u{2028}",
            "\u{2029}"
        )]
        tested: TerminalCase,
        #[values("foo", " ")]
        separator: String
    ) {
        if separator == " " {
            return;
        }
        for case in generate_cases(tested.terminal, separator.clone()) {
            assert_match_tail((tested.parser)(&case.input), &case.expected_tail);
        }
    }

    #[rstest]
    fn match_whitespace(
        #[values(
            "\u{0020}", "\u{00A0}", "\u{1680}", "\u{2000}", "\u{2001}",
            "\u{2002}", "\u{2003}", "\u{2004}", "\u{2005}", "\u{2006}",
            "\u{2007}", "\u{2008}", "\u{2009}", "\u{200A}", "\u{202F}",
            "\u{205F}", "\u{3000}"
        )]
        tested: TerminalCase,
        #[values("foo", " ")]
        separator: String
    ) {
        if separator == " " {
            return;
        }
        for case in generate_cases(tested.terminal, separator.clone()) {
            assert_match_tail((tested.parser)(&case.input), &case.expected_tail);
            assert_match_tail(
                super::match_whitespace(&case.input),
                &case.expected_tail
            )
        }
    }

    #[rstest]
    fn match_line_terminator(
        #[values(
            "\u{000A}", "\u{000D}", "\u{2028}", "\u{2029}"
        )]
        tested: TerminalCase,
        #[values("foo", " ")]
        separator: String
    ) {
        // This test is a subset of match_space used to separately test
        // match_line_terminator and match_line_terminator_sequence. Thus,
        // tested.parser is touched in match_space but deliberately unused here.
        for case in generate_cases(tested.terminal, separator) {
            assert_match_tail(
                super::match_line_terminator(&case.input),
                &case.expected_tail
            );
            assert_match_tail(
                super::match_line_terminator_sequence(&case.input),
                &case.expected_tail
            );
        }
    }

    #[rstest]
    fn match_line_terminator_sequence_crlf(
        #[values("foo", " ")]
        separator: String
    ) {
        for case in generate_cases("\r\n".to_string(), separator) {
            assert_match_tail(
                super::match_line_terminator_sequence(&case.input),
                &case.expected_tail
            );
        }
    }
}
