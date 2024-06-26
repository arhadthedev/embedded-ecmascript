//! ORM for tokenizer of `.js` and `.mjs` files.
//!
//! Supplements the tokenizer by repacking weakly typed span-based parse tree
//! into the strongly typed struct-per-nonterminal parse tree.
//!
//! Each strongly typed parse tree node has a few methods specified by ECMA-262.
//! These methods, called semantics, calculate values and perform checks. From
//! <https://262.ecma-international.org/14.0/#sec-static-semantic-rules>:
//!
//! > Context-free grammars are not sufficiently powerful to express all
//! > the rules that define whether a stream of input elements form a valid
//! > ECMAScript Script or Module that may be evaluated. In some situations
//! > additional rules are needed that may be expressed using either ECMAScript
//! > algorithm conventions or prose requirements. Such rules are always
//! > associated with a production of a grammar and are called the static
//! > semantics of the production.
//! >
//! > Static Semantic Rules have names and typically are defined using
//! > an algorithm. Named Static Semantic Rules are associated with grammar
//! > productions and a production that has multiple alternative definitions
//! > will typically have for each alternative a distinct algorithm for each
//! > applicable named static semantic rule.
//!
//! Implements <https://262.ecma-international.org/14.0/#sec-ecmascript-language-lexical-grammar>.
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

/************************************************
 * 
 * Lexical grammar tree
 *
 ************************************************/

use pest::Span;
use pest_ast::FromPest;
use pest_derive::Parser;

fn span_into_str(span: Span) -> &str {
    span.as_str()
}

#[derive(Parser)]
#[grammar = "lexical_grammar.pest"]
struct Ecma262Parser;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::WhiteSpace))]
pub struct WhiteSpace;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::LineTerminator))]
pub struct LineTerminator;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::PrivateIdentifier))]
pub struct PrivateIdentifier {
    identifier_name: IdentifierName
}

impl PrivateIdentifier {
    /// <https://262.ecma-international.org/14.0/#sec-static-semantics-stringvalue>
    #[must_use]
    pub fn string_value(&self) -> String {
        // 1. Return the string-concatenation of 0x0023 (NUMBER SIGN) and
        //    the StringValue of IdentifierName.
        "#".to_owned() + &self.identifier_name.string_value()
    }
}

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::IdentifierName))]
pub struct IdentifierName {
    // Escape sequence decoding do not allow to use `&str`
    #[pest_ast(outer(with(span_into_str), with(str::to_string)))]
    decoded: String
}

