#[cfg(test)]
mod tests {
    use embedded_ecmascript::parse;
    use rstest::rstest;

    #[rstest]
    fn test_simple_statements(#[values(false, true)] is_module: bool) {
        assert_eq!(parse(";", is_module), Ok(()));

        assert_eq!(parse("debug;", is_module), Ok(()));
    }
}
