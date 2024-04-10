//! Tokenizer of `.js` and `.mjs` files.
//! 
//! Implements <https://262.ecma-international.org/14.0/#sec-ecmascript-language-lexical-grammar>.
//!
//! Note: Tokenization in ECMAScript is highly context-dependend so we cannot
//! make this class public for a user; they would need to create their own
//! parser to timely switch sets of lexical grammars.

pub mod names;
pub mod numeric;
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
        Box::new(move |text| callable(text).map(|result| result.1.to_string()))
    }

    impl FromStr for TerminalCase {
        type Err = CaseParameterError;

        fn from_str(text: &str) -> Result<Self, Self::Err> {
            // Keep the arms unmerged for proper sorting of disjoined patterns.
            #[allow(clippy::match_same_arms)]
            let tested_parser = match text {
                "\u{0009}" => wrap(super::space::match_tab),
                "\u{000A}" => wrap(super::space::match_lf),
                "\u{000B}" => wrap(super::space::match_vt),
                "\u{000C}" => wrap(super::space::match_ff),
                "\u{000D}" => wrap(super::space::match_cr),
                "\u{0020}" => wrap(super::space::match_usp),
                "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" =>
                    wrap(super::numeric::match_decimal_digit),
                "/" | "/=" => wrap(super::punctuators::match_div_punctuator),
                "{" | "(" | ")" | "[" | "]" | "." | "..." | ";" | "," | "<" |
                ">" | "<=" | ">=" | "==" | "!=" | "===" | "!==" | "+" | "-" |
                "*" | "%" | "**" | "++" | "--" | "<<" | ">>" | ">>>" | "&" |
                "|" | "^" | "!" | "~" | "&&" | "||" | "??" | "?" | ":" | "=" |
                "+=" | "-=" | "*=" | "%=" | "**=" | "<<=" | ">>=" | ">>>=" |
                "&=" | "|=" | "^=" | "&&=" | "||=" | "??=" |
                "=>" => wrap(super::punctuators::match_other_punctuator),
                "}" => wrap(super::punctuators::match_right_brace_punctuator),
                "\u{00A0}" => wrap(super::space::match_usp),
                "\u{1680}" => wrap(super::space::match_usp),
                "\u{2000}" | "\u{2001}" | "\u{2002}" | "\u{2003}" |
                "\u{2004}" | "\u{2005}" | "\u{2006}" | "\u{2007}" |
                "\u{2008}" | "\u{2009}" | "\u{200A}" => wrap(super::space::match_usp),
                "\u{200C}" => wrap(super::names::match_zwnj),
                "\u{200D}" => wrap(super::names::match_zwj),
                "\u{2028}" => wrap(super::space::match_ls),
                "\u{2029}" => wrap(super::space::match_ps),
                "\u{202F}" => wrap(super::space::match_usp),
                "\u{205F}" => wrap(super::space::match_usp),
                "\u{3000}" => wrap(super::space::match_usp),
                "\u{FEFF}" => wrap(super::space::match_zwnbsp),
                _ => wrap(|_| None::<((), &str)>)
            };
            Ok(Self {
                terminal: text.to_string(),
                parser: tested_parser
            })
        }
    }
}
