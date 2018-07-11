//#![allow(dead_code)]

pub mod relation_tabular;
pub mod relation_vec;

pub trait Relation {
	type X;
	type Y;
//	type Domain = (&Self::X, &Self::Y);

	fn eval(&self, x: &Self::X, y: &Self::Y) -> bool;

	fn is_homogeneous(&self) -> bool;
	fn is_heterogeneous(&self) -> bool { !self.is_homogeneous() }

	fn is_reflexive(&self) -> bool; /* xRx */
	fn is_irreflexive(&self) -> bool; /* aka strict */
	fn is_antisymmetric(&self) -> bool;
	fn is_transitive(&self) -> bool;

	fn is_symmetric(&self) -> bool; /* xRy <=> yRx */
	fn is_asymmetric(&self) -> bool { self.is_irreflexive() && self.is_antisymmetric() }

	fn is_preorder(&self) -> bool { self.is_reflexive() && self.is_transitive() }
	fn is_partial_order(&self) -> bool { self.is_preorder() && self.is_antisymmetric() }
	fn is_equivalent(&self) -> bool { self.is_preorder() && self.is_symmetric() }

	fn is_difunctional(&self) -> bool; /* aka regular: xRy & zRy & zRw => xRw */

//	fn is_lattice(&self) -> bool; // TODO Result
//	fn is_sublattice(&self, other: &Relation) -> bool; // TODO Result

	fn is_injective(&self) -> bool; /* aka left-unique */
	fn is_functional(&self) -> bool; /* aka univalent, right-unique, right-definite */
	fn is_surjective(&self) -> bool; /* aka right-total, onto */
	fn is_lefttotal(&self) -> bool;
	fn is_bijective(&self) -> bool { self.is_injective() && self.is_surjective() }
	fn is_function(&self) -> bool { self.is_functional() && self.is_lefttotal() }

	fn union<'a, P, Q, XX, YY>(p: &'a P, q: &'a Q) -> Union<'a, P, Q, XX, YY>
	where P: Relation<X=XX, Y=YY>,
	      Q: Relation<X=XX, Y=YY>,
	{
		Union { p: p, q: q }
	}
	fn intersection<'a, P, Q, XX, YY>(p: &'a P, q: &'a Q) -> Intersection<'a, P, Q, XX, YY>
	where P: Relation<X=XX, Y=YY>,
	      Q: Relation<X=XX, Y=YY>,
	{
		Intersection { p: p, q: q }
	}
	fn complement<R: Relation>(r: &R) -> Complement<R> {
		Complement { r: r }
	}
	fn converse<R: Relation>(r: &R) -> Converse<R> {
		Converse { r: r }
	}
	fn concatenation<'a, P, Q, XX, YY, ZZ>(p: &'a P, q: &'a Q) -> Concatenation<'a, P, Q, XX, YY, ZZ>
	where P: 'a + Relation<X=XX, Y=YY>,
	      Q: 'a + Relation<X=YY, Y=ZZ>,
	{
		Concatenation { p: p, q: q }
	}
}

/*
 * No implementations, because performance would be (even more) terrible
 */
#[derive(Debug)]
pub struct Union<'a, P, Q, XX, YY>
where P: 'a + Relation<X=XX, Y=YY>,
      Q: 'a + Relation<X=XX, Y=YY>,
{
	p: &'a P,
	q: &'a Q,
}

#[derive(Debug)]
pub struct Intersection<'a, P, Q, XX, YY>
where P: 'a + Relation<X=XX, Y=YY>,
      Q: 'a + Relation<X=XX, Y=YY>,
{
	p: &'a P,
	q: &'a Q,
}

#[derive(Debug,PartialEq,Eq)]
pub struct Complement<'a, R: 'a + Relation> {
	r: &'a R,
}

#[derive(Debug,PartialEq,Eq)]
pub struct Converse<'a, R: 'a + Relation> {
	r: &'a R,
}

#[derive(Debug)]
pub struct Concatenation<'a, P, Q, XX, YY, ZZ>
where P: 'a + Relation<X=XX, Y=YY>,
      Q: 'a + Relation<X=YY, Y=ZZ>,
{
	p: &'a P,
	q: &'a Q,
}

mod tests {
	use super::*;

	fn relation_property_test<R>(r: &R)
	where R: Relation
	{
		assert_eq!(r.is_homogeneous(), !r.is_heterogeneous());
		assert_eq!(r.is_reflexive() && r.is_irreflexive(), false);

		assert_eq!(r.is_symmetric() && r.is_antisymmetric(), false);
		assert_eq!(r.is_asymmetric(), r.is_irreflexive() && r.is_antisymmetric());

		assert_eq!(r.is_preorder(), r.is_reflexive() && r.is_transitive());
		assert_eq!(r.is_partial_order(), r.is_preorder() && r.is_antisymmetric());
		assert_eq!(r.is_equivalent(), r.is_preorder() && r.is_symmetric());

		assert_eq!(r.is_bijective(), r.is_injective() && r.is_surjective());
		assert_eq!(r.is_function(), r.is_functional() && r.is_lefttotal());
	}
}
