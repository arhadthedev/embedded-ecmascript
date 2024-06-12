#[cfg(test)]
mod tests {
    use claims::assert_matches;
    use embedded_ecmascript::{get_next_token, Token};
    use rstest::rstest;

    #[test]
    fn test_error_infrastructure() {
        assert_matches!(get_next_token("`"), Err(message) if !message.is_empty());
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
    ) {
        assert_matches!(get_next_token(tested), Ok((Token::WhiteSpace, "")));
    }

    #[rstest]
    fn test_line_terminator(
        #[values("\r", "\n", "\u{2028}", "\u{2029}")]
        tested: &str,
    ) {
        assert_matches!(get_next_token(tested), Ok((Token::LineTerminator, "")));
    }

    #[rstest]
    fn test_line_terminator_special() {
        // The parser consumes `\r\n` as string literal line continuation only.
        // See how `LineTerminator` and `LineTerminatorSequence` grammar rules
        // are defined and used in ECMA-262.
        assert_matches!(get_next_token("\r\n"), Ok((Token::LineTerminator, "\n")));
    }

    #[rstest]
    fn test_identifier_name(
        #[values("X", "d", "д", "大", "$")]
        tested: &str,
    ) {
        assert_eq!(
            get_next_token(tested),
            Ok((Token::IdentifierName(tested.to_owned()), ""))
        );
        let doubled = tested.to_owned() + tested;
        assert_eq!(
            get_next_token(&doubled),
            Ok((Token::IdentifierName(doubled.clone()), ""))
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
    ) {
        assert_matches!(get_next_token(tested), Ok((Token::ReservedWord(_), "")));
    }

    #[test]
    fn test_onechar_punctuators() {
        assert_matches!(get_next_token("{"), Ok((Token::OpeningBrace, "")));
        assert_matches!(get_next_token("("), Ok((Token::OpeningParenthesis, "")));
        assert_matches!(get_next_token(")"), Ok((Token::ClosingParenthesis, "")));
        assert_matches!(get_next_token("["), Ok((Token::OpeningBracket, "")));
        assert_matches!(get_next_token("]"), Ok((Token::ClosingBracket, "")));
        assert_matches!(get_next_token("."), Ok((Token::Dot, "")));
        assert_matches!(get_next_token(";"), Ok((Token::Semicolon, "")));
        assert_matches!(get_next_token(","), Ok((Token::Comma, "")));
        assert_matches!(get_next_token("<"), Ok((Token::Less, "")));
        assert_matches!(get_next_token(">"), Ok((Token::More, "")));
        assert_matches!(get_next_token("+"), Ok((Token::Addition, "")));
        assert_matches!(get_next_token("-"), Ok((Token::Subtraction, "")));
        assert_matches!(get_next_token("*"), Ok((Token::Multiplication, "")));
        assert_matches!(get_next_token("%"), Ok((Token::Modulo, "")));
        assert_matches!(get_next_token("&"), Ok((Token::BitAnd, "")));
        assert_matches!(get_next_token("|"), Ok((Token::BitOr, "")));
        assert_matches!(get_next_token("^"), Ok((Token::BitXor, "")));
        assert_matches!(get_next_token("!"), Ok((Token::Not, "")));
        assert_matches!(get_next_token("~"), Ok((Token::BitNot, "")));
        assert_matches!(get_next_token("?"), Ok((Token::QuestionMark, "")));
        assert_matches!(get_next_token(":"), Ok((Token::Colon, "")));
        assert_matches!(get_next_token("="), Ok((Token::Assignment, "")));
    }

    #[test]
    fn test_twochar_punctuators() {
        assert_matches!(get_next_token("?."), Ok((Token::OptionalChaining, "")));
        assert_matches!(get_next_token("<="), Ok((Token::LessOrEqual, "")));
        assert_matches!(get_next_token(">="), Ok((Token::MoreOrEqual, "")));
        assert_matches!(get_next_token("=="), Ok((Token::LooseEquality, "")));
        assert_matches!(get_next_token("!="), Ok((Token::LooseInequality, "")));
        assert_matches!(get_next_token("**"), Ok((Token::Exponentiation, "")));
        assert_matches!(get_next_token("++"), Ok((Token::Increment, "")));
        assert_matches!(get_next_token("--"), Ok((Token::Decrement, "")));
        assert_matches!(get_next_token("<<"), Ok((Token::LeftShift, "")));
        assert_matches!(get_next_token(">>"), Ok((Token::RightShift, "")));
        assert_matches!(get_next_token("&&"), Ok((Token::And, "")));
        assert_matches!(get_next_token("||"), Ok((Token::Or, "")));
        assert_matches!(get_next_token("??"), Ok((Token::NullishCoalescence, "")));
        assert_matches!(get_next_token("+="), Ok((Token::AdditionAssignment, "")));
        assert_matches!(get_next_token("-="), Ok((Token::SubtractionAssignment, "")));
        assert_matches!(get_next_token("*="), Ok((Token::MultiplicationAssignment, "")));
        assert_matches!(get_next_token("%="), Ok((Token::ModuloAssignment, "")));
        assert_matches!(get_next_token("&="), Ok((Token::BitAndAssignment, "")));
        assert_matches!(get_next_token("|="), Ok((Token::BitOrAssignment, "")));
        assert_matches!(get_next_token("^="), Ok((Token::BitXorAssignment, "")));
        assert_matches!(get_next_token("=>"), Ok((Token::FunctionArrow, "")));
    }

    #[test]
    fn test_threechar_punctuators() {
        assert_matches!(get_next_token("..."), Ok((Token::Ellipsis, "")));
        assert_matches!(get_next_token("==="), Ok((Token::StrictEquality, "")));
        assert_matches!(get_next_token("!=="), Ok((Token::StrictInequality, "")));
        assert_matches!(get_next_token(">>>"), Ok((Token::UnsignedRightShift, "")));
        assert_matches!(get_next_token("**="), Ok((Token::ExponentiationAssignment, "")));
        assert_matches!(get_next_token("<<="), Ok((Token::LeftShiftAssignment, "")));
        assert_matches!(get_next_token(">>="), Ok((Token::RightShiftAssignment, "")));
        assert_matches!(get_next_token("&&="), Ok((Token::AndAssignment, "")));
        assert_matches!(get_next_token("||="), Ok((Token::OrAssignment, "")));
        assert_matches!(get_next_token("??="), Ok((Token::NullishCoalescenceAssignment, "")));
    }

    #[test]
    fn test_fourchar_punctuators() {
        assert_matches!(get_next_token(">>>="), Ok((Token::UnsignedRightShiftAssignment, "")));
    }

    #[rstest]
    fn match_decimal_digit(
        #[values("0", "1", "2", "3", "4", "5", "6", "7", "8", "9")]
        tested: &str,
    ) {
        let parsed = Token::NumericLiteral(tested.parse().unwrap());

        assert_eq!(get_next_token(tested), Ok((parsed.clone(), "")));

        let tail = " ".to_owned() + tested;
        let with_tail = tested.to_owned() + &tail;
        assert_eq!(get_next_token(&with_tail), Ok((parsed, tail.as_str())));
    }
}
