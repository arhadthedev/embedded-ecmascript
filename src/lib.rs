//! Embed ECMAScript into desktop and automotive programs.
//!
//! This library works with ECMA-262 grammar definition, both for parsing and
//! execution of static&dynamic semantics. So here is a reminder on grammar
//! terminology used in the specification.
//!
//! Each grammar rule looks like `Production :: ProductionDefinition`. Each
//! production has an algorithm for each static and dynamic semantics.

pub mod lexical_grammar;

use from_pest::FromPest;
use lexical_grammar::{Comment, CommonToken, DivPunctuator, Ecma262Parser, HashbangComment, InputElementDiv, InputElementHashbangOrRegExp, InputElementRegExp, InputElementRegExpOrTemplateTail, InputElementTemplateTail, LineTerminator, ReservedWord, RightBracePunctuator, Rule, WhiteSpace};
use pest::{iterators::Pairs, Parser};

/// An output of the tokenization step
#[derive(Debug, Eq, PartialEq)]
pub enum Token<'src> {
    Comment(Comment),
    CommonToken(CommonToken),
    DivPunctuator(DivPunctuator),
    HashbangComment(HashbangComment<'src>),
    LineTerminator(LineTerminator),
    ReservedWord(ReservedWord),
    RightBracePunctuator(RightBracePunctuator),
    WhiteSpace(WhiteSpace),
}

/// Kind of a grammar used for tokenization.
///
/// From <https://262.ecma-international.org/14.0/#sec-ecmascript-language-lexical-grammar>:
///
/// > There are several situations where the identification of lexical input
/// > elements is sensitive to the syntactic grammar context that is consuming
/// > the input elements. This requires multiple goal symbols for the lexical
/// > grammar.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GoalSymbols {
    /// > The *InputElementHashbangOrRegExp* goal is used at the start of
    /// > a *Script* or *Module*.
    InputElementHashbangOrRegExp,

    /// > The *InputElementRegExpOrTemplateTail* goal is used in syntactic
    /// > grammar contexts where a *RegularExpressionLiteral*,
    /// > a *TemplateMiddle*, or a *TemplateTail* is permitted.
    InputElementRegExpOrTemplateTail,

    /// > The *InputElementRegExp* goal symbol is used in all syntactic grammar
    /// > contexts where a *RegularExpressionLiteral* is permitted but neither
    /// > a *TemplateMiddle*, nor a *TemplateTail* is permitted.
    InputElementRegExp,

    /// > The *InputElementTemplateTail* goal is used in all syntactic grammar
    /// > contexts where a *TemplateMiddle* or a *TemplateTail* is permitted
    /// > but a *RegularExpressionLiteral* is not permitted.
    InputElementTemplateTail,

    /// > In all other contexts, *InputElementDiv* is used as the lexical goal
    /// > symbol.
    InputElementDiv
}

