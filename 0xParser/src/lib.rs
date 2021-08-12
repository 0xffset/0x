mod macros;

use std:: str::FromStr;

use crate::string_utils::StringUtils;
use regex::Regex;

pub mod string_utils;

#[allow(dead_code)]

pub trait PT: std::fmt::Debug + Clone + 'static {}
impl<T: std::fmt::Debug + Clone + 'static> PT for T {}

pub type Parser<T> = Box<dyn Fn(Context) -> Result<Success<T>, Failure>>;

#[derive(Debug, Clone)]
pub struct Context {
    pub txt: String,
    pub pos: usize,
}

#[derive(Debug, Clone)]
pub struct Success<T: std::fmt::Debug + Clone> {
    pub val: T,
    pub ctx: Context,
}

#[derive(Debug, Clone)]
pub struct Failure {
    pub exp: String,
    pub ctx: Context,
}

pub fn success<T: std::fmt::Debug + Clone>(ctx: Context, val: T) -> Success<T> {
    Success { val, ctx }
}

pub fn failure(ctx: Context, exp: String) -> Failure {
    Failure { exp, ctx }
}

pub fn string<S: AsRef<str>>(target: S) -> Parser<String> {
    let target = target.as_ref().to_string();

    Box::new(move |mut ctx: Context| {
        if ctx.txt.slice(ctx.pos..).starts_with(&target.clone()) {
            ctx.pos += target.len();
            return Ok(success(ctx, target.clone()));
        }

        return Err(failure(ctx, target.clone()));
    })
}

pub fn regex<A: AsRef<str>, B: AsRef<str>>(target: A, expected: B) -> Parser<String> {
    let target = target.as_ref().to_string();
    let expected = expected.as_ref().to_string();

    Box::new(move |mut ctx: Context| {
        let regex = match Regex::new(&target.clone()) {
            Ok(regex) => regex,
            Err(_) => panic!("Invalid regex: {}", target),
        };

        let sliced_ctx = ctx.txt.slice(ctx.pos..);
        let mat = regex.find(&sliced_ctx);
        if mat.is_some() {
            let mat = mat.unwrap();
            if mat.start() == 0 {
                ctx.pos += mat.end();
                return Ok(success(ctx, mat.as_str().to_string()));
            }
        }

        return Err(failure(ctx, expected.clone()));
    })
}

pub fn optional<T: PT>(parser: Parser<T>) -> Parser<Option<T>> {
    Box::new(move |ctx: Context| {
        let res = parser(ctx.clone());

        if res.is_err() {
            return Ok(success(res.unwrap_err().ctx, None));
        }

        let res = res.unwrap();
        return Ok(success(res.ctx, Some(res.val)));
    })
}

pub fn sequence<T: PT, U: PT>(a: Parser<T>, b: Parser<U>) -> Parser<(T, U)> {
    Box::new(move |mut ctx: Context| {
        let res_a = a(ctx.clone());
        if res_a.is_err() {
            return Err(res_a.unwrap_err());
        }
        let res_a = res_a.unwrap();
        ctx = res_a.ctx;

        let res_b = b(ctx.clone());
        if res_b.is_err() {
            return Err(res_b.unwrap_err());
        }
        let res_b = res_b.unwrap();
        ctx = res_b.ctx;

        return Ok(success(ctx, (res_a.val, res_b.val)));
    })
}

pub fn any<T: PT>(parsers: Vec<Parser<T>>) -> Parser<T> {
    Box::new(move |ctx: Context| {
        for parser in parsers.iter() {
            let res = parser(ctx.clone());
            if res.is_ok() {
                return res;
            }
        }

        return Err(failure(ctx, String::from("any()")));
    })
}

pub fn map<T: PT, U: PT>(parser: Parser<T>, mapper: fn(T) -> Result<U, String>) -> Parser<U> {
    Box::new(move |ctx: Context| {
        let res = parser(ctx.clone());
        if res.is_err() {
            return Err(res.unwrap_err());
        }
        let res = res.unwrap();

        let ctx = res.ctx;
        let new_res = mapper(res.val);
        if new_res.is_ok() {
            return Ok(success(ctx, new_res.unwrap()));
        }

        return Err(failure(ctx, new_res.unwrap_err()));
    })
}

pub fn many<T: PT>(parser: Parser<T>) -> Parser<Vec<T>> {
    Box::new(move |mut ctx: Context| {
        let mut ret: Vec<T> = Vec::new();

        loop {
            let res = parser(ctx.clone());

            if res.is_err() {
                if ret.len() == 0 {
                    let res = res.unwrap_err();
                    return Err(failure(res.ctx, res.exp));
                }

                return Ok(success(ctx, ret));
            }
            let res = res.unwrap();

            ctx = res.ctx;
            ret.push(res.val);
        }
    })
}

pub fn between<T: PT, U: PT, B: PT>(
    front: Parser<T>,
    middle: Parser<U>,
    back: Parser<B>,
) -> Parser<U> {
    return map(sequence!(front, middle, back), |res| Ok(res.1.0));
}

