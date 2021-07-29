use crate::parser_objects::*;

#[macro_export]
macro_rules! sequence_of {
    ($($parsers:expr),+) => {
        |state| internal_sequence_of(
            vec![
                $($parsers),*
            ],
            state
        )
    };
}

pub fn internal_sequence_of(
    parsers: Vec<fn(ParserState) -> ParserState>,
    state: ParserState,
) -> ParserState {
    if state.is_err {
        return state;
    }

    let mut results = Vec::new();
    let mut next_state = state;

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
