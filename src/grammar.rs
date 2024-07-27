//! Converter from `.js` and `.mjs` ECMAScript files into a grammar tree.
//!
//! Implements <https://262.ecma-international.org/14.0/>.
//!
//! Third party conditions
//! ======================
//!
//! This file cites and implements ECMA-262 14th edition also known as
//! ECMAScript 2023 (<https://262.ecma-international.org/14.0/>).
//!
//! Terminology and citations are provided under the following conditions listed
//! in section I Copyright & Software License:
//!
//! > Copyright Notice
//! >
//! > © 2023 Ecma International
//! >
//! > By obtaining and/or copying this work, you (the licensee) agree that you
//! > have read, understood, and will comply with the following terms
//! > and conditions.
//! >
//! > Permission under Ecma’s copyright to copy, modify, prepare derivative
//! > works of, and distribute this work, with or without modification, for any
//! > purpose and without fee or royalty is hereby granted, provided that you
//! > include the following on ALL copies of the work or portions thereof,
//! > including modifications:
//! >
//! > (i) The full text of this COPYRIGHT NOTICE AND COPYRIGHT LICENSE
//! > in a location viewable to users of the redistributed or derivative work.
//! >
//! > (ii) Any pre-existing intellectual property disclaimers, notices, or
//! > terms and conditions. If none exist, the Ecma alternative copyright notice
//! > should be included.
//! >
//! > (iii) Notice of any changes or modifications, through a copyright
//! > statement on the document such as “This document includes material copied
//! > from or derived from [title and URI of the Ecma document]. Copyright
//! > © Ecma International.”
//! >
//! > Disclaimers
//! >
//! > THIS WORK IS PROVIDED “AS IS,” AND COPYRIGHT HOLDERS MAKE NO
//! > REPRESENTATIONS OR WARRANTIES, EXPRESS OR IMPLIED, INCLUDING
//! > BUT NOT LIMITED TO, WARRANTIES OF MERCHANTABILITY OR FITNESS FOR ANY
//! > PARTICULAR PURPOSE OR THAT THE USE OF THE DOCUMENT WILL NOT INFRINGE ANY
//! > THIRD PARTY PATENTS, COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS.
//! >
//! > COPYRIGHT HOLDERS WILL NOT BE LIABLE FOR ANY DIRECT, INDIRECT, SPECIAL
//! > OR CONSEQUENTIAL DAMAGES ARISING OUT OF ANY USE OF THE DOCUMENT.
//! >
//! > The name and trademarks of copyright holders may NOT be used in
//! > advertising or publicity pertaining to the work without specific, written
//! > prior permission. Title to copyright in this work will at all times remain
//! > with copyright holders.

#[derive(Debug, Eq, PartialEq)]
pub enum Symbol {
    // 11.1 Source Text
    SourceCharacter(char),

    // 14 ECMAScript Language: Statements and Declarations
    Statement {
        uses_yield: Option<usize>,
        uses_await: Option<usize>,
        uses_return: Option<usize>
    },
    // 14.2 Block
    StatementList {
        uses_yield: Option<usize>,
        uses_await: Option<usize>,
        uses_return: Option<usize>
    },
    StatementListItem {
        uses_yield: Option<usize>,
        uses_await: Option<usize>,
        uses_return: Option<usize>
    },
    // 14.4 Empty Statement
    EmptyStatement,

    // 16.1 Scripts
    Script,
    ScriptBody,
}

#[derive(Debug)]
struct TokenStackDiff {
    pop: usize,
    push: Symbol
}

