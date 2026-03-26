use std::collections::BTreeMap;

use crate::tokeniser::{
	Operator,
	print_token,
	Token,
	TokenResult,
	tokenise
};

fn group_by_operators(expressions: &mut Vec<ExpressionElement>, operators: Vec<Operator>) {
	let mut i = 1;
	loop {
		if i > expressions.len() - 1 {
			break;
		}

		let token = match &expressions[i] {
			ExpressionElement::Token(t) => t.clone(),
			ExpressionElement::Expression(_) => {
				i += 1;
				continue
			}
		};
		if let Token::Operator(operator) = &token && operators.contains(operator) {
			i -= 1;
			let slice: &Vec<ExpressionElement> = &expressions.drain(i..i+3).collect();
			let expression = ExpressionElement::Expression(Box::new(Expression {
				lhs: slice[0].clone(),
				operator: *operator,
				rhs: slice[2].clone()
			}));
			expressions.insert(i, expression);
		}

		i += 1;
	}
}

fn tokens_to_expressions(tokens: &Vec<Token>) -> Result<Vec<ExpressionElement>, &str> {
	let mut expressions: Vec<ExpressionElement> = tokens.iter()
		.map(|token| ExpressionElement::Token(token.clone())).collect();

	group_by_operators(&mut expressions, vec![Operator::Power]);
	group_by_operators(&mut expressions, vec![Operator::Multiply, Operator::Divide]);
	group_by_operators(&mut expressions, vec![Operator::Add, Operator::Subtract]);
	group_by_operators(&mut expressions, vec![
		Operator::EqualTo,
		Operator::NotEqualTo,
		Operator::LessThan,
		Operator::LessThanOrEqualTo,
		Operator::GreaterThan,
		Operator::GreaterThanOrEqualTo
	]);
	group_by_operators(&mut expressions, vec![Operator::Assign]);

	Ok(expressions)
}

pub struct Evaluator {
	variables: BTreeMap<String, Token>
}

impl Evaluator {
	pub fn new() -> Self {
		Evaluator { variables: BTreeMap::new() }
	}
	pub fn evaluate(&mut self, input: &str) {
		let tokens = match tokenise(input) {
			Ok(t) => t,
			Err(err) => {
				eprintln!("Error: {}", err);
				return;
			}
		};

		let expression_list = match tokens_to_expressions(&tokens) {
			Ok(e) => e,
			Err(err) => {
				eprintln!("Error: {}", err);
				return;
			}
		};

		let result = match expression_list[0].evaluate(&mut self.variables) {
			Ok(t) => t,
			Err(err) => {
				println!("Error: {}", err);
				return;
			}
		};
		print_token(&result, &self.variables);
	}
}

