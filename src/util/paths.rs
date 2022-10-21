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

		// This is a workaround to get the correct path when running
		// in a local development environment and in docker
		// as the executable gets placed at different locations
		if cfg!(debug_assertions) {
			tmp.pop();
			tmp.pop();
			tmp.pop();
		} else {
			tmp.pop();
		}

		Paths { prefix: tmp }
	}
}
