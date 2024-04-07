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

    pub fn with_term(
        tested: fn(&str) -> Option<((), &str)>,
        input: &str,
        sep: &str,
    ) {
        // Empty strings do not match
        assert_eq!(tested(""), None);

        // Skip false match when the function recognizes a separator.
        if tested(sep) != Some(((), "")) {
            // Non-matching strings do not match
            assert_eq!(tested(sep), None);

            // Catch arbitrary (regex-like) match of a necessary symbol
            assert_eq!(tested(format!("{sep}{input}").as_ref()), None);
        }

        // Test EOF match
        assert_eq!(tested(input), Some(((), "")));

        // Test non-EOF match
        assert_eq!(
            tested(format!("{input}{sep}").as_ref()),
            Some(((), sep))
        );

        // Test repetitions
        assert_eq!(
            tested(format!("{input}{input}").as_ref()),
            Some(((), input))
        );

        // Test separated repetitions
        assert_eq!(
            tested(format!("{input}{sep}{input}").as_ref()),
            Some(((), format!("{sep}{input}").as_ref()))
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
