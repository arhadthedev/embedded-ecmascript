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
use pest::{iterators::Pairs, Parser, Span};
use pest_ast::FromPest;

/// A keyword; may be used as a name in some cases.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Keyword {
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
    Instanceof,
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

/// An output of the tokenization step
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    WhiteSpace,
    LineTerminator,
    IdentifierName(String),
    NumericLiteral(f64),
    ReservedWord(Keyword),
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
#[pest_ast(rule(lexical::Rule::DecimalDigit))]
struct DecimalDigit {
    pub digit: Digit,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::WhiteSpace))]
struct WhiteSpace;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::LineTerminator))]
struct LineTerminator;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::IdentifierName))]
struct IdentifierName {
    // Escape sequence decoding do not allow to use `&str`
    #[pest_ast(outer(with(span_into_str), with(str::to_string)))]
    pub decoded: String
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::CommonToken))]
enum CommonToken {
    IdentifierName(IdentifierName),
}
#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Await))]
struct Await;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Break))]
struct Break;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Case))]
struct Case;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Catch))]
struct Catch;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Class))]
struct Class;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Const))]
struct Const;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Continue))]
struct Continue;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Debugger))]
struct Debugger;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Default))]
struct Default;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Delete))]
struct Delete;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Do))]
struct Do;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Else))]
struct Else;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Enum))]
struct Enum;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Export))]
struct Export;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Extends))]
struct Extends;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::False))]
struct False;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Finally))]
struct Finally;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::For))]
struct For;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Function))]
struct Function;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::If))]
struct If;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Import))]
struct Import;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::In))]
struct In;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Instanceof))]
struct Instanceof;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::New))]
struct New;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Null))]
struct Null;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Return))]
struct Return;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Super))]
struct Super;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Switch))]
struct Switch;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::This))]
struct This;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Throw))]
struct Throw;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::True))]
struct True;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Try))]
struct Try;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Typeof))]
struct Typeof;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Var))]
struct Var;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Void))]
struct Void;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::While))]
struct While;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::With))]
struct With;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Yield))]
struct Yield;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::ReservedWord))]
enum ReservedWord {
    Await(Await),
    Break(Break),
    Case(Case),
    Catch(Catch),
    Class(Class),
    Const(Const),
    Continue(Continue),
    Debugger(Debugger),
    Default(Default),
    Delete(Delete),
    Do(Do),
    Else(Else),
    Enum(Enum),
    Export(Export),
    Extends(Extends),
    False(False),
    Finally(Finally),
    For(For),
    Function(Function),
    If(If),
    Import(Import),
    In(In),
    Instanceof(Instanceof),
    New(New),
    Null(Null),
    Return(Return),
    Super(Super),
    Switch(Switch),
    This(This),
    Throw(Throw),
    True(True),
    Try(Try),
    Typeof(Typeof),
    Var(Var),
    Void(Void),
    While(While),
    With(With),
    Yield(Yield),
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::InputElementDiv))]
enum InputElementDiv {
    WhiteSpace(WhiteSpace),
    LineTerminator(LineTerminator),
    CommonToken(CommonToken),
    ReservedWord(ReservedWord),
    DecimalDigit(DecimalDigit),
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
    let result = lexical::Ecma262Parser::parse(lexical::Rule::InputElementDiv, input);
    match result {
        Ok(mut tokens) => {
            let tail = get_unprocessed_tail(tokens.clone(), input);
            let parsed = InputElementDiv::from_pest(&mut tokens).unwrap();
            Ok((extract_token(parsed), tail))
        },
        Err(error) => Err(error.to_string())
    }
}

fn extract_token(symbol_tree: InputElementDiv) -> Token {
    match symbol_tree {
        InputElementDiv::DecimalDigit(value) => Token::NumericLiteral(value.digit.value),
        InputElementDiv::WhiteSpace(_) => Token::WhiteSpace,
        InputElementDiv::CommonToken(token) => {
            match token {
                CommonToken::IdentifierName(name) => Token::IdentifierName(name.decoded),
            }
        },
        InputElementDiv::ReservedWord(keyword) => {
            match keyword {
                ReservedWord::Await(_) => Token::ReservedWord(Keyword::Await),
                ReservedWord::Break(_) => Token::ReservedWord(Keyword::Break),
                ReservedWord::Case(_) => Token::ReservedWord(Keyword::Case),
                ReservedWord::Catch(_) => Token::ReservedWord(Keyword::Catch),
                ReservedWord::Class(_) => Token::ReservedWord(Keyword::Class),
                ReservedWord::Const(_) => Token::ReservedWord(Keyword::Const),
                ReservedWord::Continue(_) => Token::ReservedWord(Keyword::Continue),
                ReservedWord::Debugger(_) => Token::ReservedWord(Keyword::Debugger),
                ReservedWord::Default(_) => Token::ReservedWord(Keyword::Default),
                ReservedWord::Delete(_) => Token::ReservedWord(Keyword::Delete),
                ReservedWord::Do(_) => Token::ReservedWord(Keyword::Do),
                ReservedWord::Else(_) => Token::ReservedWord(Keyword::Else),
                ReservedWord::Enum(_) => Token::ReservedWord(Keyword::Enum),
                ReservedWord::Export(_) => Token::ReservedWord(Keyword::Export),
                ReservedWord::Extends(_) => Token::ReservedWord(Keyword::Extends),
                ReservedWord::False(_) => Token::ReservedWord(Keyword::False),
                ReservedWord::Finally(_) => Token::ReservedWord(Keyword::Finally),
                ReservedWord::For(_) => Token::ReservedWord(Keyword::For),
                ReservedWord::Function(_) => Token::ReservedWord(Keyword::Function),
                ReservedWord::If(_) => Token::ReservedWord(Keyword::If),
                ReservedWord::Import(_) => Token::ReservedWord(Keyword::Import),
                ReservedWord::In(_) => Token::ReservedWord(Keyword::In),
                ReservedWord::Instanceof(_) => Token::ReservedWord(Keyword::Instanceof),
                ReservedWord::New(_) => Token::ReservedWord(Keyword::New),
                ReservedWord::Null(_) => Token::ReservedWord(Keyword::Null),
                ReservedWord::Return(_) => Token::ReservedWord(Keyword::Return),
                ReservedWord::Super(_) => Token::ReservedWord(Keyword::Super),
                ReservedWord::Switch(_) => Token::ReservedWord(Keyword::Switch),
                ReservedWord::This(_) => Token::ReservedWord(Keyword::This),
                ReservedWord::Throw(_) => Token::ReservedWord(Keyword::Throw),
                ReservedWord::True(_) => Token::ReservedWord(Keyword::True),
                ReservedWord::Try(_) => Token::ReservedWord(Keyword::Try),
                ReservedWord::Typeof(_) => Token::ReservedWord(Keyword::Typeof),
                ReservedWord::Var(_) => Token::ReservedWord(Keyword::Var),
                ReservedWord::Void(_) => Token::ReservedWord(Keyword::Void),
                ReservedWord::While(_) => Token::ReservedWord(Keyword::While),
                ReservedWord::With(_) => Token::ReservedWord(Keyword::With),
                ReservedWord::Yield(_) => Token::ReservedWord(Keyword::Yield),
            }
        },
        InputElementDiv::LineTerminator(_) => Token::LineTerminator,
    }
}

fn get_unprocessed_tail<'src>(
    mut recognized_source_start: Pairs<lexical::Rule>,
    whole_source: &'src str
) -> &'src str {
    let processed_substring = recognized_source_start.next().unwrap().as_span();
    &whole_source[processed_substring.end()..]
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
