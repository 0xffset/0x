#[macro_export]
macro_rules! sequence {
	($x:expr) => { $x };

	($x:expr, $($y:expr),+) => {{
		sequence($x, sequence!($($y),+))
	}};
}