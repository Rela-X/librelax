use std::vec::Vec;

use SetElement;
use relation::RelationTabular;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Identity<'a> {
	set: &'a Vec<SetElement>,
}

impl<'a> Identity<'a> {
	pub fn new(set: &Vec<SetElement>) -> Identity {
		Identity {
			set: set,
		}
	}
}

impl<'a> RelationTabular for Identity<'a> {
	fn get_domain(&self) -> (&[SetElement], &[SetElement]) {
		(&self.set, &self.set)
	}
	fn eval_at(&self, ix: usize, iy: usize) -> bool {
		return ix == iy;
	}
}
