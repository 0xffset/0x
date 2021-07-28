use crate::parser_objects::{ParserStringResult, ParserState};
use crate::string_utils::StringUtils;

pub fn str(target: String) -> impl Fn(ParserState) -> ParserState {
	move |mut state: ParserState| {
		if state.is_error {
			return state;
		}

		if state.input.slice(state.index..).starts_with(target.as_str()) {
			state.index += target.len();
			state.result = Some(Box::new(ParserStringResult::new(target.clone())));

			return state;
		}

		state.is_error = true;
		state.error_message = Some(format!("Tried to match '{}', but got '{}'", target, state.input.slice(state.index..10)));

		return state;
	}
}