pub fn spaces() -> Parser<String> {
    return map(many(string(" ")), |s: Vec<String>| Ok(s.join("")));
}

fn letters() -> Parser<String> {
    return regex("[a-zA-Z]+", "letters");
}

pub fn integer() -> Parser<String> {
    return regex(r"\d+", "integer");
}

pub fn parsed_integer<T: PT + FromStr>() -> Parser<T> {
    return map(regex(r"\d+", "integer"), |s: String| match s.parse::<T>() {
        Ok(val) => Ok(val),
        Err(_) => Err("parsable integer".to_string()),
    });
}

pub fn float() -> Parser<String> {
    return regex(r"\d+\.\d*", "float");
}

pub fn parsed_float<T: PT + FromStr>() -> Parser<T> {
    return map(regex(r"\d+\.\d*", "float"), |s: String| {
        match s.parse::<T>() {
            Ok(val) => Ok(val),
            Err(_) => Err("parsable float".to_string()),
        }
    });
}

pub fn expect<T: PT, S: AsRef<str>>(parser: Parser<T>, expected: S) -> Parser<T> {
    let expected = expected.as_ref().to_string();

    Box::new(move |ctx: Context| {
        let res = parser(ctx.clone());
        if res.is_err() {
            return Err(failure(res.unwrap_err().ctx, expected.clone()));
        }

        return res;
    })
}

