use Set;
use relation::RelationTabular;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Universal<'a> {
	set: &'a Set,
}

impl<'a> Universal<'a> {
	pub fn new(set: &Set) -> Universal {
		Universal {
			set: set,
		}
	}
}

impl<'a> RelationTabular for Universal<'a> {
	fn get_domain(&self) -> (&Set, &Set) {
		(&self.set, &self.set)
	}
	fn eval_at(&self, ix: usize, iy: usize) -> bool {
		return true;
	}
}
