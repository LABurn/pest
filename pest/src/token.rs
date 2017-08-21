// pest. The Elegant Parser
// Copyright (C) 2017  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::hash::{Hash, Hasher};

use super::inputs::{Input, Position};

/// An `enum` representing tokens generated by a `Parser`.
#[derive(Debug, Eq)]
pub enum Token<R, I: Input> {
    /// The starting bit of the `Token`
    Start {
        rule: R,
        pos: Position<I>
    },
    /// The ending bit of the `Token`
    End {
        rule: R,
        pos: Position<I>
    }
}

impl<R: Clone, I: Input> Clone for Token<R, I> {
    fn clone(&self) -> Token<R, I> {
        match *self {
            Token::Start { ref rule, ref pos } => {
                Token::Start {
                    rule: rule.clone(),
                    pos: pos.clone()
                }
            }
            Token::End { ref rule, ref pos } => {
                Token::End {
                    rule: rule.clone(),
                    pos: pos.clone()
                }
            }
        }
    }
}

impl<R: PartialEq, I: Input> PartialEq for Token<R, I> {
    fn eq(&self, other: &Token<R, I>) -> bool {
        match *self {
            Token::Start { ref rule, ref pos } => {
                match *other {
                    Token::Start { rule: ref other_rule, pos: ref other_pos } => {
                        rule == other_rule && pos == other_pos
                    }
                    _ => false
                }
            }
            Token::End { ref rule, ref pos } => {
                match *other {
                    Token::End { rule: ref other_rule, pos: ref other_pos } => {
                        rule == other_rule && pos == other_pos
                    }
                    _ => false
                }
            }
        }
    }
}

impl<R: Hash, I: Input> Hash for Token<R, I> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match *self {
            Token::Start { ref rule, ref pos } | Token::End { ref rule, ref pos } => {
                rule.hash(state);
                pos.hash(state);
            }
        }
    }
}