fn reduce_once(tokens: &Vec<Symbol>, _as_module: bool) -> Option<TokenStackDiff> {
    match tokens.as_slice() {
        /************************************************
         *
         * 14.2 Block
         *
         ************************************************/

        // Serialize <https://262.ecma-international.org/14.0/#prod-StatementList>.
        //
        // ```plain
        // StatementList[Yield, Await, Return] :
        //     StatementListItem[?Yield, ?Await, ?Return]
        //     StatementList[?Yield, ?Await, ?Return] StatementListItem[?Yield, ?Await, ?Return]
        // ```
        //
        // Right side of the productions ends in the same way so put
        // the longest match first.
        [
            ..,
            Symbol::StatementList {
                uses_yield: list_uses_yield,
                uses_await: list_uses_await,
                uses_return: list_uses_return
            },
            Symbol::StatementListItem {
                uses_yield: item_uses_yield,
                uses_await: item_uses_await,
                uses_return: item_uses_return
            }
        ] => Some(TokenStackDiff {
            pop: 2,
            push: Symbol::StatementList {
                uses_yield: list_uses_yield.or(*item_uses_yield),
                uses_await: list_uses_await.or(*item_uses_await),
                uses_return: list_uses_return.or(*item_uses_return)
            }
        }),
        [
            ..,
            Symbol::StatementListItem { uses_yield, uses_await, uses_return }
        ] => Some(TokenStackDiff {
            pop: 1,
            push: Symbol::StatementList {
                uses_yield: *uses_yield,
                uses_await: *uses_await,
                uses_return: *uses_return
            }
        }),

        // Serialize <https://262.ecma-international.org/14.0/#prod-StatementListItem>.
        //
        // ```plain
        // StatementListItem[Yield, Await, Return] :
        //     Statement[?Yield, ?Await, ?Return]
        //     Declaration[?Yield, ?Await]
        // ```
        [.., Symbol::Statement { uses_yield, uses_await, uses_return } ] => Some(TokenStackDiff {
            pop: 1,
            push: Symbol::StatementListItem {
                uses_yield: *uses_yield,
                uses_await: *uses_await,
                uses_return: *uses_return
            }
        }),

        /************************************************
         *
         * 14 ECMAScript Language: Statements and Declarations
         *
         ************************************************/

        // A match for <https://262.ecma-international.org/14.0/#prod-Statement>.
        //
        // ```plain
        // Statement[Yield, Await, Return] :
        //     BlockStatement[?Yield, ?Await, ?Return]
        //     VariableStatement[?Yield, ?Await]
        //     EmptyStatement
        //     IfStatement[?Yield, ?Await, ?Return]
        //     ExpressionStatement[?Yield, ?Await]
        //     BreakableStatement[?Yield, ?Await, ?Return]
        //     ContinueStatement[?Yield, ?Await]
        //     BreakStatement[?Yield, ?Await]
        //     [+Return] ReturnStatement[?Yield, ?Await]
        //     WithStatement[?Yield, ?Await, ?Return]
        //     LabelledStatement[?Yield, ?Await, ?Return]
        //     ThrowStatement[?Yield, ?Await]
        //     TryStatement[?Yield, ?Await, ?Return]
        //     DebuggerStatement
        // ```
        [.., Symbol::EmptyStatement] => Some(TokenStackDiff {
            pop: 1,
            push: Symbol::Statement {
                uses_yield: None,
                uses_await: None,
                uses_return: None
            }
        }),

        /************************************************
         *
         * 14.4 Empty Statement
         *
         ************************************************/

        // Serialize <https://262.ecma-international.org/14.0/#prod-EmptyStatement>.
        //
        // ```plain
        // EmptyStatement :
        //     `;`
        // ```
        [.., Symbol::SourceCharacter(';')] => Some(TokenStackDiff {
            pop: 1,
            push: Symbol::EmptyStatement
        }),
        
        /************************************************
         *
         * 16.1 Scripts
         *
         ************************************************/

        // Serialize <https://262.ecma-international.org/14.0/#prod-Script>.
        //
        // ```plain
        // Script :
        //     ScriptBody_opt
        // ```
        //
        // The empty case is processed in a caller after all input characters
        // are pushed into the stack and.
        [.., Symbol::ScriptBody] => Some(TokenStackDiff {
            pop: 1,
            push: Symbol::Script
        }),

        // Serialize <https://262.ecma-international.org/14.0/#prod-ScriptBody>.
        //
        // ```plain
        // ScriptBody :
        //     StatementList[~Yield, ~Await, ~Return]
        // ```
        [.., Symbol::StatementList { uses_yield, uses_await, uses_return }] => {
            match (uses_yield, uses_await, uses_return) {
                (Some(_), Some(_), Some(_)) => None,
                _ => Some(TokenStackDiff { pop: 1, push: Symbol::ScriptBody })
            }
        },

        _ => None
    }
}

fn reduce(mut eager_parse_stack: Vec<Symbol>, codepoint: char, as_module: bool) -> Vec<Symbol> {
    match reduce_once(&eager_parse_stack, as_module) {
        Some(stack_diff) => {
            eager_parse_stack.truncate(eager_parse_stack.len() - stack_diff.pop);
            eager_parse_stack.push(stack_diff.push);
            reduce(eager_parse_stack, codepoint, as_module)
        },
        None => eager_parse_stack
    }
}

/// Parses a `.js`/`.mjs` text and performs early error checks.
///
/// Parsing is done as described in <https://262.ecma-international.org/14.0/>,
/// sections 11-16 (named *ECMAScript Language: [aspect name]*).
///
/// # Errors
///
/// Will return `Err` with rustc-style formatted error message string, if the
/// source parameter does not form a correct ECMAScript 2023 script or module.
pub fn parse(source: &str, as_module: bool) -> Result<(), Vec<Symbol>> {
    let final_parse_stack = source.chars().fold(
        Vec::with_capacity(512),
        |mut accumulator, codepoint| {
            accumulator.push(Symbol::SourceCharacter(codepoint));
            reduce(accumulator, codepoint, as_module)
        }
    );
    match final_parse_stack.len() {
        0 | 1 => Ok(()),
        _ => Err(final_parse_stack)
    }
}
