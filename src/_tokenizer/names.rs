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

#[cfg(test)]
mod tests {
    use crate::_tokenizer::tests::{generate_cases, TerminalCase};
    use rstest::rstest;

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
