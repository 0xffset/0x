# 0xParser

Parser Combinator crate.

#### Info

README is subject to change.

### How to use

```Rust
let str_parser = StringParser::new("Hello".to_string());
let str2_parser = StringParser::new("World".to_string());
let sqe_parser = SqeuenceOfParser::new(vec![str_parser, str2_parser]);

println!("{:#?}", sqe_parser.run("HelloWorld"));
```