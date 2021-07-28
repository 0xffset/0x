macro_rules! obj {
	($sn:ident, $($n:ident: $t:ty),+) => {
		impl ParserResult for $sn {}

		#[derive(Debug, Clone)]
		pub struct $sn {
			$(pub $n: $t),*
		}

		impl $sn {
			pub fn new($($n: $t),*) -> $sn {
				$sn {
					$($n),*
				}
			}
		}
	};
}

#[derive(Debug, Clone)]
pub struct ParserState {
	pub input: String,
	pub index: usize,
	pub result: Option<Box<dyn ParserResult>>,
	pub is_error: bool,
	pub error_message: Option<String>,
}

// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
// https://stackoverflow.com/a/30353928/12807712
pub trait ParserResultClone {
	fn clone_box(&self) -> Box<dyn ParserResult>;
}

impl<T> ParserResultClone for T
where
    T: 'static + ParserResult + Clone,
{
    fn clone_box(&self) -> Box<dyn ParserResult> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn ParserResult> {
    fn clone(&self) -> Box<dyn ParserResult> {
        self.clone_box()
    }
}

// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

pub trait ParserResult : std::fmt::Debug + ParserResultClone {}

obj!(ParserStringResult, value: String);
obj!(ParserVecResult, value: Vec<Box<dyn ParserResult>>);
