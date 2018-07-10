use error::TSError;
use parser::utils::{Parsed, Parser as ParserT};
use serde_json::{self, Error, Value};

/// The Raw Parser is a simple parser that performs no action on the
/// data and just hands on `raw`
pub struct Parser {}
impl Parser {
    pub fn new(_opts: &str) -> Self {
        Parser {}
    }
}
impl ParserT for Parser {
    fn parse(&self, msg: String) -> Result<Parsed, TSError> {
        match serde_json::from_str::<Value>(msg.as_str()) {
            Ok(_) => Ok(Parsed::new(msg)),
            Err(e) => {
                warn!("Bad JSON: {}", e);
                Err(TSError::new(format!("Serade error: {}", e).as_str()))
            }
        }
    }
}