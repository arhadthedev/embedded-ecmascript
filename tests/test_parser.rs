#[cfg(test)]
mod tests {
    use embedded_ecmascript::parse;

    #[test]
    fn test_token_recognition() {
        assert_eq!(parse(";"), Ok(()));
    }
}
