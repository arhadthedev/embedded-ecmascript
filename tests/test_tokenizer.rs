#[cfg(test)]
mod tests {
    use embedded_ecmascript::{get_next_token, Token};
    use rstest::rstest;

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