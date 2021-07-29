use crate::parser_objects::*;

pub fn sequence_of(
    parsers: Vec<impl Fn(ParserState) -> ParserState>,
) -> impl Fn(ParserState) -> ParserState {
    move |parser_state: ParserState| {
        if parser_state.is_err {
            return parser_state;
        }

        let mut results = Vec::new();
        let mut next_state = parser_state;

        // apply every parser in order
        for parser in &parsers {
            next_state = parser(next_state.clone());

            if next_state.is_err {
                return next_state.update_state(0, Some(Box::new(ParserVecResult::new(results))));
            }

			// push result
            results.push(next_state.res.clone().unwrap());
        }

        return next_state.update_state(0, Some(Box::new(ParserVecResult::new(results))));
    }
}
