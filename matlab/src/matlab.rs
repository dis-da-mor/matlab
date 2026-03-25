use crate::matrix::Matrix;
use std::str::FromStr;
use crate::tools::Searchable;

use std::collections::BTreeMap;

fn is_number(char: char) -> bool {
	char.is_ascii_digit() || char == '.'
}
fn is_matrix(char: char) -> bool {
	char == '['
}
fn is_operator(char: char) -> bool {
	"+-*/<>!=".contains(char)
}
fn is_variable(char: char) -> bool {
	char.is_alphabetic()
}

fn parse_token(raw_token: &str, token_type: TokenType) -> Result<Token, String> {
	if raw_token.is_empty() {
		return Err("Token is empty".to_owned());
	}

	match token_type {
			TokenType::Number => {
				let num: f64 = match raw_token.parse() {
					Ok(n) => n,
					Err(_) => return Err(format!("Failed to parse number '{}'", raw_token))
				};

				Ok(Token::Number(num))
			},
			TokenType::Variable => Ok(Token::Variable(raw_token.to_string())),
			TokenType::Operator => Ok(Token::Operator(raw_token.to_string())),
			TokenType::Matrix => {
				let mat: Matrix = match raw_token.parse() {
					Ok(m) => m,
					Err(_) => return Err(format!("Failed to parse matrix '{}'", raw_token))
				};

				Ok(Token::Matrix(mat))
			},
			_ => Err("Token type not implemented yet.".to_owned())
		}
}
fn push_token(tokens: &mut Vec<Token>, raw_token: &str, token_type: TokenType) -> Result<(), String> {
	let token = match parse_token(raw_token , token_type) {
		Ok(t) => t,
		Err(err) => return Err(err)
	};
	tokens.push(token);
	Ok(())
}

fn tokenise(input: &str) -> Result<Vec<Token>, String> {
	let mut tokens: Vec<Token> = Vec::new();

	let mut accum_type = TokenType::None;
	let mut accum = String::new();
	for char in input.chars() {
		// determine type of current token if unknown
		if accum_type == TokenType::None {
			if char.is_whitespace() {
				continue;
			} else if is_number(char) {
				accum_type = TokenType::Number;
			} else if is_matrix(char) {
				accum_type = TokenType::Matrix;
			} else if is_operator(char) {
				accum_type = TokenType::Operator;
			} else if is_variable(char) {
				accum_type = TokenType::Variable;
			}
			accum = String::new();
			accum.push(char);
			continue;
		}

		// accumulate current token
		match accum_type {
			TokenType::Number => {
				if is_number(char) {
					accum.push(char);
				} else {
					let _ = push_token(&mut tokens, &accum, accum_type);
					accum_type = TokenType::None;
				}
			},
			TokenType::Variable => {
				if char.is_alphabetic() {
					accum.push(char);
				} else {
					let _ = push_token(&mut tokens, &accum, accum_type);
					accum_type = TokenType::None;
				}
			},
			TokenType::Operator => {
				if accum.len() < 2 && is_operator(char) {
					accum.push(char);
				} else {
					let _ = push_token(&mut tokens, &accum, accum_type);
					accum_type = TokenType::None;
				}
			},
			TokenType::Matrix => {
				accum.push(char);

				if char == ']' {
					let _ = push_token(&mut tokens, &accum, accum_type);
					accum_type = TokenType::None;
				}
			},
			_ => {}
		}
	}

	if !accum.is_empty() {
		let _ = push_token(&mut tokens, &accum, accum_type);
	}

	Ok(tokens)
}

pub fn evaluate(input: &str) {
	static mut VARIABLES: BTreeMap<String, f64> = BTreeMap::new();

	let tokens = match tokenise(input) {
		Ok(t) => t,
		Err(err) => {
			eprintln!("Error: {}", err);
			return;
		}
	};

	// TODO
	// start evaluating!!!
}

enum Variable {
	Bool(bool),
	Number(f64),
	Matrix(Matrix)
}

#[derive(Clone)]
enum Token {
	Number(f64),
	Variable(String),
	Operator(String),
	Matrix(Matrix)
}

#[derive(Copy, Clone, PartialEq)]
enum TokenType {
	None,
	Number,
	Variable,
	Operator,
	Matrix
}

impl FromStr for Matrix {
	type Err = String;

	fn from_str(string: &str) -> Result<Self, Self::Err> {
		let mut rows: Vec<Vec<f64>> = Vec::new();

		// parse matrix
		let matrix_start = match string.index_of("[") {
			Some(idx) => idx + 1,
			None => 0
		};
		let mut accum = String::new();
		let mut row: Vec<f64> = Vec::new();
		for char in string.chars().skip(matrix_start) {
			if char == ']' {
				break;
			}

			if char.is_ascii_digit() || char == '.' || char == '-' {
				accum.push(char);
			} else if char == ' ' || char == ',' || char == ';' {
				if accum.len() > 0 {
					let num: f64 = match accum.parse() {
						Ok(n) => n,
						Err(_) => return Err("Failed to parse value.".to_owned())
					};

					row.push(num);
					accum = String::new();
				}
			}
			
			if char == ';' {
				rows.push(row);
				row = Vec::new();
			}
		}

		// flush current values
		if accum.len() > 0 {
			let num: f64 = match accum.parse() {
				Ok(n) => n,
				Err(_) => return Err("Failed to parse value.".to_owned())
			};
			
			row.push(num);
		}
		if rows.len() > 0 {
			rows.push(row);
		}

		// validate matrix
		if rows.len() == 0 || rows[0].len() == 0 {
			return Ok(Matrix::new(0, 0));
		}
		for row in rows.iter().skip(1) {
			if row.len() != rows[0].len() {
				return Err("Row lengths don't match.".to_owned());
			}
		}

		// populate matrix
		let mut mat = Matrix::new(rows.len(), rows[0].len());
		for row in 0..rows.len() {
			for column in 0..rows[row].len() {
				match mat.set(rows[row][column], row, column) {
					Ok(_) => {},
					Err(str) => return Err(format!("Error populating matrix: {}", str))
				}
			}
		}

		Ok(mat)
	}
}