impl IdentifierName {
    #[must_use]
    pub fn string_value(&self) -> String {
        self.decoded.clone()
    }
}

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::OptionalChainingPunctuator))]
pub struct OptionalChainingPunctuator;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Addition))]
pub struct Addition;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::AdditionAssignment))]
pub struct AdditionAssignment;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::And))]
pub struct And;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::AndAssignment))]
pub struct AndAssignment;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Assignment))]
pub struct Assignment;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::BitAnd))]
pub struct BitAnd;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::BitAndAssignment))]
pub struct BitAndAssignment;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::BitNot))]
pub struct BitNot;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::BitOr))]
pub struct BitOr;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::BitOrAssignment))]
pub struct BitOrAssignment;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::BitXor))]
pub struct BitXor;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::BitXorAssignment))]
pub struct BitXorAssignment;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::ClosingBracket))]
pub struct ClosingBracket;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::ClosingParenthesis))]
pub struct ClosingParenthesis;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Colon))]
pub struct Colon;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Comma))]
pub struct Comma;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Decrement))]
pub struct Decrement;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Dot))]
pub struct Dot;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Ellipsis))]
pub struct Ellipsis;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Exponentiation))]
pub struct Exponentiation;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::ExponentiationAssignment))]
pub struct ExponentiationAssignment;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::FunctionArrow))]
pub struct FunctionArrow;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Increment))]
pub struct Increment;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::LeftShift))]
pub struct LeftShift;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::LeftShiftAssignment))]
pub struct LeftShiftAssignment;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Less))]
pub struct Less;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::LessOrEqual))]
pub struct LessOrEqual;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::LooseEquality))]
pub struct LooseEquality;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::LooseInequality))]
pub struct LooseInequality;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Modulo))]
pub struct Modulo;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::ModuloAssignment))]
pub struct ModuloAssignment;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::More))]
pub struct More;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::MoreOrEqual))]
pub struct MoreOrEqual;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Multiplication))]
pub struct Multiplication;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::MultiplicationAssignment))]
pub struct MultiplicationAssignment;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Not))]
pub struct Not;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::NullishCoalescence))]
pub struct NullishCoalescence;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::NullishCoalescenceAssignment))]
pub struct NullishCoalescenceAssignment;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::OpeningBrace))]
pub struct OpeningBrace;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::OpeningBracket))]
pub struct OpeningBracket;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::OpeningParenthesis))]
pub struct OpeningParenthesis;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Or))]
pub struct Or;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::OrAssignment))]
pub struct OrAssignment;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::QuestionMark))]
pub struct QuestionMark;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::RightShift))]
pub struct RightShift;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::RightShiftAssignment))]
pub struct RightShiftAssignment;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Semicolon))]
pub struct Semicolon;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::StrictEquality))]
pub struct StrictEquality;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::StrictInequality))]
pub struct StrictInequality;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Subtraction))]
pub struct Subtraction;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::SubtractionAssignment))]
pub struct SubtractionAssignment;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::UnsignedRightShift))]
pub struct UnsignedRightShift;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::UnsignedRightShiftAssignment))]
pub struct UnsignedRightShiftAssignment;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::OtherPunctuator))]
pub enum OtherPunctuator {
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

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Punctuator))]
pub enum Punctuator {
    OptionalChainingPunctuator(OptionalChainingPunctuator),
    OtherPunctuator(OtherPunctuator),
}

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::CommonToken))]
pub enum CommonToken {
    IdentifierName(IdentifierName),
    PrivateIdentifier(PrivateIdentifier),
    Punctuator(Punctuator),
}

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Await))]
pub struct Await;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Break))]
pub struct Break;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Case))]
pub struct Case;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Catch))]
pub struct Catch;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Class))]
pub struct Class;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Const))]
pub struct Const;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Continue))]
pub struct Continue;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Debugger))]
pub struct Debugger;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Default))]
pub struct Default;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Delete))]
pub struct Delete;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Do))]
pub struct Do;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Else))]
pub struct Else;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Enum))]
pub struct Enum;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Export))]
pub struct Export;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Extends))]
pub struct Extends;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::False))]
pub struct False;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Finally))]
pub struct Finally;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::For))]
pub struct For;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Function))]
pub struct Function;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::If))]
pub struct If;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Import))]
pub struct Import;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::In))]
pub struct In;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Instanceof))]
pub struct Instanceof;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::New))]
pub struct New;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Null))]
pub struct Null;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Return))]
pub struct Return;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Super))]
pub struct Super;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Switch))]
pub struct Switch;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::This))]
pub struct This;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Throw))]
pub struct Throw;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::True))]
pub struct True;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Try))]
pub struct Try;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Typeof))]
pub struct Typeof;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Var))]
pub struct Var;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Void))]
pub struct Void;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::While))]
pub struct While;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::With))]
pub struct With;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Yield))]
pub struct Yield;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::ReservedWord))]
pub enum ReservedWord {
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

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::RightBracePunctuator))]
pub struct RightBracePunctuator;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::DivisionAssignment))]
pub struct DivisionAssignment;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Division))]
pub struct Division;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::DivPunctuator))]
pub enum DivPunctuator {
    DivisionAssignment(DivisionAssignment),
    Division(Division),
}

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::Comment))]
pub enum Comment {
    MultiLineComment(MultiLineComment),
    SingleLineComment(SingleLineComment),
}

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::MultiLineComment))]
pub struct MultiLineComment;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::SingleLineComment))]
pub struct SingleLineComment;

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::HashbangComment))]
pub struct HashbangComment<'src> {
     #[pest_ast(outer(with(span_into_str)))]
    content: &'src str,
}

