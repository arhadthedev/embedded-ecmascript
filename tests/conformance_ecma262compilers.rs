use rstest::rstest;
use std::fs::read_to_string;
use std::path::PathBuf;

fn check_if_readable(path: PathBuf) {
    read_to_string(path).unwrap();
}

fn get_explicit_variant(default_variant: &Path) -> PathBuf {
    let mut new_variant = default_variant.canonicalize().unwrap();
    new_variant.pop();
    new_variant.pop();
    new_variant.push("pass-explicit");
    new_variant.push(default_variant.file_name().unwrap());
    new_variant
}

#[rstest]
fn script_pass(
    #[files("tests/_data/test262-parser-tests/pass/*.js")]
    path: PathBuf,
) {
    check_if_readable(get_explicit_variant(&path));
    check_if_readable(path);
}

#[rstest]
fn script_fail(
    #[files("tests/_data/test262-parser-tests/fail/*.js")]
    path: PathBuf,
) {
    check_if_readable(path);
}

#[rstest]
fn script_early_error(
    #[files("tests/_data/test262-parser-tests/early/*.js")]
    path: PathBuf,
) {
    check_if_readable(path);
}
