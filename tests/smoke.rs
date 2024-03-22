use embedded_ecmascript::hello;

#[test]
fn test_import() -> Result<(), Box<dyn std::error::Error>> {
    assert!(hello());
    Ok(())
}
