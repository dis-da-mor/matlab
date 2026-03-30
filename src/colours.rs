#[allow(non_snake_case)]
pub mod Colours {
	pub const RESET: &str = "\x1b[0m";
	pub const RED: &str = "\x1b[31m";
}

pub fn println_error(str: String) {
	eprintln!("{}{}{}", Colours::RED, str, Colours::RESET);
}
