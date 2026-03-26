#[allow(non_snake_case)]
pub mod Colours {
	pub const RESET: &str = r"\e[0m";
	pub const RED: &str = r"\e[31m";
}

pub fn println_error(str: String) {
	eprintln!("{}{}{}", Colours::RED, str, Colours::RESET);
}
