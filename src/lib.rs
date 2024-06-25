//! Embed ECMAScript into desktop and automotive programs.
//!
//! This library works with ECMA-262 grammar definition, both for parsing and
//! execution of static&dynamic semantics. So here is a reminder on grammar
//! terminology used in the specification.
//!
//! Each grammar rule looks like `Production :: ProductionDefinition`. Each
//! production has an algorithm for each static and dynamic semantics.

mod lexical_grammar;

use from_pest::FromPest;
use lexical_grammar::{Comment, CommonToken, DivPunctuator, Ecma262Parser, HashbangComment, InputElementDiv, InputElementHashbangOrRegExp, InputElementRegExp, InputElementRegExpOrTemplateTail, InputElementTemplateTail, LineTerminator, OtherPunctuator, Punctuator, ReservedWord, RightBracePunctuator, Rule, WhiteSpace};
use pest::{iterators::Pairs, Parser};

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
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    WhiteSpace,
    LineTerminator,
    Comment,
    HashbangComment(String),

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
    ClosingBrace,
    ClosingBracket,
    ClosingParenthesis,
    Colon,
    Comma,
    Decrement,
    Division,
    DivisionAssignment,
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
    OptionalChaining,
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

    IdentifierName(String),
    PrivateIdentifier(String),
    ReservedWord(Keyword),
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

enum UnpackedToken<'src> {
    Comment(Comment),
    CommonToken(CommonToken),
    DivPunctuator(DivPunctuator),
    HashbangComment(HashbangComment<'src>),
    LineTerminator(LineTerminator),
    ReservedWord(ReservedWord),
    RightBracePunctuator(RightBracePunctuator),
    WhiteSpace(WhiteSpace),
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
            Ok((flatten_token(unpack_token(typed_packed)), tail))
        },
        Err(error) => Err(error.to_string())
    }
}

