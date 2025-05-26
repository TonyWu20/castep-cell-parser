mod error;
mod pest_parser;

pub use error::CellParseError;
pub use pest_parser::*;

#[cfg(test)]
mod tests {

    #[test]
    fn block() {
        todo!()
    }
}
