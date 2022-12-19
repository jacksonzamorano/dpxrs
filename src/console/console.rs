use std::io::{Stdin, Stdout, stdin, stdout, Write};

pub struct Console {
	input: Stdin,
	output: Stdout
}
impl Console {

	pub fn create() -> Console {
		Console {
			input: stdin(),
			output: stdout()
		}
	}	

	pub fn print(&self, input: &String) {
		let mut op = self.output.lock();
		_ = op.write(input.as_bytes());
	}	

	pub fn print_static(&self, input: &str) {
		let mut op = self.output.lock();
		_ = op.write(input.as_bytes());
	}

	pub fn get(&mut self) -> String {
		let mut input = String::new();
		_ = self.output.flush();
		_ = self.input.read_line(&mut input);
		input = input.replace("\n", "");
		return input;
	}
}