use Set;
use relation::Relation;
use relation::relation_tabular::RelationTabular;

#[derive(Debug)]
pub struct Concatenation<'a, P, Q>
where P: 'a + Relation,
      Q: 'a + Relation,
{
	p: &'a P,
	q: &'a Q,
}

impl<'a, P, Q> Concatenation<'a, P, Q>
where P: 'a + Relation,
      Q: 'a + Relation,
{
	pub fn new(p: &'a P, q: &'a Q) -> Concatenation<'a, P, Q> {
		Concatenation { p: p, q: q }
	}
}

impl<'a, P, Q> RelationTabular for Concatenation<'a, P, Q>
where P: RelationTabular,
      Q: RelationTabular,
{
	fn get_domain(&self) -> (&Set, &Set) {
		(self.p.get_domain().0, self.q.get_domain().1)
	}
	fn eval_at(&self, ix: usize, iy: usize) -> bool {
		self.p.eval_at(ix, iy) && (self.q.iys()).any(|iz| self.q.eval_at(iy, iz))
	}
}

impl<'a, R, P, Q> PartialEq<R> for Concatenation<'a, P, Q>
where R: RelationTabular,
      P: RelationTabular,
      Q: RelationTabular,
{
	fn eq(&self, other: &R) -> bool {
		::relation::relation_tabular::eq(self, other)
	}
}
