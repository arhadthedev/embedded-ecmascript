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
