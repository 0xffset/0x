macro_rules! obj {
	($sn:ident, $($n:ident: $t:ty),+) => {
		impl ParserResult for $sn {}

		#[derive(Debug, Clone)]
		pub struct $sn {
			$(pub $n: $t),*
		}

		impl $sn {
			pub fn new($($n: $t),*) -> Self {
				$sn {
					$($n),*
				}
			}
		}
	};
}

pub struct Parser<T> {
	pub transformer: T
}

#[derive(Debug, Clone)]
pub struct ParserState {
    pub input: String,
    pub index: usize,
    pub res: Option<Box<dyn ParserResult>>,
    pub is_err: bool,
    pub err_msg: Option<String>,
}

// scuffed way to compare two ParserStates
// convert them to their string representation and compare them
impl ParserState {
	pub fn get_bytes(&self) -> Vec<u8> {
		let bytes = format!("{:?}", self);
		bytes.as_bytes().to_vec()
	}

	pub fn set_err(mut self, err: String) -> ParserState {
		self.is_err = true;
		self.err_msg = Some(err);

		self
	}

	pub fn update_state(mut self, offset: usize, res: Option<Box<dyn ParserResult>>) -> ParserState {
		self.index += offset;
		self.res = res;

		self
	}
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

pub trait ParserResult: std::fmt::Debug + ParserResultClone {}

obj!(ParserStringResult, value: String);
obj!(ParserVecResult, value: Vec<Box<dyn ParserResult>>);
