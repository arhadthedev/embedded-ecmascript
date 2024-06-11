//! Tokenizer of `.js` and `.mjs` files.
//! 
//! Implements <https://262.ecma-international.org/14.0/#sec-ecmascript-language-lexical-grammar>.
//!
//! Note: Tokenization in ECMAScript is highly context-dependend so we cannot
//! make this class public for a user; they would need to create their own
//! parser to timely switch sets of lexical grammars.

pub mod punctuators;

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

    pub fn unwrap_tail<X>(parsed: Option<(X, &str)>) -> Option<String> {
        parsed.map(|(_, tail)| tail.to_string())
    }

    pub type WrappedParser = Box<dyn Fn(&str) -> Option<String>>;

    /// A test case for a parser to check how it splits the input string into
    /// a literal (ignored) and a tail (checked).
    ///
    /// The creation is performed in [`TerminalCase.from_str`] and invoked
    /// by the `#[values("\u{...}, ...)]` macro provided by rstest.
    pub struct TerminalCase {
        pub terminal: String,

        /// A wrapper that discards returned object leaving only a tail
        pub parser: WrappedParser
    }

    pub struct CaseParameterError;

    fn wrap<O, F: Fn(&str) -> Option<(O, &str)> + 'static>(callable: F)
        -> WrappedParser
    {
        Box::new(move |text| callable(text).map(|(_, tail)| tail.to_string()))
    }

    impl FromStr for TerminalCase {
        type Err = CaseParameterError;

        fn from_str(text: &str) -> Result<Self, Self::Err> {
            // Keep the arms unmerged for proper sorting of disjoined patterns.
            #[allow(clippy::match_same_arms)]
            let tested_parser = match text {
                "/" | "/=" => wrap(super::punctuators::match_div_punctuator),
                "?." => wrap(super::punctuators::match_optional_chaining_punctuator),
                "{" | "(" | ")" | "[" | "]" | "." | "..." | ";" | "," | "<" |
                ">" | "<=" | ">=" | "==" | "!=" | "===" | "!==" | "+" | "-" |
                "*" | "%" | "**" | "++" | "--" | "<<" | ">>" | ">>>" | "&" |
                "|" | "^" | "!" | "~" | "&&" | "||" | "??" | "?" | ":" | "=" |
                "+=" | "-=" | "*=" | "%=" | "**=" | "<<=" | ">>=" | ">>>=" |
                "&=" | "|=" | "^=" | "&&=" | "||=" | "??=" |
                "=>" => wrap(super::punctuators::match_other_punctuator),
                "}" => wrap(super::punctuators::match_right_brace_punctuator),
                _ => wrap(|_| None::<((), &str)>)
            };
            Ok(Self {
                terminal: text.to_string(),
                parser: tested_parser
            })
        }
    }
}