fn add(lhs: &mut Token, rhs: &mut Token) -> TokenResult {
	match lhs {
		Token::Matrix(lhs) => {
			match rhs {
				Token::Number(rhs) => {
					lhs.add_scalar(*rhs);
					Ok(Token::Matrix(lhs.clone()))
				},
				Token::Matrix(rhs) => match lhs.checked_add(rhs) {
					Ok(m) => Ok(Token::Matrix(m.clone())),
					Err(err) => Err(err.to_owned())
				},
				_ => Err("Cannot add RHS to matrix".to_owned())
			}
		},
		Token::Number(lhs) => {
			match rhs {
				Token::Number(rhs) => {
					Ok(Token::Number(*lhs + *rhs))
				},
				Token::Matrix(rhs) => {
					rhs.add_scalar(*lhs);
					Ok(Token::Matrix(rhs.clone()))
				},
				_ => Err("Cannot add RHS to number".to_owned())
			}
		}
		_ => Err("Cannot add to LHS type".to_owned())
	}
}
fn subtract(lhs: &mut Token, rhs: &mut Token) -> TokenResult {
	match lhs {
		Token::Matrix(lhs) => {
			match rhs {
				Token::Number(rhs) => {
					lhs.subtract_scalar(*rhs);
					Ok(Token::Matrix(lhs.clone()))
				},
				Token::Matrix(rhs) => match &mut lhs.checked_subtract(rhs) {
					Ok(m) => Ok(Token::Matrix(m.clone())),
					Err(err) => Err(err.to_owned())
				},
				_ => Err("Cannot subtract RHS from matrix".to_owned())
			}
		},
		Token::Number(lhs) => {
			match rhs {
				Token::Number(rhs) => {
					Ok(Token::Number(*lhs - *rhs))
				},
				_ => Err("Cannot subtract RHS from number".to_owned())
			}
		}
		_ => Err("Cannot add to LHS type".to_owned())
	}
}
fn multiply(lhs: &mut Token, rhs: &mut Token) -> TokenResult {
	match lhs {
		Token::Number(lhs) => match rhs {
			Token::Number(rhs) => Ok(Token::Number(*lhs * *rhs)),
			Token::Matrix(rhs) => {
				rhs.multiply_by_scalar(*lhs);
				Ok(Token::Matrix(rhs.clone()))
			},
			_ => Err("Cannot multiply number by type of RHS".to_owned())
		},
		Token::Matrix(lhs) => match rhs {
			Token::Number(rhs) => {
				lhs.multiply_by_scalar(*rhs);
				Ok(Token::Matrix(lhs.clone()))
			},
			Token::Matrix(rhs) => {
				match lhs.checked_multiply(rhs) {
					Ok(m) => Ok(Token::Matrix(m)),
					Err(err) => Err(format!("{}", err))
				}
			},
			_ => Err("Cannot multiply matrix by type of RHS".to_owned())
		}
		_ => Err("Cannot multiply type of LHS".to_owned())
	}
}
fn divide(lhs: &mut Token, rhs: &mut Token) -> TokenResult {
	match lhs {
		Token::Number(lhs) => match rhs {
			Token::Number(rhs) => Ok(Token::Number(*lhs / *rhs)),
			_ => Err("Cannot divide by type of RHS".to_owned())
		},
		Token::Matrix(lhs) => match rhs {
			Token::Number(rhs) => {
				lhs.divide_by_scalar(*rhs);
				Ok(Token::Matrix(lhs.clone()))
			},
			_ => Err("Can only divide matrix by a number".to_owned())
		},
		_ => Err("Cannot divide type of LHS".to_owned())
	}
}
fn power(lhs: &mut Token, rhs: &mut Token) -> TokenResult {
	match lhs {
		Token::Number(lhs) => {
			match rhs {
				Token::Number(rhs) => Ok(Token::Number(lhs.powf(*rhs))),
				_ => Err("Cannot raise LHS by the type of RHS".to_owned())
			}
		},
		// Token::Matrix(lhs) => {},
		_ => Err("Cannot compute power of type of LHS".to_owned())
	}
}
fn equal_to(lhs: &mut Token, rhs: &mut Token) -> TokenResult {
	match lhs {
		Token::Number(lhs) => match rhs {
			Token::Number(rhs) => Ok(Token::Number(if lhs == rhs { 1.0 } else { 0.0 })),
			// error rather than silently fail because this is likely a mistake
			_ => Err("Cannot compare equality of different types".to_owned())
		},
		//Token::Matrix(lhs) => match rhs {
		//	Token::Matrix(rhs) => {}
		//}
		_ => Err("Cannot compare equality of LHS type".to_owned())
	}
}
fn not_equal_to(lhs: &mut Token, rhs: &mut Token) -> TokenResult {
	match lhs {
		Token::Number(lhs) => match rhs {
			Token::Number(rhs) => Ok(Token::Number(if lhs != rhs { 1.0 } else { 0.0 })),
			// error rather than silently fail because this is likely a mistake
			_ => Err("Cannot compare inequality of different types".to_owned())
		},
		//Token::Matrix(lhs) => match rhs {
		//	Token::Matrix(rhs) => {}
		//}
		_ => Err("Cannot compare inequality of LHS type".to_owned())
	}
}
fn less_than(lhs: &mut Token, rhs: &mut Token) -> TokenResult {
	match lhs {
		Token::Number(lhs) => match rhs {
			Token::Number(rhs) => Ok(Token::Number(if lhs < rhs { 1.0 } else { 0.0 })),
			// error rather than silently fail because this is likely a mistake
			_ => Err("Only numbers can be compared with less than".to_owned())
		},
		_ => Err("Only numbers can be compared with less than".to_owned())
	}
}
fn less_than_or_equal_to(lhs: &mut Token, rhs: &mut Token) -> TokenResult {
	match lhs {
		Token::Number(lhs) => match rhs {
			Token::Number(rhs) => Ok(Token::Number(if lhs <= rhs { 1.0 } else { 0.0 })),
			// error rather than silently fail because this is likely a mistake
			_ => Err("Only numbers can be compared with less than or equal to".to_owned())
		},
		_ => Err("Only numbers can be compared with less than or equal to".to_owned())
	}
}
fn greater_than(lhs: &mut Token, rhs: &mut Token) -> TokenResult {
	match lhs {
		Token::Number(lhs) => match rhs {
			Token::Number(rhs) => Ok(Token::Number(if lhs > rhs { 1.0 } else { 0.0 })),
			// error rather than silently fail because this is likely a mistake
			_ => Err("Only numbers can be compared with greater than".to_owned())
		},
		_ => Err("Only numbers can be compared with greater than".to_owned())
	}
}
fn greater_than_or_equal_to(lhs: &mut Token, rhs: &mut Token) -> TokenResult {
	match lhs {
		Token::Number(lhs) => match rhs {
			Token::Number(rhs) => Ok(Token::Number(if lhs >= rhs { 1.0 } else { 0.0 })),
			// error rather than silently fail because this is likely a mistake
			_ => Err("Only numbers can be compared with greater than or equal to".to_owned())
		},
		_ => Err("Only numbers can be compared with greater than or equal to".to_owned())
	}
}

