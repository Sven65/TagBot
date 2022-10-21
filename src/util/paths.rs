use std::env;
use std::path::PathBuf;

pub struct Paths {
	pub prefix: PathBuf,
}

impl Paths {
	pub fn new() -> Paths {
		let mut tmp = match env::current_exe() {
			Ok(p) => p,
			Err(e) => panic!("Can't find exe path: {}", e),
		};
		tmp.pop();
		tmp.pop();
		tmp.pop();
		Paths { prefix: tmp }
	}
}