enum PackedToken<'src> {
    Div(InputElementDiv),
    HashbangOrRegExp(InputElementHashbangOrRegExp<'src>),
    RegExp(InputElementRegExp),
    RegExpOrTemplateTail(InputElementRegExpOrTemplateTail),
    TemplateTail(InputElementTemplateTail),
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
pub fn get_next_token(input: &str, mode: GoalSymbols) -> Result<(Token, &str), String> {
    let goal = match mode {
        GoalSymbols::InputElementHashbangOrRegExp => Rule::InputElementHashbangOrRegExp,
        GoalSymbols::InputElementRegExpOrTemplateTail => Rule::InputElementRegExpOrTemplateTail,
        GoalSymbols::InputElementRegExp => Rule::InputElementRegExp,
        GoalSymbols::InputElementTemplateTail => Rule::InputElementTemplateTail,
        GoalSymbols::InputElementDiv => Rule::InputElementDiv
    };
    let result = Ecma262Parser::parse(goal, input);
    match result {
        Ok(mut tokens) => {
            let tail = get_unprocessed_tail(tokens.clone(), input);
            let typed_packed: PackedToken = match mode {
                GoalSymbols::InputElementHashbangOrRegExp => {
                    let typed = crate::InputElementHashbangOrRegExp::from_pest(&mut tokens);
                    PackedToken::HashbangOrRegExp(typed.unwrap())
                },
                GoalSymbols::InputElementRegExpOrTemplateTail => {
                    let typed = crate::InputElementRegExpOrTemplateTail::from_pest(&mut tokens);
                    PackedToken::RegExpOrTemplateTail(typed.unwrap())
                },
                GoalSymbols::InputElementRegExp => {
                    let typed = crate::InputElementRegExp::from_pest(&mut tokens);
                    PackedToken::RegExp(typed.unwrap())
                },
                GoalSymbols::InputElementTemplateTail => {
                    let typed = crate::InputElementTemplateTail::from_pest(&mut tokens);
                    PackedToken::TemplateTail(typed.unwrap())
                },
                GoalSymbols::InputElementDiv => {
                    let typed = crate::InputElementDiv::from_pest(&mut tokens);
                    PackedToken::Div(typed.unwrap())
                },
            };
            Ok((unpack_token(typed_packed), tail))
        },
        Err(error) => Err(error.to_string())
    }
}

fn unpack_token(input: PackedToken<'_>) -> Token<'_> {
    match input {
        PackedToken::Div(root) => {
            match root {
                InputElementDiv::WhiteSpace(item) => Token::WhiteSpace(item),
                InputElementDiv::LineTerminator(item) => Token::LineTerminator(item),
                InputElementDiv::Comment(item) => Token::Comment(item),
                InputElementDiv::CommonToken(item) => Token::CommonToken(item),
                InputElementDiv::DivPunctuator(item) => Token::DivPunctuator(item),
                InputElementDiv::ReservedWord(item) => Token::ReservedWord(item),
                InputElementDiv::RightBracePunctuator(item) => Token::RightBracePunctuator(item),
            }
        },
        PackedToken::HashbangOrRegExp(root) => {
            match root {
                InputElementHashbangOrRegExp::WhiteSpace(item) => Token::WhiteSpace(item),
                InputElementHashbangOrRegExp::LineTerminator(item) => Token::LineTerminator(item),
                InputElementHashbangOrRegExp::Comment(item) => Token::Comment(item),
                InputElementHashbangOrRegExp::CommonToken(item) => Token::CommonToken(item),
                InputElementHashbangOrRegExp::HashbangComment(item) => Token::HashbangComment(item),
                InputElementHashbangOrRegExp::ReservedWord(item) => Token::ReservedWord(item),
            }
        },
        PackedToken::RegExp(root) => {
            match root {
                InputElementRegExp::WhiteSpace(item) => Token::WhiteSpace(item),
                InputElementRegExp::LineTerminator(item) => Token::LineTerminator(item),
                InputElementRegExp::Comment(item) => Token::Comment(item),
                InputElementRegExp::CommonToken(item) => Token::CommonToken(item),
                InputElementRegExp::ReservedWord(item) => Token::ReservedWord(item),
                InputElementRegExp::RightBracePunctuator(item) => Token::RightBracePunctuator(item),
            }
        },
        PackedToken::RegExpOrTemplateTail(root) => {
            match root {
                InputElementRegExpOrTemplateTail::WhiteSpace(item) => Token::WhiteSpace(item),
                InputElementRegExpOrTemplateTail::LineTerminator(item) => Token::LineTerminator(item),
                InputElementRegExpOrTemplateTail::Comment(item) => Token::Comment(item),
                InputElementRegExpOrTemplateTail::CommonToken(item) => Token::CommonToken(item),
                InputElementRegExpOrTemplateTail::DivPunctuator(item) => Token::DivPunctuator(item),
                InputElementRegExpOrTemplateTail::ReservedWord(item) => Token::ReservedWord(item),
            }
        },
        PackedToken::TemplateTail(root) => {
            match root {
                InputElementTemplateTail::WhiteSpace(item) => Token::WhiteSpace(item),
                InputElementTemplateTail::LineTerminator(item) => Token::LineTerminator(item),
                InputElementTemplateTail::Comment(item) => Token::Comment(item),
                InputElementTemplateTail::CommonToken(item) => Token::CommonToken(item),
                InputElementTemplateTail::DivPunctuator(item) => Token::DivPunctuator(item),
                InputElementTemplateTail::ReservedWord(item) => Token::ReservedWord(item),
            }
        },
    }
}

fn get_unprocessed_tail<'src>(
    mut recognized_source_start: Pairs<Rule>,
    whole_source: &'src str
) -> &'src str {
    let processed_substring = recognized_source_start.next().unwrap().as_span();
    &whole_source[processed_substring.end()..]
}
