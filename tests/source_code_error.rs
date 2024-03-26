use embedded_ecmascript::SourceCodeError;
use rstest::{fixture, rstest};
use std::error::Error;
use std::ops::Range;

#[fixture]
fn specimen() -> SourceCodeError {
    SourceCodeError {
        location: Range{start: 0, end: 100},
        message: "Some error".to_owned()
    }
}

#[rstest]
fn sourcecoderror_error_trait(specimen: SourceCodeError)
{
    assert!(specimen.source().is_none());

    // Explicitly check if we can return the error as Box<dyn std::error::Error>.
    //
    // This is useful for tests that use `Result.expect`. They are declared as:
    //
    // ```rust
    // #[test]
    // fn test_bin_runnable() -> Result<(), Box<dyn std::error::Error>> {
    //     // ...
    // }
    // ```
    let _: Box<dyn std::error::Error> = Box::new(specimen);
}

#[rstest]
fn sourcecoderror_display_trait(specimen: SourceCodeError)
{
    let message = format!("{}", specimen);
    assert_eq!(message, "error in characters #1-#100: Some error");
}
