#[cfg(test)]
mod tests {
    use claims::assert_matches;
    use embedded_ecmascript::{get_next_token, Token};
    use rstest::rstest;

    #[test]
    fn test_error_infrastructure() {
        assert_matches!(get_next_token(":"), Err(message) if !message.is_empty());
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
