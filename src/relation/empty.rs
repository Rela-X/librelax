use Set;
use relation::RelationTabular;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Empty<'a> {
	set: &'a Set,
}

impl<'a> Empty<'a> {
	pub fn new(set: &Set) -> Empty {
		Empty {
			set: set,
		}
	}
}

impl<'a> RelationTabular for Empty<'a> {
	fn get_domain(&self) -> (&Set, &Set) {
		(&self.set, &self.set)
	}
	fn eval_at(&self, ix: usize, iy: usize) -> bool {
		return false;
	}
}
