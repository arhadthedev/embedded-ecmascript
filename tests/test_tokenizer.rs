#[cfg(test)]
mod tests {
    use claims::{assert_err, assert_matches, assert_ok_eq};
    use embedded_ecmascript::{
        get_next_token,
        GoalSymbols,
        lexical_grammar::{
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
            CommonToken,
            Decrement,
            Division,
            DivisionAssignment,
            DivPunctuator,
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
            LineTerminator,
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
            OptionalChainingPunctuator,
            Or,
            OrAssignment,
            OtherPunctuator,
            Punctuator,
            RightBracePunctuator,
            RightShift,
            RightShiftAssignment,
            StrictEquality,
            StrictInequality,
            SubtractionAssignment,
            QuestionMark,
            Semicolon,
            Subtraction,
            UnsignedRightShift,
            UnsignedRightShiftAssignment,
            WhiteSpace,
        },
        Token,
    };
    use rstest::rstest;

    #[rstest]
    fn test_error_infrastructure(
        #[values(
            GoalSymbols::InputElementHashbangOrRegExp,
            GoalSymbols::InputElementRegExpOrTemplateTail,
            GoalSymbols::InputElementRegExp,
            GoalSymbols::InputElementTemplateTail,
            GoalSymbols::InputElementDiv,
        )]
        mode: GoalSymbols,
    ) {
        assert_matches!(get_next_token("`", mode), Err(message) if !message.is_empty());
    }

    #[rstest]
    fn test_whitespace(
        #[values(
            "\u{FEFF}", "\t", "\u{000B}", "\u{000C}", " ",
            "\u{00A0}", "\u{1680}", "\u{2000}", "\u{2001}", "\u{2002}",
            "\u{2003}", "\u{2004}", "\u{2005}", "\u{2006}", "\u{2007}",
            "\u{2008}", "\u{2009}", "\u{200A}", "\u{202F}", "\u{205F}",
            "\u{3000}",
        )]
        tested: &str,
        #[values(
            GoalSymbols::InputElementHashbangOrRegExp,
            GoalSymbols::InputElementRegExpOrTemplateTail,
            GoalSymbols::InputElementRegExp,
            GoalSymbols::InputElementTemplateTail,
            GoalSymbols::InputElementDiv,
        )]
        mode: GoalSymbols,
    ) {
        assert_ok_eq!(get_next_token(tested, mode), (Token::WhiteSpace(WhiteSpace), ""));
    }

    #[rstest]
    fn test_line_terminator(
        #[values("\r", "\n", "\u{2028}", "\u{2029}")]
        tested: &str,
        #[values(
            GoalSymbols::InputElementHashbangOrRegExp,
            GoalSymbols::InputElementRegExpOrTemplateTail,
            GoalSymbols::InputElementRegExp,
            GoalSymbols::InputElementTemplateTail,
            GoalSymbols::InputElementDiv,
        )]
        mode: GoalSymbols,
    ) {
        assert_ok_eq!(get_next_token(tested, mode), (Token::LineTerminator(LineTerminator), ""));
    }

    #[rstest]
    fn test_line_terminator_special(
        #[values(
            GoalSymbols::InputElementHashbangOrRegExp,
            GoalSymbols::InputElementRegExpOrTemplateTail,
            GoalSymbols::InputElementRegExp,
            GoalSymbols::InputElementTemplateTail,
            GoalSymbols::InputElementDiv,
        )]
        mode: GoalSymbols,
    ) {
        // The parser consumes `\r\n` as string literal line continuation only.
        // See how `LineTerminator` and `LineTerminatorSequence` grammar rules
        // are defined and used in ECMA-262.
        assert_ok_eq!(get_next_token("\r\n", mode), (Token::LineTerminator(LineTerminator), "\n"));
    }

    #[rstest]
    fn test_identifier_name(
        #[values("X", "d", "д", "大", "$")]
        tested: &str,
        #[values(
            GoalSymbols::InputElementHashbangOrRegExp,
            GoalSymbols::InputElementRegExpOrTemplateTail,
            GoalSymbols::InputElementRegExp,
            GoalSymbols::InputElementTemplateTail,
            GoalSymbols::InputElementDiv,
        )]
        mode: GoalSymbols,
    ) {
        assert_matches!(
            get_next_token(tested, mode),
            Ok((Token::CommonToken(CommonToken::IdentifierName(name)), "")) if name.string_value() == tested
        );
        let doubled = tested.to_owned() + tested;
        assert_matches!(
            get_next_token(&doubled, mode),
            Ok((Token::CommonToken(CommonToken::IdentifierName(name)), "")) if name.string_value() == doubled
        );

        let private = "#".to_owned() + tested;
        assert_matches!(
            get_next_token(&private, mode),
            Ok((Token::CommonToken(CommonToken::PrivateIdentifier(name)), "")) if name.string_value() == private
        );
        let doubled_private = private + tested;
        assert_matches!(
            get_next_token(&doubled_private, mode),
            Ok((Token::CommonToken(CommonToken::PrivateIdentifier(name)), "")) if name.string_value() == doubled_private
        );
    }

    #[rstest]
    fn testreserved_word(
        #[values(
            "await", "break", "case", "catch", "class", "const", "continue",
            "debugger", "default", "delete", "do", "else", "enum", "export",
            "extends", "false", "finally", "for", "function", "if", "import",
            "in", "instanceof", "new", "null", "return", "super", "switch",
            "this", "throw", "true", "try", "typeof", "var", "void", "while",
            "with", "yield",
        )]
        tested: &str,
        #[values(
            GoalSymbols::InputElementHashbangOrRegExp,
            GoalSymbols::InputElementRegExpOrTemplateTail,
            GoalSymbols::InputElementRegExp,
            GoalSymbols::InputElementTemplateTail,
            GoalSymbols::InputElementDiv,
        )]
        mode: GoalSymbols,
    ) {
        assert_matches!(
            get_next_token(tested, mode),
            Ok((Token::ReservedWord(_), ""))
        );
    }

    #[rstest]
    fn test_common_onechar_punctuators(
        #[values(
            GoalSymbols::InputElementHashbangOrRegExp,
            GoalSymbols::InputElementRegExpOrTemplateTail,
            GoalSymbols::InputElementRegExp,
            GoalSymbols::InputElementTemplateTail,
            GoalSymbols::InputElementDiv,
        )]
        mode: GoalSymbols,
    ) {
        assert_ok_eq!(get_next_token("{", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::OpeningBrace(OpeningBrace)))), ""));
        assert_ok_eq!(get_next_token("(", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::OpeningParenthesis(OpeningParenthesis)))), ""));
        assert_ok_eq!(get_next_token(")", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::ClosingParenthesis(ClosingParenthesis)))), ""));
        assert_ok_eq!(get_next_token("[", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::OpeningBracket(OpeningBracket)))), ""));
        assert_ok_eq!(get_next_token("]", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::ClosingBracket(ClosingBracket)))), ""));
        assert_ok_eq!(get_next_token(".", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::Dot(Dot)))), ""));
        assert_ok_eq!(get_next_token(";", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::Semicolon(Semicolon)))), ""));
        assert_ok_eq!(get_next_token(",", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::Comma(Comma)))), ""));
        assert_ok_eq!(get_next_token("<", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::Less(Less)))), ""));
        assert_ok_eq!(get_next_token(">", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::More(More)))), ""));
        assert_ok_eq!(get_next_token("+", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::Addition(Addition)))), ""));
        assert_ok_eq!(get_next_token("-", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::Subtraction(Subtraction)))), ""));
        assert_ok_eq!(get_next_token("*", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::Multiplication(Multiplication)))), ""));
        assert_ok_eq!(get_next_token("%", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::Modulo(Modulo)))), ""));
        assert_ok_eq!(get_next_token("&", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::BitAnd(BitAnd)))), ""));
        assert_ok_eq!(get_next_token("|", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::BitOr(BitOr)))), ""));
        assert_ok_eq!(get_next_token("^", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::BitXor(BitXor)))), ""));
        assert_ok_eq!(get_next_token("!", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::Not(Not)))), ""));
        assert_ok_eq!(get_next_token("~", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::BitNot(BitNot)))), ""));
        assert_ok_eq!(get_next_token("?", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::QuestionMark(QuestionMark)))), ""));
        assert_ok_eq!(get_next_token(":", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::Colon(Colon)))), ""));
        assert_ok_eq!(get_next_token("=", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::Assignment(Assignment)))), ""));
    }

    #[test]
    fn test_input_element_div_onechar_punctuators() {
        assert_ok_eq!(
            get_next_token("/", GoalSymbols::InputElementDiv),
            (Token::DivPunctuator(DivPunctuator::Division(Division)), "")
        );
        assert_err!(get_next_token("/", GoalSymbols::InputElementHashbangOrRegExp));
        assert_err!(get_next_token("/", GoalSymbols::InputElementRegExpOrTemplateTail));
        assert_err!(get_next_token("/", GoalSymbols::InputElementRegExp));
        assert_ok_eq!(
            get_next_token("/", GoalSymbols::InputElementTemplateTail),
            (Token::DivPunctuator(DivPunctuator::Division(Division)), "")
        );

        assert_ok_eq!(
            get_next_token("}", GoalSymbols::InputElementDiv),
            (Token::RightBracePunctuator(RightBracePunctuator), "")
        );
        assert_err!(get_next_token("}", GoalSymbols::InputElementHashbangOrRegExp));
        assert_err!(get_next_token("}", GoalSymbols::InputElementRegExpOrTemplateTail));
        assert_ok_eq!(
            get_next_token("}", GoalSymbols::InputElementRegExp),
            (Token::RightBracePunctuator(RightBracePunctuator), "")
        );
        assert_err!(get_next_token("}", GoalSymbols::InputElementTemplateTail));

    }

    #[rstest]
    fn test_common_twochar_punctuators(
        #[values(
            GoalSymbols::InputElementHashbangOrRegExp,
            GoalSymbols::InputElementRegExpOrTemplateTail,
            GoalSymbols::InputElementRegExp,
            GoalSymbols::InputElementTemplateTail,
            GoalSymbols::InputElementDiv,
        )]
        mode: GoalSymbols,
    ) {
        assert_ok_eq!(get_next_token("?.", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OptionalChainingPunctuator(OptionalChainingPunctuator))), ""));
        assert_ok_eq!(get_next_token("<=", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::LessOrEqual(LessOrEqual)))), ""));
        assert_ok_eq!(get_next_token(">=", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::MoreOrEqual(MoreOrEqual)))), ""));
        assert_ok_eq!(get_next_token("==", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::LooseEquality(LooseEquality)))), ""));
        assert_ok_eq!(get_next_token("!=", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::LooseInequality(LooseInequality)))), ""));
        assert_ok_eq!(get_next_token("**", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::Exponentiation(Exponentiation)))), ""));
        assert_ok_eq!(get_next_token("++", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::Increment(Increment)))), ""));
        assert_ok_eq!(get_next_token("--", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::Decrement(Decrement)))), ""));
        assert_ok_eq!(get_next_token("<<", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::LeftShift(LeftShift)))), ""));
        assert_ok_eq!(get_next_token(">>", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::RightShift(RightShift)))), ""));
        assert_ok_eq!(get_next_token("&&", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::And(And)))), ""));
        assert_ok_eq!(get_next_token("||", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::Or(Or)))), ""));
        assert_ok_eq!(
            get_next_token("??", mode),
            (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::NullishCoalescence(NullishCoalescence)))), "")
        );
        assert_ok_eq!(
            get_next_token("+=", mode),
            (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::AdditionAssignment(AdditionAssignment)))), "")
        );
        assert_ok_eq!(
            get_next_token("-=", mode),
            (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::SubtractionAssignment(SubtractionAssignment)))), "")
        );
        assert_ok_eq!(
            get_next_token("*=", mode),
            (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::MultiplicationAssignment(MultiplicationAssignment)))), "")
        );
        assert_ok_eq!(get_next_token("%=", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::ModuloAssignment(ModuloAssignment)))), ""));
        assert_ok_eq!(get_next_token("&=", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::BitAndAssignment(BitAndAssignment)))), ""));
        assert_ok_eq!(get_next_token("|=", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::BitOrAssignment(BitOrAssignment)))), ""));
        assert_ok_eq!(get_next_token("^=", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::BitXorAssignment(BitXorAssignment)))), ""));
        assert_ok_eq!(get_next_token("=>", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::FunctionArrow(FunctionArrow)))), ""));
    }

    #[test]
    fn test_input_element_div_twochar_punctuators() {
        assert_ok_eq!(
            get_next_token("/=", GoalSymbols::InputElementDiv),
            (Token::DivPunctuator(DivPunctuator::DivisionAssignment(DivisionAssignment)), "")
        );
        assert_err!(get_next_token("/=", GoalSymbols::InputElementHashbangOrRegExp));
        assert_err!(get_next_token("/=", GoalSymbols::InputElementRegExpOrTemplateTail));
        assert_err!(get_next_token("/=", GoalSymbols::InputElementRegExp));
        assert_ok_eq!(
            get_next_token("/=", GoalSymbols::InputElementTemplateTail),
            (Token::DivPunctuator(DivPunctuator::DivisionAssignment(DivisionAssignment)), "")
        );
    }

    #[rstest]
    fn test_threechar_punctuators(
        #[values(
            GoalSymbols::InputElementHashbangOrRegExp,
            GoalSymbols::InputElementRegExpOrTemplateTail,
            GoalSymbols::InputElementRegExp,
            GoalSymbols::InputElementTemplateTail,
            GoalSymbols::InputElementDiv,
        )]
        mode: GoalSymbols,
    ) {
        assert_ok_eq!(get_next_token("...", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::Ellipsis(Ellipsis)))), ""));
        assert_ok_eq!(get_next_token("===", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::StrictEquality(StrictEquality)))), ""));
        assert_ok_eq!(
            get_next_token("!==", mode),
            (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::StrictInequality(StrictInequality)))), "")
        );
        assert_ok_eq!(
            get_next_token(">>>", mode),
            (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::UnsignedRightShift(UnsignedRightShift)))), "")
        );
        assert_ok_eq!(
            get_next_token("**=", mode),
            (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::ExponentiationAssignment(ExponentiationAssignment)))), "")
        );
        assert_ok_eq!(
            get_next_token("<<=", mode),
            (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::LeftShiftAssignment(LeftShiftAssignment)))), "")
        );
        assert_ok_eq!(
            get_next_token(">>=", mode),
            (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::RightShiftAssignment(RightShiftAssignment)))), "")
        );
        assert_ok_eq!(get_next_token("&&=", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::AndAssignment(AndAssignment)))), ""));
        assert_ok_eq!(get_next_token("||=", mode), (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::OrAssignment(OrAssignment)))), ""));
        assert_ok_eq!(
            get_next_token("??=", mode),
            (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::NullishCoalescenceAssignment(NullishCoalescenceAssignment)))), "")
        );
    }

    #[rstest]
    fn test_fourchar_punctuators(
        #[values(
            GoalSymbols::InputElementHashbangOrRegExp,
            GoalSymbols::InputElementRegExpOrTemplateTail,
            GoalSymbols::InputElementRegExp,
            GoalSymbols::InputElementTemplateTail,
            GoalSymbols::InputElementDiv,
        )]
        mode: GoalSymbols,
    ) {
        assert_ok_eq!(
            get_next_token(">>>=", mode),
            (Token::CommonToken(CommonToken::Punctuator(Punctuator::OtherPunctuator(OtherPunctuator::UnsignedRightShiftAssignment(UnsignedRightShiftAssignment)))), "")
        );
    }

    #[rstest]
    fn test_multiline_comments(
        #[values(
            GoalSymbols::InputElementHashbangOrRegExp,
            GoalSymbols::InputElementRegExpOrTemplateTail,
            GoalSymbols::InputElementRegExp,
            GoalSymbols::InputElementTemplateTail,
            GoalSymbols::InputElementDiv,
        )]
        mode: GoalSymbols,
    ) {
        assert_matches!(get_next_token("/**/", mode), Ok((Token::Comment(_), "")));
        assert_matches!(get_next_token("/* */", mode), Ok((Token::Comment(_), "")));
        assert_matches!(get_next_token("/*foo*/", mode), Ok((Token::Comment(_), "")));
        assert_matches!(get_next_token("/*/**/", mode), Ok((Token::Comment(_), "")));
        assert_matches!(get_next_token("/*\n/*\n*/", mode), Ok((Token::Comment(_), "")));
    }

    #[rstest]
    fn test_single_line_comments(
        #[values(
            GoalSymbols::InputElementHashbangOrRegExp,
            GoalSymbols::InputElementRegExpOrTemplateTail,
            GoalSymbols::InputElementRegExp,
            GoalSymbols::InputElementTemplateTail,
            GoalSymbols::InputElementDiv,
        )]
        mode: GoalSymbols,
    ) {
        assert_matches!(get_next_token("//a b", mode), Ok((Token::Comment(_), "")));
        assert_matches!(get_next_token("//a b\n", mode), Ok((Token::Comment(_), "\n")));
        assert_matches!(
            get_next_token("//a b\n//c", mode),
            Ok((Token::Comment(_), "\n//c"))
        );
    }

    #[test]
    fn test_hashbang_comments() {
        fn get_token(input: &str) -> Result<(Token, &str), String> {
            get_next_token(input, GoalSymbols::InputElementHashbangOrRegExp)
        }

        assert_matches!(
            get_token("#!foo"),
            Ok((Token::HashbangComment(body), "")) if body.string_value() == "foo"
        );
        assert_matches!(
            get_token("#!foo\n"),
            Ok((Token::HashbangComment(body), "\n")) if body.string_value() == "foo"
        );
        assert_matches!(
            get_token("#!foo\r\n"),
            Ok((Token::HashbangComment(body), "\r\n")) if body.string_value() == "foo"
        );
        assert_matches!(
            get_token("#!foo\n\n"),
            Ok((Token::HashbangComment(body), "\n\n")) if body.string_value() == "foo"
        );
        assert_matches!(
            get_token("#!"),
            Ok((Token::HashbangComment(body), "")) if body.string_value().is_empty()
        );
        assert_matches!(
            get_token("#!\n"),
            Ok((Token::HashbangComment(body), "\n")) if body.string_value().is_empty()
        );
        assert_matches!(
            get_token("#!\n\n"),
            Ok((Token::HashbangComment(body), "\n\n")) if body.string_value().is_empty()
        );
    }

    #[rstest]
    fn test_hashbang_comments_errors(
        #[values(
            GoalSymbols::InputElementRegExpOrTemplateTail,
            GoalSymbols::InputElementRegExp,
            GoalSymbols::InputElementTemplateTail,
            GoalSymbols::InputElementDiv,
        )]
        mode: GoalSymbols,
    ) {
        assert_err!(get_next_token("#!foo", mode));
        assert_err!(get_next_token("#!foo\n", mode));
        assert_err!(get_next_token("#!", mode));
        assert_err!(get_next_token("#!\n", mode));
    }
}
