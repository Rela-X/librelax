use std;
use relation::Relation;
use relation::relation_tabular::RelationTabular;

#[derive(Debug,PartialEq,Eq)]
pub struct Complement<'a, R: 'a + Relation> {
	r: &'a R,
}

impl<'a, R: 'a + Relation> Complement<'a, R> {
	pub fn new(r: &R) -> Complement<R> {
		Complement { r: r }
	}
}

impl<'a, R: 'a + RelationTabular> RelationTabular for Complement<'a, R> {
	type X = R::X;
	type Y = R::Y;
	fn get_domain(&self) -> (&[Self::X], &[Self::Y]) { self.r.get_domain() }
	fn eval_at(&self, ix: usize, iy: usize) -> bool { !self.r.eval_at(ix, iy) }
}

impl<'a, R: 'a + RelationTabular> PartialEq<R> for Complement<'a, R> {
	fn eq(&self, other: &R) -> bool {
		::relation::relation_tabular::eq(self, other)
	}
}
