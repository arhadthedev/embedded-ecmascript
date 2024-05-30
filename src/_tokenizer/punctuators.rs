//! Tokenizers of punctuation marks in `.js` and `.mjs` files.
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

use super::numeric::match_decimal_digit;

pub enum Punctuator {
    OptionalChaining,
    Other(OtherPunctuator)
}

/// Try to match start of a string against `Punctuator` production:
///
/// ```plain
/// Punctuator ::
///     OptionalChainingPunctuator
///     OtherPunctuator
/// ```
///
/// Implements <https://262.ecma-international.org/14.0/#prod-Punctuator>.
pub fn match_punctuator(text: &str) -> Option<(Punctuator, &str)> {
    match_optional_chaining_punctuator(text).map(
        |((), tail)| (Punctuator::OptionalChaining, tail)
    )
    .or_else(|| match_other_punctuator(text).map(
        |(parsed, tail)| (Punctuator::Other(parsed), tail)
    ))
}

/// Try to match start of a string against `OptionalChainingPunctuator` production:
///
/// ```plain
/// OptionalChainingPunctuator ::
///     `?.` [lookahead ∉ `DecimalDigit`]
/// ```
///
/// Implements <https://262.ecma-international.org/14.0/#prod-OptionalChainingPunctuator>.
pub fn match_optional_chaining_punctuator(text: &str) -> Option<((), &str)> {
    text.strip_prefix("?.")
        .filter(|tail| match_decimal_digit(tail).is_none())
        .map(|tail| ((), tail))
}

#[derive(Debug, PartialEq, Eq)]
pub enum OtherPunctuator {
    Addition,
    AdditionAssignment,
    And,
    AndAssignment,
    Assignment,
    BitAnd,
    BitAndAssignment,
    BitNot,
    BitOr,
    BitOrAssignment,
    BitXor,
    BitXorAssignment,
    ClosingBracket,
    ClosingParenthesis,
    Colon,
    Comma,
    Decrement,
    Dot,
    Ellipsis,
    Exponentiation,
    ExponentiationAssignment,
    FunctionArrow,
    Increment,
    LeftShift,
    LeftShiftAssignment,
    Less,
    LessOrEqual,
    LooseEquality,
    LooseInequality,
    Modulo,
    ModuloAssignment,
    More,
    MoreOrEqual,
    Multiplication,
    MultiplicationAssignment,
    Not,
    NullishCoalescence,
    NullishCoalescenceAssignment,
    OpeningBrace,
    OpeningBracket,
    OpeningParenthesis,
    Or,
    OrAssignment,
    QuestionMark,
    RightShift,
    RightShiftAssignment,
    Semicolon,
    StrictEquality,
    StrictInequality,
    Subtraction,
    SubtractionAssignment,
    UnsignedRightShift,
    UnsignedRightShiftAssignment,
}