fn unpack_token(input: PackedToken<'_>) -> UnpackedToken<'_> {
    match input {
        PackedToken::Div(root) => {
            match root {
                InputElementDiv::WhiteSpace(item) => UnpackedToken::WhiteSpace(item),
                InputElementDiv::LineTerminator(item) => UnpackedToken::LineTerminator(item),
                InputElementDiv::Comment(item) => UnpackedToken::Comment(item),
                InputElementDiv::CommonToken(item) => UnpackedToken::CommonToken(item),
                InputElementDiv::DivPunctuator(item) => UnpackedToken::DivPunctuator(item),
                InputElementDiv::ReservedWord(item) => UnpackedToken::ReservedWord(item),
                InputElementDiv::RightBracePunctuator(item) => UnpackedToken::RightBracePunctuator(item),
            }
        },
        PackedToken::HashbangOrRegExp(root) => {
            match root {
                InputElementHashbangOrRegExp::WhiteSpace(item) => UnpackedToken::WhiteSpace(item),
                InputElementHashbangOrRegExp::LineTerminator(item) => UnpackedToken::LineTerminator(item),
                InputElementHashbangOrRegExp::Comment(item) => UnpackedToken::Comment(item),
                InputElementHashbangOrRegExp::CommonToken(item) => UnpackedToken::CommonToken(item),
                InputElementHashbangOrRegExp::HashbangComment(item) => UnpackedToken::HashbangComment(item),
                InputElementHashbangOrRegExp::ReservedWord(item) => UnpackedToken::ReservedWord(item),
            }
        },
        PackedToken::RegExp(root) => {
            match root {
                InputElementRegExp::WhiteSpace(item) => UnpackedToken::WhiteSpace(item),
                InputElementRegExp::LineTerminator(item) => UnpackedToken::LineTerminator(item),
                InputElementRegExp::Comment(item) => UnpackedToken::Comment(item),
                InputElementRegExp::CommonToken(item) => UnpackedToken::CommonToken(item),
                InputElementRegExp::ReservedWord(item) => UnpackedToken::ReservedWord(item),
                InputElementRegExp::RightBracePunctuator(item) => UnpackedToken::RightBracePunctuator(item),
            }
        },
        PackedToken::RegExpOrTemplateTail(root) => {
            match root {
                InputElementRegExpOrTemplateTail::WhiteSpace(item) => UnpackedToken::WhiteSpace(item),
                InputElementRegExpOrTemplateTail::LineTerminator(item) => UnpackedToken::LineTerminator(item),
                InputElementRegExpOrTemplateTail::Comment(item) => UnpackedToken::Comment(item),
                InputElementRegExpOrTemplateTail::CommonToken(item) => UnpackedToken::CommonToken(item),
                InputElementRegExpOrTemplateTail::DivPunctuator(item) => UnpackedToken::DivPunctuator(item),
                InputElementRegExpOrTemplateTail::ReservedWord(item) => UnpackedToken::ReservedWord(item),
            }
        },
        PackedToken::TemplateTail(root) => {
            match root {
                InputElementTemplateTail::WhiteSpace(item) => UnpackedToken::WhiteSpace(item),
                InputElementTemplateTail::LineTerminator(item) => UnpackedToken::LineTerminator(item),
                InputElementTemplateTail::Comment(item) => UnpackedToken::Comment(item),
                InputElementTemplateTail::CommonToken(item) => UnpackedToken::CommonToken(item),
                InputElementTemplateTail::DivPunctuator(item) => UnpackedToken::DivPunctuator(item),
                InputElementTemplateTail::ReservedWord(item) => UnpackedToken::ReservedWord(item),
            }
        },
    }
}

fn flatten_token(symbol_tree: UnpackedToken) -> Token {
    match symbol_tree {
        UnpackedToken::WhiteSpace(_) => Token::WhiteSpace,
        UnpackedToken::Comment(kind) => {
            match kind {
                Comment::MultiLineComment(_) | Comment::SingleLineComment(_) => Token::Comment
            }
        },
        UnpackedToken::HashbangComment(line) => {
            Token::HashbangComment(line.content[2..].to_string())
        },
        UnpackedToken::CommonToken(token) => {
            match token {
                CommonToken::IdentifierName(name) => Token::IdentifierName(name.decoded),
                CommonToken::PrivateIdentifier(name) => Token::PrivateIdentifier(name.payload.decoded),
                CommonToken::Punctuator(punctuator) => {
                    match punctuator {
                        Punctuator::OptionalChainingPunctuator(_) => Token::OptionalChaining,
                        Punctuator::OtherPunctuator(symbol) => {
                            match symbol  {
                                OtherPunctuator::Addition(_) => Token::Addition,
                                OtherPunctuator::AdditionAssignment(_) => Token::AdditionAssignment,
                                OtherPunctuator::And(_) => Token::And,
                                OtherPunctuator::AndAssignment(_) => Token::AndAssignment,
                                OtherPunctuator::Assignment(_) => Token::Assignment,
                                OtherPunctuator::BitAnd(_) => Token::BitAnd,
                                OtherPunctuator::BitAndAssignment(_) => Token::BitAndAssignment,
                                OtherPunctuator::BitNot(_) => Token::BitNot,
                                OtherPunctuator::BitOr(_) => Token::BitOr,
                                OtherPunctuator::BitOrAssignment(_) => Token::BitOrAssignment,
                                OtherPunctuator::BitXor(_) => Token::BitXor,
                                OtherPunctuator::BitXorAssignment(_) => Token::BitXorAssignment,
                                OtherPunctuator::ClosingBracket(_) => Token::ClosingBracket,
                                OtherPunctuator::ClosingParenthesis(_) => Token::ClosingParenthesis,
                                OtherPunctuator::Colon(_) => Token::Colon,
                                OtherPunctuator::Comma(_) => Token::Comma,
                                OtherPunctuator::Decrement(_) => Token::Decrement,
                                OtherPunctuator::Dot(_) => Token::Dot,
                                OtherPunctuator::Ellipsis(_) => Token::Ellipsis,
                                OtherPunctuator::Exponentiation(_) => Token::Exponentiation,
                                OtherPunctuator::ExponentiationAssignment(_) => Token::ExponentiationAssignment,
                                OtherPunctuator::FunctionArrow(_) => Token::FunctionArrow,
                                OtherPunctuator::Increment(_) => Token::Increment,
                                OtherPunctuator::LeftShift(_) => Token::LeftShift,
                                OtherPunctuator::LeftShiftAssignment(_) => Token::LeftShiftAssignment,
                                OtherPunctuator::Less(_) => Token::Less,
                                OtherPunctuator::LessOrEqual(_) => Token::LessOrEqual,
                                OtherPunctuator::LooseEquality(_) => Token::LooseEquality,
                                OtherPunctuator::LooseInequality(_) => Token::LooseInequality,
                                OtherPunctuator::Modulo(_) => Token::Modulo,
                                OtherPunctuator::ModuloAssignment(_) => Token::ModuloAssignment,
                                OtherPunctuator::More(_) => Token::More,
                                OtherPunctuator::MoreOrEqual(_) => Token::MoreOrEqual,
                                OtherPunctuator::Multiplication(_) => Token::Multiplication,
                                OtherPunctuator::MultiplicationAssignment(_) => Token::MultiplicationAssignment,
                                OtherPunctuator::Not(_) => Token::Not,
                                OtherPunctuator::NullishCoalescence(_) => Token::NullishCoalescence,
                                OtherPunctuator::NullishCoalescenceAssignment(_) => Token::NullishCoalescenceAssignment,
                                OtherPunctuator::OpeningBrace(_) => Token::OpeningBrace,
                                OtherPunctuator::OpeningBracket(_) => Token::OpeningBracket,
                                OtherPunctuator::OpeningParenthesis(_) => Token::OpeningParenthesis,
                                OtherPunctuator::Or(_) => Token::Or,
                                OtherPunctuator::OrAssignment(_) => Token::OrAssignment,
                                OtherPunctuator::QuestionMark(_) => Token::QuestionMark,
                                OtherPunctuator::RightShift(_) => Token::RightShift,
                                OtherPunctuator::RightShiftAssignment(_) => Token::RightShiftAssignment,
                                OtherPunctuator::Semicolon(_) => Token::Semicolon,
                                OtherPunctuator::StrictEquality(_) => Token::StrictEquality,
                                OtherPunctuator::StrictInequality(_) => Token::StrictInequality,
                                OtherPunctuator::Subtraction(_) => Token::Subtraction,
                                OtherPunctuator::SubtractionAssignment(_) => Token::SubtractionAssignment,
                                OtherPunctuator::UnsignedRightShift(_) => Token::UnsignedRightShift,
                                OtherPunctuator::UnsignedRightShiftAssignment(_) => Token::UnsignedRightShiftAssignment,
                            }
                        }
                    }
                }
            }
        },
        UnpackedToken::DivPunctuator(punctuator) => {
            match punctuator {
                DivPunctuator::DivisionAssignment(_) => Token::DivisionAssignment,
                DivPunctuator::Division(_) => Token::Division,
            }
        },
        UnpackedToken::ReservedWord(keyword) => {
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
        UnpackedToken::RightBracePunctuator(_) => Token::ClosingBrace,
        UnpackedToken::LineTerminator(_) => Token::LineTerminator,
    }
}

fn get_unprocessed_tail<'src>(
    mut recognized_source_start: Pairs<Rule>,
    whole_source: &'src str
) -> &'src str {
    let processed_substring = recognized_source_start.next().unwrap().as_span();
    &whole_source[processed_substring.end()..]
}
