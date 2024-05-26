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

use unicode_ident::{is_xid_continue, is_xid_start};

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

pub enum ReservedWord {
    Await,
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Debugger,
    Default,
    Delete,
    Do,
    Else,
    Enum,
    Export,
    Extends,
    False,
    Finally,
    For,
    Function,
    If,
    Import,
    In,
    InstanceOf,
    New,
    Null,
    Return,
    Super,
    Switch,
    This,
    Throw,
    True,
    Try,
    Typeof,
    Var,
    Void,
    While,
    With,
    Yield,
}

/// Try to match start of a string against `ReservedWord` production:
///
/// ```plain
/// ReservedWord :: one of
///     await break case catch class const continue debugger default delete do
///     else enum export extends false finally for function if import in
///     instanceof new null return super switch this throw true try typeof var
///     void while with yield
/// ```
///
/// Returns a tuple of an object created from the matched part and an unparsed
/// tail after the matched part.
///
/// Implements <https://262.ecma-international.org/14.0/#prod-ReservedWord>.
pub fn match_reserved_word(text: &str) -> Option<(ReservedWord, &str)> {
    text
        .strip_prefix("await").map(
            |tail| (ReservedWord::Await, tail)
        )
        .or_else(|| text.strip_prefix("break").map(
            |tail| (ReservedWord::Break, tail)
        ))
        .or_else(|| text.strip_prefix("case").map(
            |tail| (ReservedWord::Case, tail)
        ))
        .or_else(|| text.strip_prefix("catch").map(
            |tail| (ReservedWord::Catch, tail)
        ))
        .or_else(|| text.strip_prefix("class").map(
            |tail| (ReservedWord::Class, tail)
        ))
        .or_else(|| text.strip_prefix("const").map(
            |tail| (ReservedWord::Const, tail)
        ))
        .or_else(|| text.strip_prefix("continue").map(
            |tail| (ReservedWord::Continue, tail)
        ))
        .or_else(|| text.strip_prefix("debugger").map(
            |tail| (ReservedWord::Debugger, tail)
        ))
        .or_else(|| text.strip_prefix("default").map(
            |tail| (ReservedWord::Default, tail)
        ))
        .or_else(|| text.strip_prefix("delete").map(
            |tail| (ReservedWord::Delete, tail)
        ))
        .or_else(|| text.strip_prefix("do").map(
            |tail| (ReservedWord::Do, tail)
        ))
        .or_else(|| text.strip_prefix("else").map(
            |tail| (ReservedWord::Else, tail)
        ))
        .or_else(|| text.strip_prefix("enum").map(
            |tail| (ReservedWord::Enum, tail)
        ))
        .or_else(|| text.strip_prefix("export").map(
            |tail| (ReservedWord::Export, tail)
        ))
        .or_else(|| text.strip_prefix("extends").map(
            |tail| (ReservedWord::Extends, tail)
        ))
        .or_else(|| text.strip_prefix("false").map(
            |tail| (ReservedWord::False, tail)
        ))
        .or_else(|| text.strip_prefix("finally").map(
            |tail| (ReservedWord::Finally, tail)
        ))
        .or_else(|| text.strip_prefix("for").map(
            |tail| (ReservedWord::For, tail)
        ))
        .or_else(|| text.strip_prefix("function").map(
            |tail| (ReservedWord::Function, tail)
        ))
        .or_else(|| text.strip_prefix("if").map(
            |tail| (ReservedWord::If, tail)
        ))
        .or_else(|| text.strip_prefix("import").map(
            |tail| (ReservedWord::Import, tail)
        ))
        .or_else(|| text.strip_prefix("instanceof").map(
            |tail| (ReservedWord::InstanceOf, tail)
        ))
        .or_else(|| text.strip_prefix("in").map(
            |tail| (ReservedWord::In, tail)
        ))
        .or_else(|| text.strip_prefix("new").map(
            |tail| (ReservedWord::New, tail)
        ))
        .or_else(|| text.strip_prefix("null").map(
            |tail| (ReservedWord::Null, tail)
        ))
        .or_else(|| text.strip_prefix("return").map(
            |tail| (ReservedWord::Return, tail)
        ))
        .or_else(|| text.strip_prefix("super").map(
            |tail| (ReservedWord::Super, tail)
        ))
        .or_else(|| text.strip_prefix("switch").map(
            |tail| (ReservedWord::Switch, tail)
        ))
        .or_else(|| text.strip_prefix("this").map(
            |tail| (ReservedWord::This, tail)
        ))
        .or_else(|| text.strip_prefix("throw").map(
            |tail| (ReservedWord::Throw, tail)
        ))
        .or_else(|| text.strip_prefix("true").map(
            |tail| (ReservedWord::True, tail)
        ))
        .or_else(|| text.strip_prefix("try").map(
            |tail| (ReservedWord::Try, tail)
        ))
        .or_else(|| text.strip_prefix("typeof").map(
            |tail| (ReservedWord::Typeof, tail)
        ))
        .or_else(|| text.strip_prefix("var").map(
            |tail| (ReservedWord::Var, tail)
        ))
        .or_else(|| text.strip_prefix("void").map(
            |tail| (ReservedWord::Void, tail)
        ))
        .or_else(|| text.strip_prefix("while").map(
            |tail| (ReservedWord::While, tail)
        ))
        .or_else(|| text.strip_prefix("with").map(
            |tail| (ReservedWord::With, tail)
        ))
        .or_else(|| text.strip_prefix("yield").map(
            |tail| (ReservedWord::Yield, tail)
        ))
}

