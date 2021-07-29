use crate::parser_objects::{ParserState, ParserStringResult};
use crate::string_utils::StringUtils;

#[macro_export]
macro_rules! str {
    ($target:expr) => {
        |state| internal_str($target, state)
    };
}

pub fn internal_str(target: &str, state: ParserState) -> ParserState {
    if state.is_err {
        return state;
    }

    // slice to current index in input
    let input_slice = state.input.slice(state.index..);

    // simple checks before expensive string matching
    if input_slice.len() == 0 || input_slice.len() < target.len() {
        return state.set_err(format!(
            "[str] Tried to match '{}', but got unexpected end of input",
            target
        ));
    }

    // match the string
    if input_slice.starts_with(target) {
        return state.update_state(
            target.len(),
            Some(Box::new(ParserStringResult::new(target.to_string()))),
        );
    }

    // no match
    return state.set_err(format!(
        "[str] Tried to match '{}', but got '{}'",
        target,
        input_slice.slice(..10)
    ));
}
