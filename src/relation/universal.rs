use std::vec::Vec;

use set::SetElement;
use relation::RelationTabular;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Universal<'a> {
	set: &'a Vec<SetElement>,
}

impl<'a> Universal<'a> {
	pub fn new(set: &Vec<SetElement>) -> Universal {
		Universal {
			set: set,
		}
	}
}

impl<'a> RelationTabular for Universal<'a> {
	fn get_domain(&self) -> (&[SetElement], &[SetElement]) {
		(&self.set, &self.set)
	}
	fn eval_at(&self, ix: usize, iy: usize) -> bool {
		return true;
	}
}
