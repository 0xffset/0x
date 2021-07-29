# 0xParser

Parser Combinator crate.

#### Info

README is subject to change.

### How to use

```Rust
let parser = Parser::new(sequence_of!(str!("Hello"), str!("World")));

parser.run("HelloWorld".to_string());
```