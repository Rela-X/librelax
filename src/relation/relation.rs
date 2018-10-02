use relation::union::Union;
use relation::intersection::Intersection;
use relation::complement::Complement;
use relation::converse::Converse;
use relation::concatenation::Concatenation;

use set;

pub trait Relation {
	fn eval(&self, x: &set::SetElement, y: &set::SetElement) -> bool;

	fn is_homogeneous(&self) -> bool;
	fn is_heterogeneous(&self) -> bool { !self.is_homogeneous() }

	fn is_injective(&self) -> bool; /* aka left-unique */
	fn is_functional(&self) -> bool; /* aka univalent, right-unique, right-definite */
	fn is_surjective(&self) -> bool; /* aka onto, right-total */
	fn is_lefttotal(&self) -> bool;
	fn is_bijective(&self) -> bool { self.is_injective() && self.is_surjective() }
	fn is_function(&self) -> bool { self.is_functional() && self.is_lefttotal() }

	fn union<'a, P, Q>(p: &'a P, q: &'a Q) -> Union<'a, P, Q>
	where P: Relation,
	      Q: Relation,
	{
		Union::new(p, q)
	}
	fn intersection<'a, P, Q>(p: &'a P, q: &'a Q) -> Intersection<'a, P, Q>
	where P: Relation,
	      Q: Relation,
	{
		Intersection::new(p, q)
	}
	fn complement<R>(r: &R) -> Complement<R>
	where R: Relation,
	{
		Complement::new(r)
	}
	fn converse<R>(r: &R) -> Converse<R>
	where R: Relation,
	{
		Converse::new(r)
	}
	fn concatenation<'a, P, Q>(p: &'a P, q: &'a Q) -> Concatenation<'a, P, Q>
	where P: Relation,
	      Q: Relation,
	{
		Concatenation::new(p, q)
	}
}

mod tests {
	use super::*;

	pub fn relation_property_test<R>(r: &R)
	where R: Relation
	{
		assert_eq!(r.is_homogeneous(), !r.is_heterogeneous());

		assert_eq!(r.is_bijective(), r.is_injective() && r.is_surjective());
		assert_eq!(r.is_function(), r.is_functional() && r.is_lefttotal());
	}
}
