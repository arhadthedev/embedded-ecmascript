//! Tokenizer of `.js` and `.mjs` files.
//! 
//! Implements <https://262.ecma-international.org/14.0/#sec-ecmascript-language-lexical-grammar>.
//!
//! Note: Tokenization in ECMAScript is highly context-dependend so we cannot
//! make this class public for a user; they would need to create their own
//! parser to timely switch sets of lexical grammars.

pub mod names;
pub mod punctuators;
pub mod space;

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    #[derive(Debug)]
    pub struct TestCase {
        pub input: String,
        pub expected_tail: Option<String>,
    }

    pub fn generate_cases(input: &str, sep: &str) -> Vec<TestCase> {
        vec![
            // Empty strings do not match
            TestCase{input: String::new(), expected_tail: None},

            // Non-matching strings do not match
            TestCase{input: sep.to_string(), expected_tail: None},

            // Match in start of the string only
            TestCase{input: format!("{sep}{input}"), expected_tail: None},

            // EOF match
            TestCase{input: input.to_string(), expected_tail: Some(String::new())},

            // Non-EOF match
            TestCase{input: format!("{input}{sep}"), expected_tail: Some(sep.to_string())},

            // Head-to-tail repetition
            TestCase{
                input: format!("{input}{input}"),
                expected_tail: Some(input.to_string())
            },

            // Intervined repetition
            TestCase{
                input: format!("{input}{sep}{input}"),
                expected_tail: Some(format!("{sep}{input}"))
            },
        ]
    }

    pub fn assert_match_tail<ParsedNode>(
        checked: Option<(ParsedNode, &str)>,
        reference_tail: &Option<String>
    ) {
        assert_eq!(
            checked.map(|result| result.1.to_string()),
            reference_tail.clone()
        );
    }

    /// A test case for a parser, creatable from a literal the parser
    /// is documented to process.
    ///
    /// The creation is performed in [`TerminalCase.from_str`] and invoked
    /// by the `#[values("\u{...}, ...)]` macro provided by rstest.
    pub struct TerminalCase {
        pub terminal: String,
        pub parser: fn(&str) -> Option<((), &str)>
    }

    pub struct CaseParameterError;

    const fn return_none(_: &str) -> Option<((), &str)> {
        None
    }

    impl FromStr for TerminalCase {
        type Err = CaseParameterError;

        fn from_str(text: &str) -> Result<Self, Self::Err> {
            // Keep the arms unmerged for proper sorting of disjoined patterns.
            #[allow(clippy::match_same_arms)]
            let tested_parser = match text {
                "\u{0009}" => super::space::match_tab,
                "\u{000A}" => super::space::match_lf,
                "\u{000B}" => super::space::match_vt,
                "\u{000C}" => super::space::match_ff,
                "\u{000D}" => super::space::match_cr,
                "\u{0020}" => super::space::match_usp,
                "/" | "/=" => super::punctuators::match_div_punctuator,
                "}" => super::punctuators::match_right_brace_punctuator,
                "\u{00A0}" => super::space::match_usp,
                "\u{1680}" => super::space::match_usp,
                "\u{2000}" | "\u{2001}" | "\u{2002}" | "\u{2003}" |
                "\u{2004}" | "\u{2005}" | "\u{2006}" | "\u{2007}" |
                "\u{2008}" | "\u{2009}" | "\u{200A}" => super::space::match_usp,
                "\u{200C}" => super::names::match_zwnj,
                "\u{200D}" => super::names::match_zwj,
                "\u{2028}" => super::space::match_ls,
                "\u{2029}" => super::space::match_ps,
                "\u{202F}" => super::space::match_usp,
                "\u{205F}" => super::space::match_usp,
                "\u{3000}" => super::space::match_usp,
                "\u{FEFF}" => super::space::match_zwnbsp,
                _ => return_none
            };
            Ok(Self {
                terminal: text.to_string(),
                parser: tested_parser
            })
        }
    }
}
