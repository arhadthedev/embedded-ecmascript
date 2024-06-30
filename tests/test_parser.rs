#[cfg(test)]
mod tests {
    use embedded_ecmascript::parse;
    use rstest::rstest;

    #[rstest]
    fn test_token_recognition(#[values(false, true)] is_module: bool) {
        assert_eq!(parse(";", is_module), Ok(()));
    }
}
