use std;
use relation::Relation;
use relation::relation_tabular::RelationTabular;

#[derive(Debug)]
pub struct Complement<'a, R>
where R: 'a + Relation,
{
	r: &'a R,
}

impl<'a, R> Complement<'a, R>
where R: 'a + Relation,
{
	pub fn new(r: &R) -> Complement<R> {
		Complement { r: r }
	}
}

impl<'a, R> RelationTabular for Complement<'a, R>
where R: 'a + RelationTabular,
{
	type X = R::X;
	type Y = R::Y;
	fn get_domain(&self) -> (&[Self::X], &[Self::Y]) { self.r.get_domain() }
	fn eval_at(&self, ix: usize, iy: usize) -> bool { !self.r.eval_at(ix, iy) }
}

impl<'a, R, RR, XX, YY> PartialEq<R> for Complement<'a, RR>
where R: 'a + RelationTabular<X=XX, Y=YY>,
      RR: 'a + RelationTabular<X=XX, Y=YY>,
      XX: PartialEq<YY> + Eq + std::fmt::Debug,
      YY: PartialEq<XX> + Eq + std::fmt::Debug,
{
	fn eq(&self, other: &R) -> bool {
		::relation::relation_tabular::eq(self, other)
	}
}