/// Try to match start of a string against `IdentifierStartChar` production:
///
/// ```plain
/// IdentifierStartChar ::
///     UnicodeIDStart
///     $
///     _
/// ```
///
/// Returns a tuple of an object created from the matched part and an unparsed
/// tail after the matched part.
///
/// Implements <https://262.ecma-international.org/14.0/#prod-IdentifierStartChar>.
pub fn match_identifier_start_char(text: &str) -> Option<(char, &str)> {
    match_unicode_id_start(text) 
        .or_else(|| text.strip_prefix('$').map(
            |tail| ('$', tail)
        ))
        .or_else(|| text.strip_prefix('_').map(
            |tail| ('_', tail)
        ))
}

/// Try to match start of a string against `IdentifierPartChar` production:
///
/// ```plain
/// IdentifierPartChar ::
///     UnicodeIDContinue
///     $
///     <ZWNJ>
///     <ZWJ>
/// ```
///
/// Returns a tuple of an object created from the matched part and an unparsed
/// tail after the matched part.
///
/// Implements <https://262.ecma-international.org/14.0/#prod-IdentifierPartChar>.
pub fn match_identifier_part_char(text: &str) -> Option<(char, &str)> {
    match_unicode_id_continue(text)
    .or_else(|| text.strip_prefix('$').map(
        |tail| ('$', tail)
    ))
    .or_else(|| match_zwnj(text).map(
        |((), tail)| ('\u{200C}', tail)
    ))
    .or_else(|| match_zwj(text).map(
        |((), tail)| ('\u{200D}', tail)
    ))
}

/// Try to match start of a string against `AsciiLetter` production:
///
/// ```plain
/// AsciiLetter :: one of
///     a b c d e f g h i j k l m n o p q r s t u v w x y z
///     A B C D E F G H I J K L M N O P Q R S T U V W X Y Z
/// ```
///
/// Returns a tuple of an object created from the matched part and an unparsed
/// tail after the matched part.
///
/// Implements <https://262.ecma-international.org/14.0/#prod-AsciiLetter>.
pub fn match_ascii_letter(text: &str) -> Option<(char, &str)> {
    let mut input = text.chars();
    let start = input.next();
    let tail = input.as_str();
    start
        .filter(char::is_ascii_alphabetic)
        .map(|character| (character, tail))
}