/// Try to match start of a string against `OtherPunctuator` production:
///
/// ```plain
/// OtherPunctuator ::
///     `{` `(` `)` `[` `]` `.` `...` `;` `,` `<` `>` `<=` `>=` `==` `!=` `===`
///     `!==` `+` `-` `*` `%` `**` `++` `--` `<<` `>>` `>>>` `&` `|` `^` `!` `~`
///     `&&` `||` `??` `?` `:` `=` `+=` `-=` `*=` `%=` `**=` `<<=` `>>=` `>>>=`
///     `&=` `|=` `^=` `&&=` `||=` `??=` `=>`
/// ```
///
/// Implements <https://262.ecma-international.org/14.0/#prod-OtherPunctuator>.
pub fn match_other_punctuator(text: &str) -> Option<(OtherPunctuator, &str)> {
    // Note: if  one punctuator is the same as a start of other punctuator
    // (like += and +), check the longer one first to not prematurely bail out
    // on the shorter one leaving an undermatched tail.
    text
        .strip_prefix('{').map(
            |tail| (OtherPunctuator::OpeningBrace, tail)
        )
        .or_else(|| text.strip_prefix('(').map(
            |tail| (OtherPunctuator::OpeningParenthesis, tail)
        ))
        .or_else(|| text.strip_prefix(')').map(
            |tail| (OtherPunctuator::ClosingParenthesis, tail)
        ))
        .or_else(|| text.strip_prefix('[').map(
            |tail| (OtherPunctuator::OpeningBracket, tail)
        ))
        .or_else(|| text.strip_prefix(']').map(
            |tail| (OtherPunctuator::ClosingBracket, tail)
        ))
        .or_else(|| text.strip_prefix("...").map(
            |tail| (OtherPunctuator::Ellipsis, tail)
        ))
        .or_else(|| text.strip_prefix('.').map(
            |tail| (OtherPunctuator::Dot, tail)
        ))
        .or_else(|| text.strip_prefix(';').map(
            |tail| (OtherPunctuator::Semicolon, tail)
        ))
        .or_else(|| text.strip_prefix(',').map(
            |tail| (OtherPunctuator::Comma, tail)
        ))
        .or_else(|| text.strip_prefix("===").map(
            |tail| (OtherPunctuator::StrictEquality, tail)
        ))
        .or_else(|| text.strip_prefix("=>").map(
            |tail| (OtherPunctuator::FunctionArrow, tail)
        ))
        .or_else(|| text.strip_prefix("==").map(
            |tail| (OtherPunctuator::LooseEquality, tail)
        ))
        .or_else(|| text.strip_prefix('=').map(
            |tail| (OtherPunctuator::Assignment, tail)
        ))
        .or_else(|| text.strip_prefix("!==").map(
            |tail| (OtherPunctuator::StrictInequality, tail)
        ))
        .or_else(|| text.strip_prefix("!=").map(
            |tail| (OtherPunctuator::LooseInequality, tail)
        ))
        .or_else(|| text.strip_prefix('!').map(
            |tail| (OtherPunctuator::Not, tail)
        ))
        .or_else(|| text.strip_prefix("++").map(
            |tail| (OtherPunctuator::Increment, tail)
        ))
        .or_else(|| text.strip_prefix("+=").map(
            |tail| (OtherPunctuator::AdditionAssignment, tail)
        ))
        .or_else(|| text.strip_prefix('+').map(
            |tail| (OtherPunctuator::Addition, tail)
        ))
        .or_else(|| text.strip_prefix("--").map(
            |tail| (OtherPunctuator::Decrement, tail)
        ))
        .or_else(|| text.strip_prefix("-=").map(
            |tail| (OtherPunctuator::SubtractionAssignment, tail)
        ))
        .or_else(|| text.strip_prefix('-').map(
            |tail| (OtherPunctuator::Subtraction, tail)
        ))
        .or_else(|| text.strip_prefix("%=").map(
            |tail| (OtherPunctuator::ModuloAssignment, tail)
        ))
        .or_else(|| text.strip_prefix('%').map(
            |tail| (OtherPunctuator::Modulo, tail)
        ))
        .or_else(|| text.strip_prefix("**=").map(
            |tail| (OtherPunctuator::ExponentiationAssignment, tail)
        ))
        .or_else(|| text.strip_prefix("**").map(
            |tail| (OtherPunctuator::Exponentiation, tail)
        ))
        .or_else(|| text.strip_prefix("*=").map(
            |tail| (OtherPunctuator::MultiplicationAssignment, tail)
        ))
        .or_else(|| text.strip_prefix('*').map(
            |tail| (OtherPunctuator::Multiplication, tail)
        ))
        .or_else(|| text.strip_prefix(">>>=").map(
            |tail| (OtherPunctuator::UnsignedRightShiftAssignment, tail)
        ))
        .or_else(|| text.strip_prefix(">>>").map(
            |tail| (OtherPunctuator::UnsignedRightShift, tail)
        ))
        .or_else(|| text.strip_prefix(">>=").map(
            |tail| (OtherPunctuator::RightShiftAssignment, tail)
        ))
        .or_else(|| text.strip_prefix(">>").map(
            |tail| (OtherPunctuator::RightShift, tail)
        ))
        .or_else(|| text.strip_prefix(">=").map(
            |tail| (OtherPunctuator::MoreOrEqual, tail)
        ))
        .or_else(|| text.strip_prefix('>').map(
            |tail| (OtherPunctuator::More, tail)
        ))
        .or_else(|| text.strip_prefix("<<=").map(
            |tail| (OtherPunctuator::LeftShiftAssignment, tail)
        ))
        .or_else(|| text.strip_prefix("<<").map(
            |tail| (OtherPunctuator::LeftShift, tail)
        ))
        .or_else(|| text.strip_prefix("<=").map(
            |tail| (OtherPunctuator::LessOrEqual, tail)
        ))
        .or_else(|| text.strip_prefix('<').map(
            |tail| (OtherPunctuator::Less, tail)
        ))
        .or_else(|| text.strip_prefix("&&=").map(
            |tail| (OtherPunctuator::AndAssignment, tail)
        ))
        .or_else(|| text.strip_prefix("&&").map(
            |tail| (OtherPunctuator::And, tail)
        ))
        .or_else(|| text.strip_prefix("&=").map(
            |tail| (OtherPunctuator::BitAndAssignment, tail)
        ))
        .or_else(|| text.strip_prefix('&').map(
            |tail| (OtherPunctuator::BitAnd, tail)
        ))
        .or_else(|| text.strip_prefix("||=").map(
            |tail| (OtherPunctuator::OrAssignment, tail)
        ))
        .or_else(|| text.strip_prefix("||").map(
            |tail| (OtherPunctuator::Or, tail)
        ))
        .or_else(|| text.strip_prefix("|=").map(
            |tail| (OtherPunctuator::BitOrAssignment, tail)
        ))
        .or_else(|| text.strip_prefix('|').map(
            |tail| (OtherPunctuator::BitOr, tail)
        ))
        .or_else(|| text.strip_prefix("^=").map(
            |tail| (OtherPunctuator::BitXorAssignment, tail)
        ))
        .or_else(|| text.strip_prefix('^').map(
            |tail| (OtherPunctuator::BitXor, tail)
        ))
        .or_else(|| text.strip_prefix('~').map(
            |tail| (OtherPunctuator::BitNot, tail)
        ))
        .or_else(|| text.strip_prefix("??=").map(
            |tail| (OtherPunctuator::NullishCoalescenceAssignment, tail)
        ))
        .or_else(|| text.strip_prefix("??").map(
            |tail| (OtherPunctuator::NullishCoalescence, tail)
        ))
        .or_else(|| text.strip_prefix('?').map(
            |tail| (OtherPunctuator::QuestionMark, tail)
        ))
        .or_else(|| text.strip_prefix(':').map(
            |tail| (OtherPunctuator::Colon, tail)
        ))
}

