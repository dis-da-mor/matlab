use crate::matrix::Matrix;
use std::str::FromStr;
use crate::tools::Searchable;

pub fn evaluate(input: &str) {
	let mat: Matrix = match input.parse() {
		Ok(m) => m,
		Err(_) => {
			println!("Failed to parse matrix...");
			return;
		}
	};

	println!("[");
	for row in 0..mat.rows() {
		for column in 0..mat.columns() {
			print!("{} ", mat.get(row, column).unwrap());
		}
		println!();
	}
	println!("]");
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
			row = Vec::new();
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
