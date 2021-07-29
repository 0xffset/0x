use crate::parser_objects::{Parser, ParserState, ParserStringResult};
use crate::string_utils::StringUtils;

#[derive(Clone)]
pub struct StringParser {
    pub target: String,
}

impl StringParser {
    pub fn new(target: String) -> Box<Self> {
        Box::new(StringParser { target: target })
    }
}

impl Parser for StringParser {
    fn exec(&self, state: ParserState) -> ParserState {
        if state.is_err {
            return state;
        }

        // slice to current index in input
        let input_slice = state.input.slice(state.index..);

        // simple checks before expensive string matching
        if input_slice.len() == 0 || input_slice.len() < self.target.len() {
            return state.set_err(format!(
                "[str] Tried to match '{}', but got unexpected end of input",
                self.target
            ));
        }

        // match the string
        if input_slice.starts_with(&self.target) {
            return state.update_state(
                self.target.len(),
                Some(Box::new(ParserStringResult::new(self.target.to_string()))),
            );
        }

        // no match
        return state.set_err(format!(
            "[str] Tried to match '{}', but got '{}'",
            self.target,
            input_slice.slice(..10)
        ));
    }
}