#[derive(Debug, PartialEq, Eq)]
pub enum DivPunctuator {
    DivisionAssignment,
    Division,
}

/// Try to match start of a string against `DivPunctuator` production:
///
/// ```plain
/// DivPunctuator ::
///     `/`
///     `/=`
/// ```
///
/// Implements <https://262.ecma-international.org/14.0/#prod-DivPunctuator>.
pub fn match_div_punctuator(text: &str) -> Option<(DivPunctuator, &str)> {
    text
        .strip_prefix("/=").map(
            |tail| (DivPunctuator::DivisionAssignment, tail)
        )
        .or_else(|| text.strip_prefix('/').map(
            |tail| (DivPunctuator::Division, tail)
        ))
}

/// Try to match start of a string against `RightBracePunctuator` production:
///
/// ```plain
/// RightBracePunctuator ::
///     `}`
/// ```
///
/// Implements <https://262.ecma-international.org/14.0/#prod-RightBracePunctuator>.
pub fn match_right_brace_punctuator(text: &str) -> Option<((), &str)> {
    text.strip_prefix('}').map(|tail| ((), tail))
}

#[cfg(test)]
mod tests {
    use crate::_tokenizer::tests::{generate_cases, TerminalCase, unwrap_tail};
    use rstest::rstest;

    /// Remove cases with token repetitions ("{token}{token}") that give
    /// unexpected results.
    fn is_double(term: &str) -> bool {
        // Operators that have both single- and double-symbol versions can
        // either:
        [
            // consume short operator repetition as a single long version
            // thus giving, for example, `Some((Increment, "")` instead of
            // lazy `Some(Plus, "+")` expected by tests
            "++", "--", "??", "**", "<<", "||", "&&",
            // or lead to an ambiguous operator syntax error giving, for
            // example, `None` for ">>>>" instead of `Some((RightShift, ">>"))`.
            ">>", ">>>>", "==", "====", "======"
        ].contains(&term)
    }