pub fn parse<S: AsRef<str>, T: PT>(txt: S, parser: Parser<T>) -> Result<T, String> {
    let txt = txt.as_ref().to_string();

    let res = parser(Context { txt, pos: 0 });
    if res.is_err() {
        let res = res.unwrap_err();
        return Err(format!(
            "Parser error, expected '{}' at position '{}'",
            res.exp, res.ctx.pos
        ));
    }

    return Ok(res.unwrap().val);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_test() {
        let res = parse("Hello World", string("Hello World"));
        assert_eq!(res.unwrap(), "Hello World");

        let res = parse("Hello World", string("Hallo World"));
        assert_eq!(
            res.unwrap_err(),
            "Parser error, expected 'Hallo World' at position '0'"
        );

        let res = parse("My Hello World", string("Hello World"));
        assert_eq!(
            res.unwrap_err(),
            "Parser error, expected 'Hello World' at position '0'"
        );
    }

    #[test]
    fn regex_test() {
        let res = parse("DE0012 2322 2323", regex(r"DE\d{4}\s\d{4}\s\d{4}", "IBAN"));
        assert_eq!(res.unwrap(), "DE0012 2322 2323");

        let res = parse("DE012 2322 2323", regex(r"DE\d{4}\s\d{4}\s\d{4}", "IBAN"));
        assert_eq!(
            res.unwrap_err(),
            "Parser error, expected 'IBAN' at position '0'"
        );

        let res = parse(
            "Bank account: DE012 2322 2323",
            regex(r"DE\d{4}\s\d{4}\s\d{4}", "IBAN"),
        );
        assert_eq!(
            res.unwrap_err(),
            "Parser error, expected 'IBAN' at position '0'"
        );
    }

    #[test]
    fn optional_test() {
        let res = parse("Hello World", optional(string("Hello World")));
        assert_eq!(res.unwrap(), Some("Hello World".to_string()));

        let res = parse("Hello World", optional(string("Hallo World")));
        assert_eq!(res.unwrap(), None);
    }

    #[test]
    fn sequence_test() {
        let res = parse("Hello World", sequence(string("Hello"), string(" World")));
        assert_eq!(res.unwrap(), ("Hello".to_string(), " World".to_string()));

        let res = parse("Hello World", sequence(string("Hallo"), string(" World")));
        assert_eq!(
            res.unwrap_err(),
            "Parser error, expected 'Hallo' at position '0'"
        );

        let res = parse("Hello World", sequence(string("Hello"), string("World")));
        assert_eq!(
            res.unwrap_err(),
            "Parser error, expected 'World' at position '5'"
        );

        let res = parse(
            "Hello World",
            sequence(sequence(string("Hello"), string(" ")), string("World")),
        );
        assert_eq!(
            res.unwrap(),
            (("Hello".to_string(), " ".to_string()), "World".to_string())
        );
    }

    #[test]
    fn any_test() {
        let res = parse(
            "Hello World",
            sequence(
                any(vec![string("Hallo"), string("Hello")]),
                string(" World"),
            ),
        );

        assert_eq!(res.unwrap(), ("Hello".to_string(), " World".to_string()));

        let res = parse(
            "Hello World",
            sequence(any(vec![string("Hallo"), string("Hola")]), string(" World")),
        );

        assert_eq!(
            res.unwrap_err(),
            "Parser error, expected 'any()' at position '0'"
        );
    }

    #[test]
    fn map_test() {
        let res = parse(
            "Hello World",
            map(
                sequence(sequence(string("Hello"), string(" ")), string("World")),
                |res| Ok((res.0 .0, res.0 .1, res.1)),
            ),
        );
        assert_eq!(
            res.unwrap(),
            ("Hello".to_string(), " ".to_string(), "World".to_string())
        );

        let res = parse::<&str, Option<String>>(
            "Hello World",
            map(
                sequence(sequence(string("Hello"), string(" ")), string("World")),
                |_| Err("mapping()".to_string()),
            ),
        );
        assert_eq!(
            res.unwrap_err(),
            "Parser error, expected 'mapping()' at position '11'"
        );
    }

    #[test]
    fn many_test() {
        let res = parse("Hello World", many(regex(r".{1}", "anything")));
        assert_eq!(res.unwrap().join(""), "Hello World");

        let res = parse("Hello World", many(regex(r"\d{1}", "number")));
        assert_eq!(
            res.unwrap_err(),
            "Parser error, expected 'number' at position '0'"
        );
    }

    #[test]
    fn between_test() {
        let res = parse(
            "\"Hello\"",
            between(string("\""), string("Hello"), string("\"")),
        );
        assert_eq!(res.unwrap(), "Hello");

        let res = parse(
            "1Hello\"",
            between(integer(), string("Hello"), string("\"")),
        );
        assert_eq!(res.unwrap(), "Hello");

        let res = parse(
            "\"Hello1",
            between(string("\""), string("Hello"), string("\"")),
        );
        assert_eq!(res.unwrap_err(), "Parser error, expected '\"' at position '6'");
    }

    #[test]
    fn spaces_test() {
        let res = parse(
            "Hello World",
            sequence(sequence(string("Hello"), spaces()), string("World")),
        );
        assert_eq!(
            res.unwrap(),
            (("Hello".to_string(), " ".to_string()), "World".to_string())
        );

        let res = parse(
            "HelloWorld",
            sequence(sequence(string("Hello"), spaces()), string("World")),
        );
        assert_eq!(
            res.unwrap_err(),
            "Parser error, expected ' ' at position '5'"
        );

        let res = parse(
            "Hello    World",
            sequence(sequence(string("Hello"), spaces()), string("World")),
        );
        assert_eq!(
            res.unwrap(),
            (
                ("Hello".to_string(), "    ".to_string()),
                "World".to_string()
            )
        );
    }

    #[test]
    fn letters_test() {
        let res = parse("Hello", letters());
        assert_eq!(res.unwrap(), "Hello");

        let res = parse("Hello!", letters());
        assert_eq!(res.unwrap(), "Hello");

        let res = parse("1Hello", letters());
        assert_eq!(
            res.unwrap_err(),
            "Parser error, expected 'letters' at position '0'"
        );
    }

    #[test]
    fn integer_test() {
        let res = parse("123456789", integer());
        assert_eq!(res.unwrap(), "123456789");

        let res = parse("a123456789", integer());
        assert_eq!(
            res.unwrap_err(),
            "Parser error, expected 'integer' at position '0'"
        );
    }

    #[test]
    fn parsed_integer_test() {
        let res = parse("123456789", parsed_integer::<i32>());
        assert_eq!(res.unwrap(), 123456789i32);

        let res = parse("123456789", parsed_integer::<u64>());
        assert_eq!(res.unwrap(), 123456789u64);

        let res = parse("123456789", parsed_integer::<u8>());
        // bad error for impossible to parse value
        assert_eq!(
            res.unwrap_err(),
            "Parser error, expected 'parsable integer' at position '9'"
        );

        let res = parse("a123456789", parsed_integer::<u32>());
        assert_eq!(
            res.unwrap_err(),
            "Parser error, expected 'integer' at position '0'"
        );
    }

    #[test]
    fn float_test() {
        let res = parse("12345.6789", float());
        assert_eq!(res.unwrap(), "12345.6789");

        let res = parse("a1234.56789", float());
        assert_eq!(
            res.unwrap_err(),
            "Parser error, expected 'float' at position '0'"
        );
    }

    #[test]
    fn parsed_float_test() {
        let res = parse("12345.6789", parsed_float::<f32>());
        assert_eq!(res.unwrap(), 12345.6789f32);

        let res = parse("12345678.9", parsed_float::<f64>());
        assert_eq!(res.unwrap(), 12345678.9f64);

        let res = parse("a12345.6789", parsed_float::<f32>());
        assert_eq!(
            res.unwrap_err(),
            "Parser error, expected 'float' at position '0'"
        );
    }

    #[test]
    fn expect_test() {
        let res = parse("Hello World", expect(string("Hello"), "\"Hello\""));
        assert_eq!(res.unwrap(), "Hello");

        let res = parse("Hello World", expect(string("Hallo"), "\"Hallo\""));
        assert_eq!(
            res.unwrap_err(),
            "Parser error, expected '\"Hallo\"' at position '0'"
        );
    }

    #[test]
    fn sequence_macro_test() {
        let res = parse(
            "Hello World",
            map(sequence!(string("Hello"), spaces(), string("World")), |r| {
                Ok((r.0, r.1 .0, r.1 .1))
            }),
        );

        assert_eq!(
            res.unwrap(),
            ("Hello".to_string(), " ".to_string(), "World".to_string())
        );
    }
}
