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
#[pest_ast(rule(lexical::Rule::OptionalChainingPunctuator))]
struct OptionalChainingPunctuator;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Addition))]
struct Addition;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::AdditionAssignment))]
struct AdditionAssignment;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::And))]
struct And;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::AndAssignment))]
struct AndAssignment;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Assignment))]
struct Assignment;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::BitAnd))]
struct BitAnd;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::BitAndAssignment))]
struct BitAndAssignment;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::BitNot))]
struct BitNot;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::BitOr))]
struct BitOr;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::BitOrAssignment))]
struct BitOrAssignment;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::BitXor))]
struct BitXor;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::BitXorAssignment))]
struct BitXorAssignment;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::ClosingBracket))]
struct ClosingBracket;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::ClosingParenthesis))]
struct ClosingParenthesis;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Colon))]
struct Colon;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Comma))]
struct Comma;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Decrement))]
struct Decrement;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Dot))]
struct Dot;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Ellipsis))]
struct Ellipsis;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Exponentiation))]
struct Exponentiation;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::ExponentiationAssignment))]
struct ExponentiationAssignment;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::FunctionArrow))]
struct FunctionArrow;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Increment))]
struct Increment;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::LeftShift))]
struct LeftShift;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::LeftShiftAssignment))]
struct LeftShiftAssignment;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Less))]
struct Less;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::LessOrEqual))]
struct LessOrEqual;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::LooseEquality))]
struct LooseEquality;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::LooseInequality))]
struct LooseInequality;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Modulo))]
struct Modulo;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::ModuloAssignment))]
struct ModuloAssignment;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::More))]
struct More;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::MoreOrEqual))]
struct MoreOrEqual;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Multiplication))]
struct Multiplication;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::MultiplicationAssignment))]
struct MultiplicationAssignment;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Not))]
struct Not;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::NullishCoalescence))]
struct NullishCoalescence;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::NullishCoalescenceAssignment))]
struct NullishCoalescenceAssignment;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::OpeningBrace))]
struct OpeningBrace;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::OpeningBracket))]
struct OpeningBracket;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::OpeningParenthesis))]
struct OpeningParenthesis;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Or))]
struct Or;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::OrAssignment))]
struct OrAssignment;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::QuestionMark))]
struct QuestionMark;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::RightShift))]
struct RightShift;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::RightShiftAssignment))]
struct RightShiftAssignment;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Semicolon))]
struct Semicolon;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::StrictEquality))]
struct StrictEquality;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::StrictInequality))]
struct StrictInequality;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Subtraction))]
struct Subtraction;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::SubtractionAssignment))]
struct SubtractionAssignment;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::UnsignedRightShift))]
struct UnsignedRightShift;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::UnsignedRightShiftAssignment))]
struct UnsignedRightShiftAssignment;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::OtherPunctuator))]
enum OtherPunctuator {
    Addition(Addition),
    AdditionAssignment(AdditionAssignment),
    And(And),
    AndAssignment(AndAssignment),
    Assignment(Assignment),
    BitAnd(BitAnd),
    BitAndAssignment(BitAndAssignment),
    BitNot(BitNot),
    BitOr(BitOr),
    BitOrAssignment(BitOrAssignment),
    BitXor(BitXor),
    BitXorAssignment(BitXorAssignment),
    ClosingBracket(ClosingBracket),
    ClosingParenthesis(ClosingParenthesis),
    Colon(Colon),
    Comma(Comma),
    Decrement(Decrement),
    Dot(Dot),
    Ellipsis(Ellipsis),
    Exponentiation(Exponentiation),
    ExponentiationAssignment(ExponentiationAssignment),
    FunctionArrow(FunctionArrow),
    Increment(Increment),
    LeftShift(LeftShift),
    LeftShiftAssignment(LeftShiftAssignment),
    Less(Less),
    LessOrEqual(LessOrEqual),
    LooseEquality(LooseEquality),
    LooseInequality(LooseInequality),
    Modulo(Modulo),
    ModuloAssignment(ModuloAssignment),
    More(More),
    MoreOrEqual(MoreOrEqual),
    Multiplication(Multiplication),
    MultiplicationAssignment(MultiplicationAssignment),
    Not(Not),
    NullishCoalescence(NullishCoalescence),
    NullishCoalescenceAssignment(NullishCoalescenceAssignment),
    OpeningBrace(OpeningBrace),
    OpeningBracket(OpeningBracket),
    OpeningParenthesis(OpeningParenthesis),
    Or(Or),
    OrAssignment(OrAssignment),
    QuestionMark(QuestionMark),
    RightShift(RightShift),
    RightShiftAssignment(RightShiftAssignment),
    Semicolon(Semicolon),
    StrictEquality(StrictEquality),
    StrictInequality(StrictInequality),
    Subtraction(Subtraction),
    SubtractionAssignment(SubtractionAssignment),
    UnsignedRightShift(UnsignedRightShift),
    UnsignedRightShiftAssignment(UnsignedRightShiftAssignment),
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Punctuator))]
enum Punctuator {
    OptionalChainingPunctuator(OptionalChainingPunctuator),
    OtherPunctuator(OtherPunctuator),
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::CommonToken))]
enum CommonToken {
    IdentifierName(IdentifierName),
    Punctuator(Punctuator),
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
#[pest_ast(rule(lexical::Rule::RightBracePunctuator))]
struct RightBracePunctuator;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::DivisionAssignment))]
struct DivisionAssignment;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::Division))]
struct Division;

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::DivPunctuator))]
enum DivPunctuator {
    DivisionAssignment(DivisionAssignment),
    Division(Division),
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(lexical::Rule::InputElementDiv))]
enum InputElementDiv {
    WhiteSpace(WhiteSpace),
    LineTerminator(LineTerminator),
    CommonToken(CommonToken),
    DivPunctuator(DivPunctuator),
    ReservedWord(ReservedWord),
    RightBracePunctuator(RightBracePunctuator),
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
        InputElementDiv::DivPunctuator(punctuator) => {
            match punctuator {
                DivPunctuator::DivisionAssignment(_) => Token::DivisionAssignment,
                DivPunctuator::Division(_) => Token::Division,
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
        InputElementDiv::RightBracePunctuator(_) => Token::ClosingBrace,
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