impl HashbangComment<'_> {
    /// Vendor-specific static semantic declaration and definition
    #[must_use]
    pub fn string_value(&self) -> &str {
        &self.content[2..]
    }
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::InputElementDiv))]
pub enum InputElementDiv {
    WhiteSpace(WhiteSpace),
    LineTerminator(LineTerminator),
    Comment(Comment),
    CommonToken(CommonToken),
    DivPunctuator(DivPunctuator),
    ReservedWord(ReservedWord),
    RightBracePunctuator(RightBracePunctuator),
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::InputElementRegExp))]
pub enum InputElementRegExp {
    WhiteSpace(WhiteSpace),
    LineTerminator(LineTerminator),
    Comment(Comment),
    CommonToken(CommonToken),
    ReservedWord(ReservedWord),
    RightBracePunctuator(RightBracePunctuator),
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::InputElementRegExpOrTemplateTail))]
pub enum InputElementRegExpOrTemplateTail {
    WhiteSpace(WhiteSpace),
    LineTerminator(LineTerminator),
    Comment(Comment),
    CommonToken(CommonToken),
    DivPunctuator(DivPunctuator),
    ReservedWord(ReservedWord),
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::InputElementTemplateTail))]
pub enum InputElementTemplateTail {
    WhiteSpace(WhiteSpace),
    LineTerminator(LineTerminator),
    Comment(Comment),
    CommonToken(CommonToken),
    DivPunctuator(DivPunctuator),
    ReservedWord(ReservedWord),
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::InputElementHashbangOrRegExp))]
pub enum InputElementHashbangOrRegExp<'src> {
    WhiteSpace(WhiteSpace),
    LineTerminator(LineTerminator),
    Comment(Comment),
    CommonToken(CommonToken),
    HashbangComment(HashbangComment<'src>),
    ReservedWord(ReservedWord),
}

/************************************************
 * 
 * Lexical grammar tree factory
 *
 ************************************************/

use from_pest::FromPest;
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

fn get_unprocessed_tail<'src>(
    recognized_source_start: &Pairs<Rule>,
    whole_source: &'src str
) -> &'src str {
    let mut tokens = recognized_source_start.clone();
    let processed_substring = tokens.next().unwrap().as_span();
    &whole_source[processed_substring.end()..]
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
    Ecma262Parser::parse(goal, input)
        .map(|mut tokens| -> (Token, &str) {
            let tail = get_unprocessed_tail(&tokens, input);
            let typed_packed: PackedToken = match mode {
                GoalSymbols::InputElementHashbangOrRegExp => {
                    let typed = InputElementHashbangOrRegExp::from_pest(&mut tokens);
                    PackedToken::HashbangOrRegExp(typed.unwrap())
                },
                GoalSymbols::InputElementRegExpOrTemplateTail => {
                    let typed = InputElementRegExpOrTemplateTail::from_pest(&mut tokens);
                    PackedToken::RegExpOrTemplateTail(typed.unwrap())
                },
                GoalSymbols::InputElementRegExp => {
                    let typed = InputElementRegExp::from_pest(&mut tokens);
                    PackedToken::RegExp(typed.unwrap())
                },
                GoalSymbols::InputElementTemplateTail => {
                    let typed = InputElementTemplateTail::from_pest(&mut tokens);
                    PackedToken::TemplateTail(typed.unwrap())
                },
                GoalSymbols::InputElementDiv => {
                    let typed = InputElementDiv::from_pest(&mut tokens);
                    PackedToken::Div(typed.unwrap())
                },
            };
            (unpack_token(typed_packed), tail)
        })
        .map_err(|error| error.to_string())
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
