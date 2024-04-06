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

    pub const fn return_none(_: &str) -> Option<((), &str)> {
        Option::None
    }
}

