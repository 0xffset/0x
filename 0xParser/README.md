# 0xParser

Parser Combinator crate.

#### Info

README is subject to change.

### How to use

```Rust
let res = parse(
	"Hello World",
	map(
		sequence!(string("Hello"), spaces(), string("World")),
		|r| Ok((r.0, r.1 .0, r.1 .1)),
	),
);

assert_eq!(
	res.unwrap(),
	("Hello".to_string(), " ".to_string(), "World".to_string())
);
```