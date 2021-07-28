use crate::parser_objects::*;

pub fn sequence_of(parsers: Vec<impl Fn(ParserState) -> ParserState>) -> impl Fn(ParserState) -> ParserState {
	move |parser_state: ParserState| {
		if parser_state.is_error {
			return parser_state;
		}

		let mut results = Vec::new();
		let mut next_state = parser_state;

		for parser in &parsers {
			next_state = parser(next_state.clone());
			
			if next_state.is_error {
				return next_state;
			}

			results.push(next_state.result.clone().unwrap());
		}

		next_state.result = Some(Box::new(ParserVecResult::new(results)));

		return next_state;
	}
}