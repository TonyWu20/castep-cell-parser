#![allow(unused_imports)]
use std::{collections::HashMap, fmt::Display};

use pest::iterators::Pairs;
use pest_derive::Parser;

mod block;
mod cell_object;
mod keyword_value;

pub use block::{Block, BlockBuilder, BlockBuilderError, BlockIO};
pub use cell_object::CELLObject;
pub use keyword_value::{KeywordValue, SingleKeyword};

#[derive(Debug, Parser)]
#[grammar = "src/cell.pest"]
pub struct CELLParser;

impl CELLParser {
    /// Build `HashMap<String, CELLObject>` from `Pairs<'_, Rule>`
    /// Caution: I decided to convert all keys to **lowercase**, because while `CASTEP` parses the files caselessly,
    /// it is hard to handle the case-sensitiveness in pure `HashMap`.
    pub fn cell_doc_map(pairs: Pairs<'_, Rule>) -> ParsedCellDoc {
        ParsedCellDoc(HashMap::from_iter(pairs.into_iter().enumerate().map(
            |(idx, pair)| match pair.as_rule() {
                Rule::block => {
                    let inner_rules = pair.into_inner();
                    let block_name = inner_rules
                        .find_first_tagged("block_name")
                        .unwrap()
                        .as_str()
                        // Forced to lowercase!
                        .to_lowercase();
                    let block_lines = inner_rules
                        .find_tagged("block_values") // get all value lines
                        .flat_map(|lines| {
                            lines
                                .into_inner()
                                .map(|pair| pair.as_str().to_string())
                                .collect::<Vec<String>>()
                        })
                        .collect::<Vec<String>>();
                    let block = Block::new(idx, block_name.to_string(), block_lines);
                    (block_name, CELLObject::Block(block))
                }
                Rule::kv_pair => {
                    let mut inner_rules = pair.into_inner();
                    // Forced to lowercase!
                    let name = inner_rules.next().unwrap().as_str();
                    let value = inner_rules.next().unwrap().as_str().to_string();
                    (
                        name.to_lowercase(),
                        CELLObject::KeywordValue(KeywordValue::new(idx, name.to_string(), value)),
                    )
                }
                Rule::single_keywords => {
                    // Forced to lowercase!
                    let name = pair.as_str();
                    (
                        name.to_lowercase(),
                        CELLObject::SingleKeyword(SingleKeyword::new(idx, name.to_string())),
                    )
                }
                _ => unreachable!(),
            },
        )))
    }

    /// Used for `.cell` format
    /// Maintained the order from parsed document
    pub fn ordered_cell_doc(cell_doc_map: &HashMap<String, CELLObject>) -> OrderedCellDoc {
        let mut ordered_cell_doc: Vec<CELLObject> = cell_doc_map.values().cloned().collect();
        ordered_cell_doc.sort_by_key(|obj| obj.order());
        OrderedCellDoc(ordered_cell_doc)
    }

    /// Used for `.param` format
    /// Maintained the order from parsed document
    pub fn ordered_param_doc(cell_doc_map: &HashMap<String, CELLObject>) -> OrderedParamDoc {
        let mut ordered_param_doc: Vec<CELLObject> = cell_doc_map.values().cloned().collect();
        ordered_param_doc.sort_by_key(|obj| obj.order());
        OrderedParamDoc(ordered_param_doc)
    }
}

#[derive(Debug, Clone)]
pub struct ParsedCellDoc(HashMap<String, CELLObject>);

impl std::ops::DerefMut for ParsedCellDoc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::ops::Deref for ParsedCellDoc {
    type Target = HashMap<String, CELLObject>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct OrderedCellDoc(Vec<CELLObject>);
#[derive(Debug, Clone)]
pub struct OrderedParamDoc(Vec<CELLObject>);

impl Display for OrderedCellDoc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|obj| obj.to_string())
                .collect::<Vec<String>>()
                .join("\n\n")
        )
    }
}

impl Display for OrderedParamDoc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|obj| obj.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