#[derive(Clone, Debug)]
enum ExpressionElement {
	Token(Token),
	Expression(Box<Expression>)
}

impl ExpressionElement {
	fn evaluate(&self, variables: &mut BTreeMap<String, Token>) -> TokenResult {
		let expression = match self {
			ExpressionElement::Expression(e) => e,
			ExpressionElement::Token(t) => return Ok(t.clone())
		};

		// recursively evaluate expression tree
		// if either we are still left with an expression, something has gone very bad
		let lhs = &expression.lhs;
		let mut lhs = match &expression.lhs {
			ExpressionElement::Token(t) => t.clone(),
			ExpressionElement::Expression(_) => match lhs.evaluate(variables) {
				Ok(t) => t,
				Err(err) => return Err(err)
			}
		};
		let rhs = &expression.rhs;
		let mut rhs = match &expression.rhs {
			ExpressionElement::Token(t) => t.clone(),
			ExpressionElement::Expression(_) => match rhs.evaluate(variables) {
				Ok(t) => t,
				Err(err) => return Err(err)
			}
		};

		// dereference variables (skip LHS on assign operations)
		if expression.operator != Operator::Assign && let Token::Variable(variable) = lhs {
			lhs = match variables.get(&variable) {
				Some(t) => t.clone(),
				None => return Err(format!("Could not retrieve variable '{}'", variable))
			};
		}
		if let Token::Variable(variable) = rhs {
			rhs = match variables.get(&variable) {
				Some(t) => t.clone(),
				None => return Err(format!("Could not retrieve variable '{}'", variable))
			};
		}

		match expression.operator {
			Operator::Power => power(&mut lhs, &mut rhs),
			Operator::Add => add(&mut lhs, &mut rhs),
			Operator::Subtract => subtract(&mut lhs, &mut rhs),
			Operator::Multiply => multiply(&mut lhs, &mut rhs),
			Operator::Divide => divide(&mut lhs, &mut rhs),
			Operator::EqualTo => equal_to(&mut lhs, &mut rhs),
			Operator::NotEqualTo => not_equal_to(&mut lhs, &mut rhs),
			Operator::LessThan => less_than(&mut lhs, &mut rhs),
			Operator::LessThanOrEqualTo => less_than_or_equal_to(&mut lhs, &mut rhs),
			Operator::GreaterThan => greater_than(&mut lhs, &mut rhs),
			Operator::GreaterThanOrEqualTo => greater_than_or_equal_to(&mut lhs, &mut rhs),
			Operator::Assign => {
				let variable = match lhs {
					Token::Variable(v) => v,
					_ => return Err("Can only assign to variables, maybe you meant to compare with '=='?".to_owned())
				};
				variables.insert(variable.to_string(), rhs.clone());
				Ok(rhs.clone())
			},
			#[allow(unreachable_patterns)]
			_ => return Err("Unknown operator".to_owned())
		}
	}
}

#[derive(Clone, Debug)]
struct Expression {
	lhs: ExpressionElement,
	rhs: ExpressionElement,
	operator: Operator
}
