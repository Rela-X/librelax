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

	fn union<P, Q, X, Y>(p: P, q: Q) -> Union<P, Q, X, Y>
	where P: Relation<X=X,Y=Y>,
	      Q: Relation<X=X,Y=Y>,
	{
		Union { p: p, q: q }
	}
	fn intersection<R: Relation>(p: R, q: R) -> Intersection<R> {
		Intersection { p: p, q: q }
	}
	fn complement<R: Relation>(r: R) -> Complement<R> {
		Complement { r: r }
	}
	fn converse<R: Relation>(r: R) -> Converse<R> {
		Converse { r: r }
	}
	fn concatenation<R: Relation>(p: R, q: R) -> Concatenation<R> {
		Concatenation { p: p, q: q }
	}
}

/*
 * No implementations, because performance would be (even more) terrible
 */
#[derive(Debug)]
pub struct Union<P, Q, X, Y>
where P: Relation<X=X,Y=Y>,
      Q: Relation<X=X,Y=Y>,
{
	p: P,
	q: Q,
}
//impl<T: Relation> Relation for Union<T> {}

#[derive(Debug,PartialEq,Eq)]
pub struct Intersection<T: Relation> {
	p: T,
	q: T,
}
//impl<T: Relation> Relation for Intersection<T> {}

#[derive(Debug,PartialEq,Eq)]
pub struct Complement<T: Relation> {
	r: T,
}
//impl<T: Relation> Relation for Complement<T> {}

#[derive(Debug,PartialEq,Eq)]
pub struct Converse<T: Relation> {
	r: T,
}
//impl<T: Relation> Relation for Converse<T> {}

#[derive(Debug,PartialEq,Eq)]
pub struct Concatenation<T: Relation> {
	p: T,
	q: T,
}
//impl<T: Relation> Relation for Concatenation<T> {}
