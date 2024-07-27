#[cfg(test)]
mod tests {
    use embedded_ecmascript::grammar::parse;
    use rstest::rstest;

    #[rstest]
    fn test_simple_statements(#[values(false, true)] is_module: bool) {
        assert_eq!(parse(";", is_module), Ok(()));
    }
}
