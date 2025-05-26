use std::fmt::Display;

use derive_builder::Builder;
use pest::iterators::Pair;

use crate::CellParseError;

use super::Rule;

/// Freeform block in `.cell`
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Builder)]
#[builder()]
pub struct Block {
    order: usize,
    name: String,
    values: Vec<String>,
}

impl Block {
    pub fn new(order: usize, name: String, values: Vec<String>) -> Self {
        Self {
            order,
            name,
            values,
        }
    }

    pub fn order(&self) -> usize {
        self.order
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn values(&self) -> &[String] {
        &self.values
    }

    pub fn values_mut(&mut self) -> &mut Vec<String> {
        &mut self.values
    }

    pub fn from_pair(pair: Pair<'_, Rule>, order: usize) -> Self {
        let inner_rules = pair.into_inner();
        let block_name = inner_rules
            .find_first_tagged("block_name")
            .unwrap()
            .as_str();
        let block_lines = inner_rules
            .find_tagged("block_values") // get all value lines
            .flat_map(|lines| {
                lines
                    .into_inner()
                    .map(|pair| pair.as_str().to_string())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<String>>();
        Self::new(order, block_name.to_string(), block_lines)
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            [
                format!("%BLOCK {}", self.name()),
                self.values().join("\n"),
                format!("%ENDBLOCK {}", self.name())
            ]
            .into_iter()
            .filter(|line| !line.is_empty())
            .collect::<Vec<String>>()
            .join("\n")
        )
    }
}

/// Convert between the impl type and `Block`
/// Usage: convert from parsed block to edit data,
/// then convert back to block to insert back to the `HashMap<&str, CELLObject>`
pub trait BlockIO {
    type Item;
    fn from_block(block: &Block) -> Result<Self::Item, CellParseError>;
    fn to_block(&self, order: usize) -> Block;
}
