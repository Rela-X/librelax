use Set;
use relation::Relation;
use relation::relation_tabular::RelationTabular;

#[derive(Debug)]
pub struct Intersection<'a, P, Q>
where P: 'a + Relation,
      Q: 'a + Relation,
{
	p: &'a P,
	q: &'a Q,
}

impl<'a, P, Q> Intersection<'a, P, Q>
where P: 'a + Relation,
      Q: 'a + Relation,
{
	pub fn new(p: &'a P, q: &'a Q) -> Intersection<'a, P, Q> {
		Intersection { p: p, q: q }
	}
}

impl<'a, P, Q> RelationTabular for Intersection<'a, P, Q>
where P: RelationTabular,
      Q: RelationTabular,
{
	fn get_domain(&self) -> (&Set, &Set) {
		self.p.get_domain()
	}
	fn eval_at(&self, ix: usize, iy: usize) -> bool {
		self.p.eval_at(ix, iy) && self.q.eval_at(ix, iy)
	}
}

impl<'a, R, P, Q> PartialEq<R> for Intersection<'a, P, Q>
where R: RelationTabular,
      P: RelationTabular,
      Q: RelationTabular,
{
	fn eq(&self, other: &R) -> bool {
		::relation::relation_tabular::eq(self, other)
	}
}
