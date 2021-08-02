# 0xParser

Parser Combinator crate.

#### Info

README is subject to change.

### How to use

```Rust
let res = parse(
	"Hello World".to_string(),
	map(
		sequence(
			sequence(string("Hello".to_string()), spaces()),
			string("World".to_string()),
		),
		|r| Ok((r.0 .0, r.0 .1, r.1)),
	),
);

assert_eq!(
	res.unwrap(),
	("Hello".to_string(), " ".to_string(), "World".to_string())
);
```