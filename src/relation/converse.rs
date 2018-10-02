use Set;
use relation::Relation;
use relation::relation_tabular::RelationTabular;

#[derive(Debug)]
pub struct Converse<'a, R>
where R: 'a + Relation,
{
	r: &'a R,
}

impl<'a, R> Converse<'a, R>
where R: 'a + Relation,
{
	pub fn new(r: &R) -> Converse<R> {
		Converse{ r: r }
	}
}

impl<'a, R> RelationTabular for Converse<'a, R>
where R: 'a + RelationTabular,
{
	fn get_domain(&self) -> (&Set, &Set) {
		self.r.get_domain()
	}
	fn eval_at(&self, ix: usize, iy: usize) -> bool {
		self.r.eval_at(iy, ix)
	}
}

impl<'a, P, Q> PartialEq<Q> for Converse<'a, P>
where P: 'a + RelationTabular,
      Q: 'a + RelationTabular,
{
	fn eq(&self, other: &Q) -> bool {
		::relation::relation_tabular::eq(self, other)
	}
}
