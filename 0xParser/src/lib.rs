pub mod string_utils;
pub mod parser_objects;
pub mod parsers;

#[cfg(test)]
mod tests {
    use crate::{
        parser_objects::{ParserState, ParserStringResult},
        parsers::{sequence_of, str},
    };

    fn temp(parser: impl Fn(ParserState) -> ParserState, input: String) -> ParserState {
        let init_state = ParserState {
            input,
            index: 0,
            result: None,
            is_error: false,
            error_message: None,
        };

        parser(init_state)
    }

    #[test]
    fn string_parse_test() {
        let parser = str("Hallo".to_string());

        println!("Got:\n{:#?}", temp(parser, "Hallo".to_string()));

        println!(
            "Expected:\n{:#?}",
            ParserState {
                input: "Hallo".to_string(),
                index: 5,
                result: Some(Box::new(ParserStringResult::new("Hallo".to_string()))),
                is_error: false,
                error_message: None,
            }
        );
    }

    #[test]
    fn sequence_of_test() {
        let str1_parser = str("Hallo".to_string());
        let str2_parser = str("Welt".to_string());
        let vec = vec![str1_parser, str2_parser];
        let parser = sequence_of(vec);

        println!("Got:\n{:#?}", temp(parser, "HalloWelt".to_string()));
    }
}
