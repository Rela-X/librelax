use std::vec::Vec;

use SetElement;
use relation::RelationTabular;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Empty<'a> {
	set: &'a Vec<SetElement>,
}

impl<'a> Empty<'a> {
	pub fn new(set: &Vec<SetElement>) -> Empty {
		Empty {
			set: set,
		}
	}
}

impl<'a> RelationTabular for Empty<'a> {
	fn get_domain(&self) -> (&[SetElement], &[SetElement]) {
		(&self.set, &self.set)
	}
	fn eval_at(&self, ix: usize, iy: usize) -> bool {
		return false;
	}
}
