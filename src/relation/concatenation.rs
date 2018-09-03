use std;
use relation::Relation;
use relation::relation_tabular::RelationTabular;

#[derive(Debug)]
pub struct Concatenation<'a, P, Q, XX, YY, ZZ>
where P: 'a + Relation<X=XX, Y=YY>,
      Q: 'a + Relation<X=YY, Y=ZZ>,
{
	p: &'a P,
	q: &'a Q,
}

impl<'a, P, Q, XX, YY, ZZ> Concatenation<'a, P, Q, XX, YY, ZZ>
where P: 'a + Relation<X=XX, Y=YY>,
      Q: 'a + Relation<X=YY, Y=ZZ>,
{
	pub fn new(p: &'a P, q: &'a Q) -> Concatenation<'a, P, Q, XX, YY, ZZ> {
		Concatenation { p: p, q: q }
	}
}

impl<'a, P, Q, XX, YY, ZZ> RelationTabular for Concatenation<'a, P, Q, XX, YY, ZZ>
where P: RelationTabular<X=XX, Y=YY>,
      Q: RelationTabular<X=YY, Y=ZZ>,
      XX: PartialEq<YY> + PartialEq<ZZ> + Eq + std::fmt::Debug,
      YY: PartialEq<XX> + PartialEq<ZZ> + Eq + std::fmt::Debug,
      ZZ: PartialEq<XX> + PartialEq<YY> + Eq + std::fmt::Debug,
{
	type X = XX;
	type Y = ZZ;
	fn get_domain(&self) -> (&[Self::X], &[Self::Y]) { (self.p.get_domain().0, self.q.get_domain().1) }
	fn eval_at(&self, ix: usize, iy: usize) -> bool {
		self.p.eval_at(ix, iy) && (0..self.q.get_domain().1.len()).any(|iz| self.q.eval_at(iy, iz))
	}
}

impl<'a, R, P, Q, XX, YY, ZZ> PartialEq<R> for Concatenation<'a, P, Q, XX, YY, ZZ>
where R: RelationTabular<X=XX, Y=ZZ>,
      P: RelationTabular<X=XX, Y=YY>,
      Q: RelationTabular<X=YY, Y=ZZ>,
      XX: PartialEq<YY> + PartialEq<ZZ> + Eq + std::fmt::Debug,
      YY: PartialEq<XX> + PartialEq<ZZ> + Eq + std::fmt::Debug,
      ZZ: PartialEq<XX> + PartialEq<YY> + Eq + std::fmt::Debug,
{
	fn eq(&self, other: &R) -> bool {
		::relation::relation_tabular::eq(self, other)
	}
}
