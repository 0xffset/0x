use crate::parser_objects::{ParserState, ParserStringResult};
use crate::string_utils::StringUtils;

pub fn str(target: String) -> impl Fn(ParserState) -> ParserState {
    move |state: ParserState| {
        if state.is_err {
            return state;
        }

        // slice to current index in input
        let input_slice = state.input.slice(state.index..);

		// simple checks before expensive string matching
        if input_slice.len() == 0 || input_slice.len() < target.len() {
            return state.set_err(format!("[str] Tried to match '{}', but got unexpected end of input", target));
        }

		// match the string
        if input_slice.starts_with(target.as_str()) {
            return state.update_state(
                target.len(),
                Some(Box::new(ParserStringResult::new(target.clone()))),
            );
        }

		// no match
        return state.set_err(format!(
            "[str] Tried to match '{}', but got '{}'",
            target,
            input_slice.slice(..10)
        ));
    }
}
