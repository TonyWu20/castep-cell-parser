use std::fmt::Display;

use super::{Block, KeywordValue, SingleKeyword};

#[derive(Debug, Clone, PartialEq)]
pub enum CELLObject {
    Block(Block),
    KeywordValue(KeywordValue),
    SingleKeyword(SingleKeyword),
}

impl PartialOrd for CELLObject {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.order().partial_cmp(&other.order())
    }
}

impl CELLObject {
    pub fn order(&self) -> usize {
        match self {
            CELLObject::Block(block) => block.order(),
            CELLObject::KeywordValue(keyword_value) => keyword_value.order(),
            CELLObject::SingleKeyword(single_keyword) => single_keyword.order(),
        }
    }

    pub fn as_block(&self) -> Option<&Block> {
        if let Self::Block(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_keyword_value(&self) -> Option<&KeywordValue> {
        if let Self::KeywordValue(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_single_keyword(&self) -> Option<&SingleKeyword> {
        if let Self::SingleKeyword(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl Display for CELLObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CELLObject::Block(block) => block.to_string(),
                CELLObject::KeywordValue(keyword_value) => keyword_value.to_string(),
                CELLObject::SingleKeyword(single_keyword) => single_keyword.to_string(),
            }
        )
    }
}
