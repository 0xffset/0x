pub mod parser_objects;
pub mod parsers;
pub mod string_utils;

#[cfg(test)]
mod tests {
    use crate::{parser_objects::*, parsers::*};

    fn compare_results(res: &ParserState, exp: &ParserState) -> bool {
        let a = res.get_bytes();
        let b = exp.get_bytes();

        a.len() == b.len() && a.len() == a.iter().zip(&b).filter(|&(a, b)| a == b).count()
    }

    #[test]
    fn string_parse_test() {
        let str_parser = StringParser::new("Hallo".to_string());

        let exp = ParserState {
            input: "Hallo".to_string(),
            index: 5,
            res: Some(Box::new(ParserStringResult::new("Hallo".to_string()))),
            is_err: false,
            err_msg: None,
        };

        assert_eq!(
            compare_results(&str_parser.run("Hallo".to_string()), &exp),
            true
        );

        let exp = ParserState {
            input: "Hello".to_string(),
            index: 0,
            res: None,
            is_err: true,
            err_msg: Some("[str] Tried to match 'Hallo', but got 'Hello'".to_string()),
        };

        assert_eq!(
            compare_results(&str_parser.run("Hello".to_string()), &exp),
            true
        );
    }

    #[test]
    fn sequence_of_test() {
        let str1 = StringParser::new("Hallo".to_string());
        let str2 = StringParser::new("Welt".to_string());
        let str3 = StringParser::new("!".to_string());
        let sqe_parser = SequenceOfParser::new(vec![str1, str2, str3]);

        let exp = ParserState {
            input: "HalloWelt!".to_string(),
            index: 10,
            res: Some(Box::new(ParserVecResult::new(vec![
                Box::new(ParserStringResult::new("Hallo".to_string())),
                Box::new(ParserStringResult::new("Welt".to_string())),
                Box::new(ParserStringResult::new("!".to_string())),
            ]))),
            is_err: false,
            err_msg: None,
        };

        assert_eq!(
            compare_results(&sqe_parser.run("HalloWelt!".to_string()), &exp),
            true
        );

        let exp = ParserState {
            input: "HalloWorld!".to_string(),
            index: 5,
            res: Some(Box::new(ParserVecResult::new(vec![Box::new(
                ParserStringResult::new("Hallo".to_string()),
            )]))),
            is_err: true,
            err_msg: Some("[str] Tried to match 'Welt', but got 'World!'".to_string()),
        };

        assert_eq!(
            compare_results(&sqe_parser.run("HalloWorld!".to_string()), &exp),
            true
        );
    }
}
