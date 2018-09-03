use std;
use relation::Relation;
use relation::relation_tabular::RelationTabular;

#[derive(Debug)]
pub struct Union<'a, P, Q, XX, YY>
where P: 'a + Relation<X=XX, Y=YY>,
      Q: 'a + Relation<X=XX, Y=YY>,
{
	p: &'a P,
	q: &'a Q,
}

impl<'a, P, Q, XX, YY> Union<'a, P, Q, XX, YY>
where P: 'a + Relation<X=XX, Y=YY>,
      Q: 'a + Relation<X=XX, Y=YY>,
{
	pub fn new(p: &'a P, q: &'a Q) -> Union<'a, P, Q, XX, YY> {
		Union { p: p, q: q }
	}
}

impl<'a, P, Q, XX, YY> RelationTabular for Union<'a, P, Q, XX, YY>
where P: RelationTabular<X=XX, Y=YY>,
      Q: RelationTabular<X=XX, Y=YY>,
      XX: PartialEq<YY> + Eq + std::fmt::Debug,
      YY: PartialEq<XX> + Eq + std::fmt::Debug,
{
	type X = XX;
	type Y = YY;
	fn get_domain(&self) -> (&[Self::X], &[Self::Y]) { self.p.get_domain() }
	fn eval_at(&self, ix: usize, iy: usize) -> bool { self.p.eval_at(ix, iy) || self.q.eval_at(ix, iy) }
}

impl<'a, R, P, Q, XX, YY> PartialEq<R> for Union<'a, P, Q, XX, YY>
where R: RelationTabular<X=XX, Y=YY>,
      P: RelationTabular<X=XX, Y=YY>,
      Q: RelationTabular<X=XX, Y=YY>,
      XX: PartialEq<YY> + Eq + std::fmt::Debug,
      YY: PartialEq<XX> + Eq + std::fmt::Debug,
{
	fn eq(&self, other: &R) -> bool {
		::relation::relation_tabular::eq(self, other)
	}
}
