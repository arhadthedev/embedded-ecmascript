//! Embed ECMAScript into desktop and automotive programs.
//!
//! This library works with ECMA-262 grammar definition, both for parsing and
//! execution of static&dynamic semantics. So here is a reminder on grammar
//! terminology used in the specification.
//!
//! Each grammar rule looks like `Production :: ProductionDefinition`. Each
//! production has an algorithm for each static and dynamic semantics.

mod lexical {
    use pest_derive::Parser;

    #[derive(Parser)]
    #[grammar = "lexical_grammar.pest"]
    pub struct Ecma262Parser;
}

use from_pest::FromPest;
use pest::{Parser, Span};
use pest_ast::FromPest;

/// An output of the tokenization step
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    NumericLiteral(f64),
}

fn span_into_str(span: Span) -> &str {
    span.as_str()
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Digit))]
struct Digit {
    #[pest_ast(outer(with(span_into_str), with(str::parse), with(Result::unwrap)))]
    pub value: f64
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::UnprocessedTail))]
struct UnprocessedTail<'src> {
     #[pest_ast(outer(with(span_into_str)))]
    pub content: &'src str,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::DecimalDigit))]
struct DecimalDigit<'src> {
    pub digit: Digit,
    pub tail: UnprocessedTail<'src>
}

/// Extract a first token from a `.js`/`.mjs` text.
///
/// Returns a tuple of the token and an unprocessed input tail.
///
/// Tokenization is done as described in
/// <https://262.ecma-international.org/14.0/#sec-ecmascript-language-lexical-grammar>.
///
/// # Errors
///
/// Will return `Err` with rustc-style formatted error message string, if input
/// start does not form a correct  ECMAScript 2023 token.
///
/// # Panics
///
/// Will panic if the root grammar errorneously defines an empty goal symbol.
/// This means a broken grammar file used by developers to build the parser.
pub fn get_next_token(input: &str) -> Result<(Token, &str), String> {
    let result = lexical::Ecma262Parser::parse(lexical::Rule::DecimalDigit, input);
    match result {
        Ok(mut tokens) => {
            let parsed = DecimalDigit::from_pest(&mut tokens).unwrap();
            Ok((Token::NumericLiteral(parsed.digit.value), parsed.tail.content))
        },
        Err(error) => Err(error.to_string())
    }
}

mod _tokenizer;

use std::cmp::{Eq, PartialEq};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as _Result};
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
    fn fmt(&self, f: &mut Formatter<'_>) -> _Result {
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
    _tokenizer::names::match_identifier_start_char("");
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
