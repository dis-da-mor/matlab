 use std::collections::BTreeMap;
use std::fmt::{Display, Error, write};
use std::str::FromStr;

use crate::matrix::Matrix;
use crate::tools::Searchable;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Operator {
	Add,
	Subtract,
	Multiply,
	Divide,
	Power,
	Not,
	EqualTo,
	NotEqualTo,
	LessThan,
	LessThanOrEqualTo,
	GreaterThan,
	GreaterThanOrEqualTo,
	Assign
}

impl Display for Operator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Operator::Add                  => write!(f, "Add"),
			Operator::Subtract             => write!(f, "Subtract"),
			Operator::Multiply             => write!(f, "Multiply"),
			Operator::Divide               => write!(f, "Divide"),
			Operator::Power                => write!(f, "Power"),
			Operator::Not                  => write!(f, "Not"),
			Operator::EqualTo              => write!(f, "EqualTo"),
			Operator::NotEqualTo           => write!(f, "NotEqualTo"),
			Operator::LessThan             => write!(f, "LessThan"),
			Operator::LessThanOrEqualTo    => write!(f, "LessThanOrEqualTo"),
			Operator::GreaterThan          => write!(f, "GreaterThan"),
			Operator::GreaterThanOrEqualTo => write!(f, "GreaterThanOrEqualTo"),
			Operator::Assign               => write!(f, "Assign")
		}
	}
}

#[derive(Clone, Debug)]
pub enum Token {
	Number(f64),
	Variable(String),
	Operator(Operator),
	Function(String),
	Matrix(Matrix)
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum TokenType {
	None,
	Number,
	Variable,
	Operator,
	Function,
	Matrix
}

pub type TokenResult = Result<Token, String>;

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

fn parse_operator(operator: &str) -> Result<Operator, &str> {
	Ok(match operator {
		"^" | "**" => Operator::Power,
		"*"        => Operator::Multiply,
		"/"        => Operator::Divide,
		"+"        => Operator::Add,
		"-"        => Operator::Subtract,
		"=="       => Operator::EqualTo,
		"!="       => Operator::NotEqualTo,
		"<"        => Operator::LessThan,
		">"        => Operator::GreaterThan,
		"<="       => Operator::LessThanOrEqualTo,
		">="       => Operator::GreaterThanOrEqualTo,
		"!"        => Operator::Not,
		"="        => Operator::Assign,
		_ => return Err("Unknown operator")
	})
}

fn parse_token(raw_token: &str, token_type: TokenType) -> TokenResult {
	if raw_token.is_empty() {
		return Err("Token is empty".to_owned());
	}

	match token_type {
			TokenType::Number => {
				let num: f64 = match raw_token.parse() {
					Ok(n) => n,
					Err(_) => return Err(format!("Number parsing error: '{}'", raw_token))
				};

				Ok(Token::Number(num))
			},
			TokenType::Variable => Ok(Token::Variable(raw_token.to_owned())),
			TokenType::Operator => {
				let operator = match parse_operator(raw_token) {
					Ok(o) => o,
					Err(err) => return Err(err.to_owned()),
				};

				Ok(Token::Operator(operator))
			},
			TokenType::Matrix => {
				let mat: Matrix = match raw_token.parse() {
					Ok(m) => m,
					Err(err) => return Err(format!("Matrix parsing error: '{}'", err))
				};

				Ok(Token::Matrix(mat))
			},
			_ => Err(format!("Token type not implemented yet."))
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

pub fn tokenise(input: &str) -> Result<Vec<Token>, String> {
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
					match push_token(&mut tokens, &accum, accum_type) {
						Ok(_) => {},
						Err(err) => return Err(err)
					}
					accum_type = TokenType::None;
				}
			},
			TokenType::Variable => {
				if char.is_alphabetic() {
					accum.push(char);
				} else if char == '(' {
					accum_type = TokenType::Function;
					accum.push(char);
				} else {
					match push_token(&mut tokens, &accum, accum_type) {
						Ok(_) => {},
						Err(err) => return Err(err)
					}
					accum_type = TokenType::None;
				}
			},
			TokenType::Function => {
				accum.push(char);

				if char == ')' {
					match push_token(&mut tokens, &accum, accum_type) {
						Ok(_) => {},
						Err(err) => return Err(err)
					}
					accum_type = TokenType::None;
				}
			},
			TokenType::Operator => {
				if accum.len() < 2 && is_operator(char) {
					accum.push(char);
				} else {
					match push_token(&mut tokens, &accum, accum_type) {
						Ok(_) => {},
						Err(err) => return Err(err)
					}
					accum_type = TokenType::None;
				}
			},
			TokenType::Matrix => {
				accum.push(char);

				if char == ']' {
					match push_token(&mut tokens, &accum, accum_type) {
						Ok(_) => {},
						Err(err) => return Err(err)
					}
					accum_type = TokenType::None;
				}
			},
			_ => {}
		}
	}

	if accum_type != TokenType::None && !accum.is_empty() {
		match push_token(&mut tokens, &accum, accum_type) {
			Ok(_) => {},
			Err(err) => return Err(err)
		}
	}

	Ok(tokens)
}

pub fn print_token(token: &Token, variables: &BTreeMap<String, Token>) {
	match token {
		Token::Number(n) => println!("{}", n),
		Token::Operator(o) => println!("{}", o),
		Token::Matrix(m) => println!("{}", m),
		Token::Variable(v) => {
			let value = match variables.get(v) {
				Some(v) => v,
				None => return
			};
			print!("{} = ", v);
			print_token(value, variables);
			println!();
		},
		_ => {}
	}
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
		if row.len() > 0 {
			rows.push(row);
		}

		// validate matrix
		if rows.len() == 0 {
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
