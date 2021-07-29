use crate::parser_objects::*;

#[derive(Clone)]
pub struct SequenceOfParser {
    pub parsers: Vec<Box<dyn Parser>>,
}

impl SequenceOfParser {
    pub fn new(parsers: Vec<Box<dyn Parser>>) -> Box<Self> {
        Box::new(SequenceOfParser { parsers })
    }
}

impl Parser for SequenceOfParser {
    fn exec(&self, state: ParserState) -> ParserState {
        if state.is_err {
            return state;
        }

        let mut results = Vec::new();
        let mut next_state = state;

        // apply every parser in order
        for parser in &self.parsers {
            next_state = parser.exec(next_state.clone());

            if next_state.is_err {
                return next_state.update_state(0, Some(Box::new(ParserVecResult::new(results))));
            }

            // push result
            results.push(next_state.res.clone().unwrap());
        }

        return next_state.update_state(0, Some(Box::new(ParserVecResult::new(results))));
    }
}
