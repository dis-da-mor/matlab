pub trait Searchable {
	fn index_of(&self, search: &str) -> Option<usize>;
}

impl Searchable for str {
	fn index_of(&self, search: &str) -> Option<usize> {
		for i in 0..self.len() - search.len() {
			if self[i..i + search.len()] == *search {
				return Some(i);
			}
		}
		None
	}
}
