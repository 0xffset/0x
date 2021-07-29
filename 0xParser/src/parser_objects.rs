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

// https://stackoverflow.com/a/30353928/12807712
macro_rules! clone_boxed {
    ($trait:ident, $clone:ident) => {
        pub trait $clone {
            fn clone_box(&self) -> Box<dyn $trait>;
        }
        impl<T> $clone for T
        where
            T: 'static + $trait + Clone,
        {
            fn clone_box(&self) -> Box<dyn $trait> {
                Box::new(self.clone())
            }
        }

        impl Clone for Box<dyn $trait> {
            fn clone(&self) -> Box<dyn $trait> {
                self.clone_box()
            }
        }
    };
}

// ##########
// # Parser #
// ##########
clone_boxed!(Parser, ParserClone);
pub trait Parser: ParserClone {
    fn exec(&self, state: ParserState) -> ParserState;
    fn run(&self, target: String) -> ParserState {
        let state = ParserState {
            input: target,
            index: 0,
            res: None,
            is_err: false,
            err_msg: None,
        };

        self.exec(state)
    }
}

// ################
// # Parser State #
// ################
#[derive(Debug, Clone)]
pub struct ParserState {
    pub input: String,
    pub index: usize,
    pub res: Option<Box<dyn ParserResult>>,
    pub is_err: bool,
    pub err_msg: Option<String>,
}

impl ParserState {
	// scuffed way to compare two ParserStates
	// convert them to their string representation and compare them
    pub fn get_bytes(&self) -> Vec<u8> {
        let bytes = format!("{:?}", self);
        bytes.as_bytes().to_vec()
    }

    pub fn set_err(mut self, err: String) -> ParserState {
        self.is_err = true;
        self.err_msg = Some(err);

        self
    }

    pub fn update_state(
        mut self,
        offset: usize,
        res: Option<Box<dyn ParserResult>>,
    ) -> ParserState {
        self.index += offset;
        self.res = res;

        self
    }
}

// ##################
// # Parser Results #
// ##################
clone_boxed!(ParserResult, ParserResultClone);
pub trait ParserResult: std::fmt::Debug + ParserResultClone {}

obj!(ParserStringResult, value: String);
obj!(ParserVecResult, value: Vec<Box<dyn ParserResult>>);
