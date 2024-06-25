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

use pest::Span;
use pest_ast::FromPest;
use pest_derive::Parser;

fn span_into_str(span: Span) -> &str {
    span.as_str()
}

#[derive(Parser)]
#[grammar = "lexical_grammar.pest"]
pub struct Ecma262Parser;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::WhiteSpace))]
pub struct WhiteSpace;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::LineTerminator))]
pub struct LineTerminator;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::PrivateIdentifier))]
pub struct PrivateIdentifier {
    pub payload: IdentifierName
}

#[derive(Debug, Eq, FromPest, PartialEq)]
#[pest_ast(rule(Rule::IdentifierName))]
pub struct IdentifierName {
    // Escape sequence decoding do not allow to use `&str`
    #[pest_ast(outer(with(span_into_str), with(str::to_string)))]
    decoded: String
}

impl IdentifierName {
    pub fn string_value(&self) -> String {
        self.decoded.clone()
    }
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::OptionalChainingPunctuator))]
pub struct OptionalChainingPunctuator;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Addition))]
pub struct Addition;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::AdditionAssignment))]
pub struct AdditionAssignment;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::And))]
pub struct And;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::AndAssignment))]
pub struct AndAssignment;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Assignment))]
pub struct Assignment;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::BitAnd))]
pub struct BitAnd;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::BitAndAssignment))]
pub struct BitAndAssignment;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::BitNot))]
pub struct BitNot;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::BitOr))]
pub struct BitOr;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::BitOrAssignment))]
pub struct BitOrAssignment;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::BitXor))]
pub struct BitXor;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::BitXorAssignment))]
pub struct BitXorAssignment;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::ClosingBracket))]
pub struct ClosingBracket;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::ClosingParenthesis))]
pub struct ClosingParenthesis;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Colon))]
pub struct Colon;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Comma))]
pub struct Comma;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Decrement))]
pub struct Decrement;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Dot))]
pub struct Dot;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Ellipsis))]
pub struct Ellipsis;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Exponentiation))]
pub struct Exponentiation;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::ExponentiationAssignment))]
pub struct ExponentiationAssignment;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::FunctionArrow))]
pub struct FunctionArrow;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Increment))]
pub struct Increment;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::LeftShift))]
pub struct LeftShift;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::LeftShiftAssignment))]
pub struct LeftShiftAssignment;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Less))]
pub struct Less;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::LessOrEqual))]
pub struct LessOrEqual;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::LooseEquality))]
pub struct LooseEquality;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::LooseInequality))]
pub struct LooseInequality;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Modulo))]
pub struct Modulo;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::ModuloAssignment))]
pub struct ModuloAssignment;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::More))]
pub struct More;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::MoreOrEqual))]
pub struct MoreOrEqual;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Multiplication))]
pub struct Multiplication;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::MultiplicationAssignment))]
pub struct MultiplicationAssignment;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Not))]
pub struct Not;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::NullishCoalescence))]
pub struct NullishCoalescence;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::NullishCoalescenceAssignment))]
pub struct NullishCoalescenceAssignment;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::OpeningBrace))]
pub struct OpeningBrace;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::OpeningBracket))]
pub struct OpeningBracket;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::OpeningParenthesis))]
pub struct OpeningParenthesis;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Or))]
pub struct Or;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::OrAssignment))]
pub struct OrAssignment;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::QuestionMark))]
pub struct QuestionMark;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::RightShift))]
pub struct RightShift;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::RightShiftAssignment))]
pub struct RightShiftAssignment;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Semicolon))]
pub struct Semicolon;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::StrictEquality))]
pub struct StrictEquality;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::StrictInequality))]
pub struct StrictInequality;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Subtraction))]
pub struct Subtraction;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::SubtractionAssignment))]
pub struct SubtractionAssignment;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::UnsignedRightShift))]
pub struct UnsignedRightShift;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::UnsignedRightShiftAssignment))]
pub struct UnsignedRightShiftAssignment;

#[derive(Debug, FromPest)]
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

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Punctuator))]
pub enum Punctuator {
    OptionalChainingPunctuator(OptionalChainingPunctuator),
    OtherPunctuator(OtherPunctuator),
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::CommonToken))]
pub enum CommonToken {
    IdentifierName(IdentifierName),
    PrivateIdentifier(PrivateIdentifier),
    Punctuator(Punctuator),
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Await))]
pub struct Await;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Break))]
pub struct Break;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Case))]
pub struct Case;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Catch))]
pub struct Catch;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Class))]
pub struct Class;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Const))]
pub struct Const;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Continue))]
pub struct Continue;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Debugger))]
pub struct Debugger;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Default))]
pub struct Default;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Delete))]
pub struct Delete;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Do))]
pub struct Do;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Else))]
pub struct Else;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Enum))]
pub struct Enum;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Export))]
pub struct Export;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Extends))]
pub struct Extends;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::False))]
pub struct False;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Finally))]
pub struct Finally;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::For))]
pub struct For;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Function))]
pub struct Function;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::If))]
pub struct If;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Import))]
pub struct Import;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::In))]
pub struct In;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Instanceof))]
pub struct Instanceof;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::New))]
pub struct New;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Null))]
pub struct Null;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Return))]
pub struct Return;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Super))]
pub struct Super;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Switch))]
pub struct Switch;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::This))]
pub struct This;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Throw))]
pub struct Throw;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::True))]
pub struct True;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Try))]
pub struct Try;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Typeof))]
pub struct Typeof;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Var))]
pub struct Var;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Void))]
pub struct Void;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::While))]
pub struct While;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::With))]
pub struct With;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Yield))]
pub struct Yield;

#[derive(Debug, FromPest)]
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

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::RightBracePunctuator))]
pub struct RightBracePunctuator;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::DivisionAssignment))]
pub struct DivisionAssignment;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Division))]
pub struct Division;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::DivPunctuator))]
pub enum DivPunctuator {
    DivisionAssignment(DivisionAssignment),
    Division(Division),
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::Comment))]
pub enum Comment {
    MultiLineComment(MultiLineComment),
    SingleLineComment(SingleLineComment),
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::MultiLineComment))]
pub struct MultiLineComment;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::SingleLineComment))]
pub struct SingleLineComment;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::HashbangComment))]
pub struct HashbangComment<'src> {
     #[pest_ast(outer(with(span_into_str)))]
    pub content: &'src str,
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
