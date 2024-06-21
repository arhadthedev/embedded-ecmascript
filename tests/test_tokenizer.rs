#[cfg(test)]
mod tests {
    use claims::{assert_matches, assert_ok_eq};
    use embedded_ecmascript::{get_next_token, GoalSymbols, Token};
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
        assert_ok_eq!(get_next_token(tested, mode), (Token::WhiteSpace, ""));
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
        assert_ok_eq!(get_next_token(tested, mode), (Token::LineTerminator, ""));
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
        assert_ok_eq!(get_next_token("\r\n", mode), (Token::LineTerminator, "\n"));
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
        assert_ok_eq!(
            get_next_token(tested, mode),
            (Token::IdentifierName(tested.to_owned()), "")
        );
        let doubled = tested.to_owned() + tested;
        assert_ok_eq!(
            get_next_token(&doubled, mode),
            (Token::IdentifierName(doubled.clone()), "")
        );

        let private = "#".to_owned() + tested;
        assert_ok_eq!(
            get_next_token(&private, mode),
            (Token::PrivateIdentifier(tested.to_owned()), "")
        );
        let doubled_private = private + tested;
        assert_ok_eq!(
            get_next_token(&doubled_private, mode),
            (Token::PrivateIdentifier(doubled), "")
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
        assert_ok_eq!(get_next_token("{", mode), (Token::OpeningBrace, ""));
        assert_ok_eq!(get_next_token("(", mode), (Token::OpeningParenthesis, ""));
        assert_ok_eq!(get_next_token(")", mode), (Token::ClosingParenthesis, ""));
        assert_ok_eq!(get_next_token("[", mode), (Token::OpeningBracket, ""));
        assert_ok_eq!(get_next_token("]", mode), (Token::ClosingBracket, ""));
        assert_ok_eq!(get_next_token(".", mode), (Token::Dot, ""));
        assert_ok_eq!(get_next_token(";", mode), (Token::Semicolon, ""));
        assert_ok_eq!(get_next_token(",", mode), (Token::Comma, ""));
        assert_ok_eq!(get_next_token("<", mode), (Token::Less, ""));
        assert_ok_eq!(get_next_token(">", mode), (Token::More, ""));
        assert_ok_eq!(get_next_token("+", mode), (Token::Addition, ""));
        assert_ok_eq!(get_next_token("-", mode), (Token::Subtraction, ""));
        assert_ok_eq!(get_next_token("*", mode), (Token::Multiplication, ""));
        assert_ok_eq!(get_next_token("%", mode), (Token::Modulo, ""));
        assert_ok_eq!(get_next_token("&", mode), (Token::BitAnd, ""));
        assert_ok_eq!(get_next_token("|", mode), (Token::BitOr, ""));
        assert_ok_eq!(get_next_token("^", mode), (Token::BitXor, ""));
        assert_ok_eq!(get_next_token("!", mode), (Token::Not, ""));
        assert_ok_eq!(get_next_token("~", mode), (Token::BitNot, ""));
        assert_ok_eq!(get_next_token("?", mode), (Token::QuestionMark, ""));
        assert_ok_eq!(get_next_token(":", mode), (Token::Colon, ""));
        assert_ok_eq!(get_next_token("=", mode), (Token::Assignment, ""));
    }