    #[rstest]
    fn match_punctuators(
        #[values(
            "?.",

            "}", "/", "/=",

            "{", "(", ")", "[", "]", ".", "...", ";", ",", "<", ">", "<=", ">=",
            "==", "!=", "===", "!==", "+", "-", "*", "%", "**", "++", "--",
            "<<", ">>", ">>>", "&", "|", "^", "!", "~", "&&", "||", "??", "?",
            ":", "=", "+=", "-=", "*=", "%=", "**=", "<<=", ">>=", ">>>=", "&=",
            "|=", "^=", "&&=", "||=", "??=", "=>",
        )]
        tested: TerminalCase,
        #[values("foo", " ")]
        separator: &str
    ) {
        let all = generate_cases(&tested.terminal, separator);
        let safe_cases = all.iter().filter(|case| !is_double(&case.input));
        for case in safe_cases {
            assert_eq!((tested.parser)(&case.input), case.expected_tail);
        }
    }

    #[rstest]
    fn match_punctuator(
        #[values(
            "?.",

            "{", "(", ")", "[", "]", ".", "...", ";", ",", "<", ">", "<=", ">=",
            "==", "!=", "===", "!==", "+", "-", "*", "%", "**", "++", "--",
            "<<", ">>", ">>>", "&", "|", "^", "!", "~", "&&", "||", "??", "?",
            ":", "=", "+=", "-=", "*=", "%=", "**=", "<<=", ">>=", ">>>=", "&=",
            "|=", "^=", "&&=", "||=", "??=", "=>",
        )]
        tested: TerminalCase,
        #[values("foo", " ")]
        separator: &str
    ) {
        let all = generate_cases(&tested.terminal, separator);
        let safe_cases = all.iter().filter(|case| !is_double(&case.input));
        for case in safe_cases {
            assert_eq!(
                unwrap_tail(super::match_punctuator(&case.input)),
                case.expected_tail
            );
        }
    }

    #[rstest]
    fn match_optional_chaining_punctuator() {
        // Check the [lookahead ∉ `DecimalDigit`] rule
        assert_eq!(
            super::match_optional_chaining_punctuator("?.9"),
            None
        );

        // ... and its ignorance of hex digits
        assert_eq!(
            super::match_optional_chaining_punctuator("?.A"),
            Some(((), "A"))
        );
        assert_eq!(
            super::match_optional_chaining_punctuator("?.a"),
            Some(((), "a"))
        );
    }

    #[rstest]
    fn match_parsed_div() {
        assert_eq!(
            super::match_div_punctuator("/"),
            Some((super::DivPunctuator::Division, ""))
        );
        assert_eq!(
            super::match_div_punctuator("/="),
            Some((super::DivPunctuator::DivisionAssignment, ""))
        );
    }

    #[rstest]
    fn match_parsed_other() {
        assert_eq!(
            super::match_other_punctuator("+"),
            Some((super::OtherPunctuator::Addition, ""))
        );
        assert_eq!(
            super::match_other_punctuator("+="),
            Some((super::OtherPunctuator::AdditionAssignment, ""))
        );
        assert_eq!(
            super::match_other_punctuator("&&"),
            Some((super::OtherPunctuator::And, ""))
        );
        assert_eq!(
            super::match_other_punctuator("&&="),
            Some((super::OtherPunctuator::AndAssignment, ""))
        );
        assert_eq!(
            super::match_other_punctuator("="),
            Some((super::OtherPunctuator::Assignment, ""))
        );
        assert_eq!(
            super::match_other_punctuator("&"),
            Some((super::OtherPunctuator::BitAnd, ""))
        );
        assert_eq!(
            super::match_other_punctuator("&="),
            Some((super::OtherPunctuator::BitAndAssignment, ""))
        );
        assert_eq!(
            super::match_other_punctuator("~"),
            Some((super::OtherPunctuator::BitNot, ""))
        );
        assert_eq!(
            super::match_other_punctuator("|"),
            Some((super::OtherPunctuator::BitOr, ""))
        );
        assert_eq!(
            super::match_other_punctuator("|="),
            Some((super::OtherPunctuator::BitOrAssignment, ""))
        );
        assert_eq!(
            super::match_other_punctuator("^"),
            Some((super::OtherPunctuator::BitXor, ""))
        );
        assert_eq!(
            super::match_other_punctuator("^="),
            Some((super::OtherPunctuator::BitXorAssignment, ""))
        );
        assert_eq!(
            super::match_other_punctuator("]"),
            Some((super::OtherPunctuator::ClosingBracket, ""))
        );
        assert_eq!(
            super::match_other_punctuator(")"),
            Some((super::OtherPunctuator::ClosingParenthesis, ""))
        );
        assert_eq!(
            super::match_other_punctuator(":"),
            Some((super::OtherPunctuator::Colon, ""))
        );
        assert_eq!(
            super::match_other_punctuator(","),
            Some((super::OtherPunctuator::Comma, ""))
        );
        assert_eq!(
            super::match_other_punctuator("--"),
            Some((super::OtherPunctuator::Decrement, ""))
        );
        assert_eq!(
            super::match_other_punctuator("."),
            Some((super::OtherPunctuator::Dot, ""))
        );
        assert_eq!(
            super::match_other_punctuator("..."),
            Some((super::OtherPunctuator::Ellipsis, ""))
        );
        assert_eq!(
            super::match_other_punctuator("**"),
            Some((super::OtherPunctuator::Exponentiation, ""))
        );
        assert_eq!(
            super::match_other_punctuator("**="),
            Some((super::OtherPunctuator::ExponentiationAssignment, ""))
        );
        assert_eq!(
            super::match_other_punctuator("=>"),
            Some((super::OtherPunctuator::FunctionArrow, ""))
        );
        assert_eq!(
            super::match_other_punctuator("++"),
            Some((super::OtherPunctuator::Increment, ""))
        );
        assert_eq!(
            super::match_other_punctuator("<<"),
            Some((super::OtherPunctuator::LeftShift, ""))
        );
        assert_eq!(
            super::match_other_punctuator("<<="),
            Some((super::OtherPunctuator::LeftShiftAssignment, ""))
        );
        assert_eq!(
            super::match_other_punctuator("<"),
            Some((super::OtherPunctuator::Less, ""))
        );
        assert_eq!(
            super::match_other_punctuator("<="),
            Some((super::OtherPunctuator::LessOrEqual, ""))
        );
        assert_eq!(
            super::match_other_punctuator("=="),
            Some((super::OtherPunctuator::LooseEquality, ""))
        );
        assert_eq!(
            super::match_other_punctuator("!="),
            Some((super::OtherPunctuator::LooseInequality, ""))
        );
        assert_eq!(
            super::match_other_punctuator("%"),
            Some((super::OtherPunctuator::Modulo, ""))
        );
        assert_eq!(
            super::match_other_punctuator("%="),
            Some((super::OtherPunctuator::ModuloAssignment, ""))
        );
        assert_eq!(
            super::match_other_punctuator(">"),
            Some((super::OtherPunctuator::More, ""))
        );
        assert_eq!(
            super::match_other_punctuator(">="),
            Some((super::OtherPunctuator::MoreOrEqual, ""))
        );
        assert_eq!(
            super::match_other_punctuator("*"),
            Some((super::OtherPunctuator::Multiplication, ""))
        );
        assert_eq!(
            super::match_other_punctuator("*="),
            Some((super::OtherPunctuator::MultiplicationAssignment, ""))
        );
        assert_eq!(
            super::match_other_punctuator("!"),
            Some((super::OtherPunctuator::Not, ""))
        );
        assert_eq!(
            super::match_other_punctuator("??"),
            Some((super::OtherPunctuator::NullishCoalescence, ""))
        );
        assert_eq!(
            super::match_other_punctuator("??="),
            Some((super::OtherPunctuator::NullishCoalescenceAssignment, ""))
        );
        assert_eq!(
            super::match_other_punctuator("{"),
            Some((super::OtherPunctuator::OpeningBrace, ""))
        );
        assert_eq!(
            super::match_other_punctuator("["),
            Some((super::OtherPunctuator::OpeningBracket, ""))
        );
        assert_eq!(
            super::match_other_punctuator("("),
            Some((super::OtherPunctuator::OpeningParenthesis, ""))
        );
        assert_eq!(
            super::match_other_punctuator("||"),
            Some((super::OtherPunctuator::Or, ""))
        );
        assert_eq!(
            super::match_other_punctuator("||="),
            Some((super::OtherPunctuator::OrAssignment, ""))
        );
        assert_eq!(
            super::match_other_punctuator("?"),
            Some((super::OtherPunctuator::QuestionMark, ""))
        );
        assert_eq!(
            super::match_other_punctuator(">>"),
            Some((super::OtherPunctuator::RightShift, ""))
        );
        assert_eq!(
            super::match_other_punctuator(">>="),
            Some((super::OtherPunctuator::RightShiftAssignment, ""))
        );
        assert_eq!(
            super::match_other_punctuator(";"),
            Some((super::OtherPunctuator::Semicolon, ""))
        );
        assert_eq!(
            super::match_other_punctuator("==="),
            Some((super::OtherPunctuator::StrictEquality, ""))
        );
        assert_eq!(
            super::match_other_punctuator("!=="),
            Some((super::OtherPunctuator::StrictInequality, ""))
        );
        assert_eq!(
            super::match_other_punctuator("-"),
            Some((super::OtherPunctuator::Subtraction, ""))
        );
        assert_eq!(
            super::match_other_punctuator("-="),
            Some((super::OtherPunctuator::SubtractionAssignment, ""))
        );
        assert_eq!(
            super::match_other_punctuator(">>>"),
            Some((super::OtherPunctuator::UnsignedRightShift, ""))
        );
        assert_eq!(
            super::match_other_punctuator(">>>="),
            Some((super::OtherPunctuator::UnsignedRightShiftAssignment, ""))
        );
    }
}