/// Try to match start of a string against `UnicodeIDStart` production:
///
/// ```plain
/// UnicodeIDStart ::
///     any Unicode code point with the Unicode property “ID_Start”
/// ```
///
/// Returns a tuple of an object created from the matched part and an unparsed
/// tail after the matched part.
///
/// Implements <https://262.ecma-international.org/14.0/#prod-UnicodeIDStart>.
pub fn match_unicode_id_start(text: &str) -> Option<(char, &str)> {
    let mut input = text.chars();
    let start = input.next();
    let tail = input.as_str();
    start
        .filter(|character| is_xid_start(*character))
        .map(|character| (character, tail))
}

/// Try to match start of a string against `UnicodeIDContinue` production:
///
/// ```plain
/// UnicodeIDContinue ::
///     any Unicode code point with the Unicode property “ID_Continue”
/// ```
///
/// Returns a tuple of an object created from the matched part and an unparsed
/// tail after the matched part.
///
/// Implements <https://262.ecma-international.org/14.0/#prod-UnicodeIDContinue>.
pub fn match_unicode_id_continue(text: &str) -> Option<(char, &str)> {
    let mut input = text.chars();
    let start = input.next();
    let tail = input.as_str();
    start
        .filter(|character| is_xid_continue(*character))
        .map(|character| (character, tail))
}

#[cfg(test)]
mod tests {
    use crate::_tokenizer::tests::{generate_cases, TerminalCase};
    use rstest::rstest;

     #[rstest]
    fn match_identifier_start_char(
        #[values(
            "X", "d", "д", "大", "$", "_", "\u{1885}", "\u{212E}" 
        )]
        tested: TerminalCase,
        #[values("foo", " ")]
        separator: &str
    ) {
        let all = generate_cases(&tested.terminal, separator);
        let safe_cases = all.iter().filter(|case| !case.input.starts_with("foo"));
        for case in safe_cases {
            assert_eq!(
                super::match_identifier_start_char(&case.input).map(|(_, tail)| tail),
                case.expected_tail.as_deref()
            );
        }
    }

    #[rstest]
    fn match_identifier_part_char(
        #[values(
            "X", "d", "д", "大", "$", "\u{0903}", "\u{200C}", "\u{200D}"
        )]
        tested: TerminalCase,
        #[values("foo", " ")]
        separator: &str
    ) {
        let all = generate_cases(&tested.terminal, separator);
        let safe_cases = all.iter().filter(|case| !case.input.starts_with("foo"));
        for case in safe_cases {
            assert_eq!(
                super::match_identifier_part_char(&case.input).map(|(_, tail)| tail),
                case.expected_tail.as_deref()
            );
        }
    }

    #[rstest]
    fn match_id(
        #[values(
            "X", "d", "д", "大", "\u{0903}", "\u{200C}", "\u{200D}"
        )]
        tested: TerminalCase,
        #[values("foo", " ")]
        separator: &str
    ) {
        let all = generate_cases(&tested.terminal, separator);
        let safe_cases = all.iter().filter(|case| !case.input.starts_with("foo"));
        for case in safe_cases {
            assert_eq!((tested.parser)(&case.input), case.expected_tail);
        }
    }

    #[rstest]
    fn match_reserved_word(
        #[values(
            "await", "break", "case", "catch", "class", "const", "continue",
            "debugger", "default", "delete", "do", "else", "enum", "export",
            "extends", "false", "finally", "for", "function", "if", "import",
            "in", "instanceof", "new", "null", "return", "super", "switch",
            "this", "throw", "true", "try", "typeof", "var", "void", "while",
            "with", "yield",
        )]
        tested: TerminalCase,
        #[values("foo", " ")]
        separator: &str
    ) {
        for case in generate_cases(&tested.terminal, separator) {
            assert_eq!((tested.parser)(&case.input), case.expected_tail);
        }
    }
}
