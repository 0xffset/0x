pub mod parser_objects;
pub mod parsers;
pub mod string_utils;

#[cfg(test)]
mod tests {
    use crate::{
        parser_objects::*,
        parsers::{sequence_of, str},
    };

    fn run(parser: &impl Fn(ParserState) -> ParserState, input: String) -> ParserState {
        let init_state = ParserState {
            input,
            index: 0,
            res: None,
            is_err: false,
            err_msg: None,
        };

        parser(init_state)
    }

    fn compare_results(res: &ParserState, exp: &ParserState) -> bool {
        let a = res.get_bytes();
        let b = exp.get_bytes();

        a.len() == b.len() && a.len() == a.iter().zip(&b).filter(|&(a, b)| a == b).count()
    }

    #[test]
    fn string_parse_test() {
        let parser = str("Hallo".to_string());

        let res = run(&parser, "Hallo".to_string());
        let exp = ParserState {
            input: "Hallo".to_string(),
            index: 5,
            res: Some(Box::new(ParserStringResult::new("Hallo".to_string()))),
            is_err: false,
            err_msg: None,
        };

        assert_eq!(compare_results(&res, &exp), true);

        let res = run(&parser, "Hello".to_string());
        let exp = ParserState {
            input: "Hello".to_string(),
            index: 0,
            res: None,
            is_err: true,
            err_msg: Some("[str] Tried to match 'Hallo', but got 'Hello'".to_string())
        };

        assert_eq!(compare_results(&res, &exp), true);
    }

    #[test]
    fn sequence_of_test() {
        let str1_parser = str("Hallo".to_string());
        let str2_parser = str("Welt".to_string());
        let vec = vec![str1_parser, str2_parser];
        let parser = sequence_of(vec);

        let res = run(&parser, "HalloWelt".to_string());
        let exp = ParserState {
            input: "HalloWelt".to_string(),
            index: 9,
            res: Some(Box::new(ParserVecResult::new(vec![
                Box::new(ParserStringResult::new("Hallo".to_string())),
                Box::new(ParserStringResult::new("Welt".to_string())),
            ]))),
            is_err: false,
            err_msg: None,
        };

        assert_eq!(compare_results(&res, &exp), true);

        let res = run(&parser, "HalloWorld".to_string());
        let exp = ParserState {
            input: "HalloWorld".to_string(),
            index: 5,
            res: Some(Box::new(ParserVecResult::new(vec![
                Box::new(ParserStringResult::new("Hallo".to_string()))
            ]))),
            is_err: true,
            err_msg: Some("[str] Tried to match 'Welt', but got 'World'".to_string()),
        };

        assert_eq!(compare_results(&res, &exp), true);
    }
}
