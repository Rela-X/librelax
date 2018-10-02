use Set;
use relation::RelationTabular;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Identity<'a> {
	set: &'a Set,
}

impl<'a> Identity<'a> {
	pub fn new(set: &Set) -> Identity {
		Identity {
			set: set,
		}
	}
}

impl<'a> RelationTabular for Identity<'a> {
	fn get_domain(&self) -> (&Set, &Set) {
		(&self.set, &self.set)
	}
	fn eval_at(&self, ix: usize, iy: usize) -> bool {
		return ix == iy;
	}
}