    #[rstest]
    fn test_input_element_div_onechar_punctuators() {
        assert_ok_eq!(
            get_next_token("/", GoalSymbols::InputElementDiv),
            (Token::Division, "")
        );
        assert_ok_eq!(
            get_next_token("}", GoalSymbols::InputElementDiv),
            (Token::ClosingBrace, "")
        );
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
        assert_ok_eq!(get_next_token("?.", mode), (Token::OptionalChaining, ""));
        assert_ok_eq!(get_next_token("<=", mode), (Token::LessOrEqual, ""));
        assert_ok_eq!(get_next_token(">=", mode), (Token::MoreOrEqual, ""));
        assert_ok_eq!(get_next_token("==", mode), (Token::LooseEquality, ""));
        assert_ok_eq!(get_next_token("!=", mode), (Token::LooseInequality, ""));
        assert_ok_eq!(get_next_token("**", mode), (Token::Exponentiation, ""));
        assert_ok_eq!(get_next_token("++", mode), (Token::Increment, ""));
        assert_ok_eq!(get_next_token("--", mode), (Token::Decrement, ""));
        assert_ok_eq!(get_next_token("<<", mode), (Token::LeftShift, ""));
        assert_ok_eq!(get_next_token(">>", mode), (Token::RightShift, ""));
        assert_ok_eq!(get_next_token("&&", mode), (Token::And, ""));
        assert_ok_eq!(get_next_token("||", mode), (Token::Or, ""));
        assert_ok_eq!(
            get_next_token("??", mode),
            (Token::NullishCoalescence, "")
        );
        assert_ok_eq!(
            get_next_token("+=", mode),
            (Token::AdditionAssignment, "")
        );
        assert_ok_eq!(
            get_next_token("-=", mode),
            (Token::SubtractionAssignment, "")
        );
        assert_ok_eq!(
            get_next_token("*=", mode),
            (Token::MultiplicationAssignment, "")
        );
        assert_ok_eq!(get_next_token("%=", mode), (Token::ModuloAssignment, ""));
        assert_ok_eq!(get_next_token("&=", mode), (Token::BitAndAssignment, ""));
        assert_ok_eq!(get_next_token("|=", mode), (Token::BitOrAssignment, ""));
        assert_ok_eq!(get_next_token("^=", mode), (Token::BitXorAssignment, ""));
        assert_ok_eq!(get_next_token("=>", mode), (Token::FunctionArrow, ""));
    }

    #[rstest]
    fn test_input_element_div_twochar_punctuators() {
        assert_ok_eq!(
            get_next_token("/=", GoalSymbols::InputElementDiv),
            (Token::DivisionAssignment, "")
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
        assert_ok_eq!(get_next_token("...", mode), (Token::Ellipsis, ""));
        assert_ok_eq!(get_next_token("===", mode), (Token::StrictEquality, ""));
        assert_ok_eq!(
            get_next_token("!==", mode),
            (Token::StrictInequality, "")
        );
        assert_ok_eq!(
            get_next_token(">>>", mode),
            (Token::UnsignedRightShift, "")
        );
        assert_ok_eq!(
            get_next_token("**=", mode),
            (Token::ExponentiationAssignment, "")
        );
        assert_ok_eq!(
            get_next_token("<<=", mode),
            (Token::LeftShiftAssignment, "")
        );
        assert_ok_eq!(
            get_next_token(">>=", mode),
            (Token::RightShiftAssignment, "")
        );
        assert_ok_eq!(get_next_token("&&=", mode), (Token::AndAssignment, ""));
        assert_ok_eq!(get_next_token("||=", mode), (Token::OrAssignment, ""));
        assert_ok_eq!(
            get_next_token("??=", mode),
            (Token::NullishCoalescenceAssignment, "")
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
            (Token::UnsignedRightShiftAssignment, "")
        );
    }

    #[rstest]
    fn match_decimal_digit(
        #[values("0", "1", "2", "3", "4", "5", "6", "7", "8", "9")]
        tested: &str,
    ) {
        let parsed = Token::NumericLiteral(tested.parse().unwrap());

        assert_ok_eq!(
            get_next_token(tested, GoalSymbols::InputElementDiv),
            (parsed.clone(), "")
        );

        let tail = " ".to_owned() + tested;
        let with_tail = tested.to_owned() + &tail;
        assert_ok_eq!(
            get_next_token(&with_tail, GoalSymbols::InputElementDiv),
            (parsed, tail.as_str())
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
        assert_ok_eq!(get_next_token("/**/", mode), (Token::Comment, ""));
        assert_ok_eq!(get_next_token("/* */", mode), (Token::Comment, ""));
        assert_ok_eq!(get_next_token("/*foo*/", mode), (Token::Comment, ""));
        assert_ok_eq!(get_next_token("/*/**/", mode), (Token::Comment, ""));
        assert_ok_eq!(get_next_token("/*\n/*\n*/", mode), (Token::Comment, ""));
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
        assert_ok_eq!(get_next_token("//a b", mode), (Token::Comment, ""));
        assert_ok_eq!(get_next_token("//a b\n", mode), (Token::Comment, "\n"));
        assert_ok_eq!(
            get_next_token("//a b\n//c", mode),
            (Token::Comment, "\n//c")
        );
    }
}
