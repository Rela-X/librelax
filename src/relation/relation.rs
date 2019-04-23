//! A binary [`Relation`]
//!
//! This module contains the `Relation` type aswell as types for
//! various operations such as Union and Intersection.

use std;
use crate::cow::LCow;

use crate::set::{Set, SetElement};

pub trait Relation : Clone {
	fn get_domain(&self) -> (&Set, &Set);
	/// Evaluate if x relates to y, i.e. for a Relation `R`
	/// `R.eval(x, y)` returns `true ⇔ xRy` and `false` otherwise.
	fn eval(&self, x: &SetElement, y: &SetElement) -> bool {
		// TODO O(log n) should be possible here
		let ix = self.get_domain().0.iter().position(|e| e == x).unwrap();
		let iy = self.get_domain().1.iter().position(|e| e == y).unwrap();
		return self.eval_at(ix, iy);
	}
	/// Evaluate the `Relation`s incidence matrix at column `ix` and row `iy`.
	fn eval_at(&self, ix: usize, iy: usize) -> bool;

	/// [`Range`] of the column-indices of the `Relation`s incidence matrix.
	fn ixs(&self) -> std::ops::Range<usize> { 0..self.get_domain().0.cardinality() }
	/// [`Range`] of the row-indices of the `Relation`s incidence matrix.
	fn iys(&self) -> std::ops::Range<usize> { 0..self.get_domain().1.cardinality() }

	fn is_homogeneous(&self) -> bool { self.get_domain().0 == self.get_domain().1 }
	fn is_heterogeneous(&self) -> bool { !self.is_homogeneous() }

	/// Return `true` if the relation is injective.
	/// A relation is injective iff `∀x,z ∈ X: xRy ∧ zRy ⇔ x = z`
	///
	/// aka left-unique
	fn is_injective(&self) -> bool {
		/*
		self.iys().all(
			|iy| self.ixs()
				.filter(|ix| self.eval_at(ix, iy))
				.take(2)
				.len() <= 1
		)
		*/
		for iy in self.iys() {
			let mut found_one = false;
			for ix in self.ixs() {
				if self.eval_at(ix, iy) {
					if !found_one {
						found_one = true;
					} else {
						return false;
					}
				}
			}
		}
		return true;
	}
	/// Return `true` if the relation is functional.
	/// A relation is functional iff `∀x ∈ X and y,z ∈ Y: xRy ∧ xRz ⇒ y = z`
	///
	/// aka univalent, right-unique, right-definite
	fn is_functional(&self) -> bool {
		/*
		self.ixs().all(
			|ix| self.iys()
				.filter(|iy| self.eval_at(ix, iy))
				.take(2)
				.len() <= 1
		)
		*/
		for ix in self.ixs() {
			let mut found_one = false;
			for iy in self.iys() {
				if self.eval_at(ix, iy) {
					if !found_one {
						found_one = true;
					} else {
						return false;
					}
				}
			}
		}
		return true;
	}
	/// Return `true` if the relation is left-total.
	/// A relation is left-total iff `∀x ∈ X: ∃y ∈ Y: xRy`
	fn is_lefttotal(&self) -> bool {
		self.ixs().all(                                 // ∀x:
			|ix| self.iys().any(                    // ∃y:
				|iy| self.eval_at(ix, iy)       // (x,y) ∈ self
			)
		)
	}
	/// Return `true` if the relation is surjective.
	/// A relation is surjective iff `∀y ∈ Y: ∃x ∈ X: xRy`
	///
	/// aka onto, right-total
	fn is_surjective(&self) -> bool {
		self.iys().all(                                 // ∀y:
			|iy| self.ixs().any(                    // ∃x:
				|ix| self.eval_at(ix, iy)       // (x,y) ∈ self
			)
		)
	}
	/// Return `true` if the relation is bijective.
	/// A relation is bijective iff it is injective and surjective
	fn is_bijective(&self) -> bool { self.is_injective() && self.is_surjective() }
	/// Return `true` if the relation is a function.
	/// A relation is a function iff it is functional and left-total
	fn is_function(&self) -> bool { self.is_functional() && self.is_lefttotal() }

	/// Return the source of the relation.
	///
	/// Given a relation `R` over the sets `X, Y`,
	/// the source of `R` is defined as the set `{ x ∈ X: ∃y: y ∈ Y ∧ xRy }`
	fn source(&self) -> Set {
		self.get_domain().0.iter().enumerate()
			.filter(
				|&(ix, _)| self.iys().any(
					|iy| self.eval_at(ix, iy)
				)
			)
			.map(|(_, x)| x)
			.cloned()
			.collect()
	}
	/// Return the range of the relation.
	///
	/// Given a relation `R` over the sets `X, Y`,
	/// the range of `R` is defined as the set `{ y ∈ Y: ∃x: x ∈ X ∧ xRy }`
	fn range(&self) -> Set {
		self.get_domain().1.iter().enumerate()
			.filter(
				|&(iy, _)| self.ixs().any(
					|ix| self.eval_at(ix, iy)
				)
			)
			.map(|(_, y)| y)
			.cloned()
			.collect()
	}
	/// Return the image of the given set under the relation.
	///
	/// Given a relation `R` over the sets `X, Y`,
	/// the image of the set `U \subseteq X` under `R` is the set `{ y ∈ Y: ∃x: x ∈ U ∧ xRy }`
	fn image(&self, set: &Set) -> Set {
		debug_assert!(set.is_subset(self.get_domain().0));
		let u: Vec<((usize, usize), &SetElement)> = Set::intersection_enumerated(
				self.get_domain().0,
				set,
			).collect();
		self.get_domain().1.iter().enumerate()
			.filter(
				|&(iy, _)| u.iter().any(|&((ix, _), _)| self.eval_at(ix, iy))
			)
			.map(|(_, y)| y)
			.cloned()
			.collect()
	}
	/// Return the pre-image of the given set under the relation.
	///
	/// Given a relation `R` over the sets `X, Y`,
	/// the pre-image of the set `V \subseteq Y` under `R` is the set `{ x ∈ X: ∃y: y ∈ V ∧ xRy }`
	fn preimage(&self, set: &Set) -> Set {
		debug_assert!(set.is_subset(self.get_domain().0));
		let v: Vec<((usize, usize), &SetElement)> = Set::intersection_enumerated(
				self.get_domain().1,
				set,
			).collect();
		self.get_domain().0.iter().enumerate()
			.filter(
				|&(ix, _)| v.iter().any(|&((iy, _), _)| self.eval_at(ix, iy))
			)
			.map(|(_, x)| x)
			.cloned()
			.collect()
	}

	/// Return the strict image of the given set under the relation.
	///
	/// Given a relation `R` over the sets `X, Y`,
	/// the image of the set `U \subseteq X` under `R` is the set `{ y ∈ Y: ∀x: x ∈ U → xRy }`
	fn image_strict(&self, set: &Set) -> Set {
		debug_assert!(set.is_subset(self.get_domain().0));
		let u: Vec<((usize, usize), &SetElement)> = Set::intersection_enumerated(
				self.get_domain().0,
				set,
			).collect();
		self.get_domain().1.iter().enumerate()
			.filter(
				|&(iy, _)| u.iter().all(|&((ix, _), _)| self.eval_at(ix, iy))
			)
			.map(|(_, y)| y)
			.cloned()
			.collect()
	}

	/// Return the strict pre-image of the given set under the relation.
	///
	/// Given a relation `R` over the sets `X, Y`,
	/// the pre-image of the set `V \subseteq Y` under `R` is the set `{ x ∈ X: ∀y: y ∈ V → xRy }`
	fn preimage_strict(&self, set: &Set) -> Set {
		debug_assert!(set.is_subset(self.get_domain().0));
		let v: Vec<((usize, usize), &SetElement)> = Set::intersection_enumerated(
				self.get_domain().1,
				set,
			).collect();
		self.get_domain().0.iter().enumerate()
			.filter(
				|&(ix, _)| v.iter().all(|&((iy, _), _)| self.eval_at(ix, iy))
			)
			.map(|(_, x)| x)
			.cloned()
			.collect()
	}

	/// The empty `Relation E` where `xEy` does not hold for any `(x,y) ∈ X × Y`
	fn empty<'a>(domain: (&'a Set, &'a Set)) -> Empty<'a> {
		Empty(domain)
	}
	/// The universal `Relation U` where `xUy` holds for all `(x,y) ∈ X × Y`
	fn universal<'a>(domain: (&'a Set, &'a Set)) -> Universal<'a> {
		Universal(domain)
	}

	/// The complement of a relation.
	///
	/// `xSy ⇔ not xRy`
	fn complement<R>(r: &R) -> Complement<'_, R>
	where R: Relation,
	{
		Complement::new(r)
	}
	/// The concatenation of two relations.
	///
	/// `S ; R`, also denoted `S ∘ R` (or `R ∘ S`), defined as `S ; R = { (x, z) | ∃y ∈ Y: (x, y) ∈ R ∧ (y, z) ∈ S }`
	fn concatenation<'a, P, Q>(p: &'a P, q: &'a Q) -> Concatenation<'a, P, Q>
	where P: Relation,
	      Q: Relation,
	{
		Concatenation::new(p, q)
	}
	/// The converse of a relation.
	///
	/// `R^T = { (y, x) | (x, y) ∈ R }`
	fn converse<R>(r: &R) -> Converse<'_, R>
	where R: Relation,
	{
		Converse::new(r)
	}
	/// The intersection of two relations.
	///
	/// `R ∩ S ⊆ X × Y`, defined as `R ∩ S = { (x, y) | (x, y) ∈ R ∧ (x, y) ∈ S }`
	fn intersection<'a, P, Q>(p: &'a P, q: &'a Q) -> Intersection<'a, P, Q>
	where P: Relation,
	      Q: Relation,
	{
		Intersection::new(p, q)
	}
	/// The union of two relations.
	///
	/// `R ∪ S ⊆ X × Y`, defined as `R ∪ S = { (x, y) | (x, y) ∈ R ∨ (x, y) ∈ S }`
	fn union<'a, P, Q>(p: &'a P, q: &'a Q) -> Union<'a, P, Q>
	where P: Relation,
	      Q: Relation,
	{
		Union::new(p, q)
	}
}

/// Compares two relations for equality.
pub fn eq<P: Relation, Q: Relation>(p: &P, q: &Q) -> bool {
	if p.get_domain() != q.get_domain() { return false; }
	for ix in p.ixs() {
		for iy in p.iys() {
			if p.eval_at(ix, iy) != q.eval_at(ix, iy) { return false; }
		}
	}
	return true;
}

/// The [`Empty`] `Relation`
#[derive(Clone, Debug)]
pub struct Empty<'a>((&'a Set, &'a Set));

impl Relation for Empty<'_> {
	fn get_domain(&self) -> (&Set, &Set) {
		self.0
	}
	fn eval_at(&self, _ix: usize, _iy: usize) -> bool {
		false
	}
}

/// The [`Universal`] `Relation`
#[derive(Clone, Debug)]
pub struct Universal<'a>((&'a Set, &'a Set));

impl Relation for Universal<'_> {
	fn get_domain(&self) -> (&Set, &Set) {
		self.0
	}
	fn eval_at(&self, _ix: usize, _iy: usize) -> bool {
		true
	}
}


#[derive(Clone, Debug)]
pub struct Complement<'a, R: Relation> {
	r: LCow<'a, R>,
}

impl<'a, R: Relation> Complement<'a, R> {
	fn new<T: Into<LCow<'a, R>>>(t: T) -> Self {
		Complement { r: t.into() }
	}
}

impl<R: Relation> Relation for Complement<'_, R> {
	fn get_domain(&self) -> (&Set, &Set) {
		self.r.get_domain()
	}
	fn eval_at(&self, ix: usize, iy: usize) -> bool {
		!self.r.eval_at(ix, iy)
	}
}

#[derive(Clone, Debug)]
pub struct Concatenation<'a, P: Relation, Q: Relation> {
	p: LCow<'a, P>,
	q: LCow<'a, Q>,
}

impl<'a, P: Relation, Q: Relation> Concatenation<'a, P, Q> {
	pub fn new<S: Into<LCow<'a, P>>, T: Into<LCow<'a, Q>>>(s: S, t: T) -> Self {
		let p = s.into();
		let q = t.into();
		debug_assert_eq!(p.get_domain().1, q.get_domain().0);
		Concatenation { p, q }
	}
}

impl<P: Relation, Q: Relation> Relation for Concatenation<'_, P, Q> {
	fn get_domain(&self) -> (&Set, &Set) {
		(self.p.get_domain().0, self.q.get_domain().1)
	}
	fn eval_at(&self, ix: usize, iz: usize) -> bool {
		// p.iys() == q.ixs()
		self.p.iys().any(|iy| self.p.eval_at(ix, iy) && self.q.eval_at(iy, iz))
	}
}

#[derive(Clone, Debug)]
pub struct Converse<'a, R: Relation> {
	r: LCow<'a, R>,
}

impl<'a, R: Relation> Converse<'a, R> {
	fn new<T: Into<LCow<'a, R>>>(t: T) -> Self {
		Converse { r: t.into() }
	}
}

impl<R: Relation> Relation for Converse<'_, R> {
	fn get_domain(&self) -> (&Set, &Set) {
		(self.r.get_domain().1, self.r.get_domain().0)
	}
	fn eval_at(&self, ix: usize, iy: usize) -> bool {
		self.r.eval_at(iy, ix)
	}
}

#[derive(Clone, Debug)]
pub struct Intersection<'a, P: Relation, Q: Relation> {
	p: LCow<'a, P>,
	q: LCow<'a, Q>,
}

impl<'a, P: Relation, Q: Relation> Intersection<'a, P, Q> {
	pub fn new<S: Into<LCow<'a, P>>, T: Into<LCow<'a, Q>>>(s: S, t: T) -> Self {
		Intersection { p: s.into(), q: t.into() }
	}
}

impl<P: Relation, Q: Relation> Relation for Intersection<'_, P, Q> {
	fn get_domain(&self) -> (&Set, &Set) {
		self.p.get_domain()
	}
	fn eval_at(&self, ix: usize, iy: usize) -> bool {
		self.p.eval_at(ix, iy) && self.q.eval_at(ix, iy)
	}
}

#[derive(Clone, Debug)]
pub struct Union<'a, P: Relation, Q: Relation> {
	p: LCow<'a, P>,
	q: LCow<'a, Q>,
}

impl<'a, P: Relation, Q: Relation> Union<'a, P, Q> {
	pub fn new<S: Into<LCow<'a, P>>, T: Into<LCow<'a, Q>>>(s: S, t: T) -> Self {
		Union { p: s.into(), q: t.into() }
	}
}

impl<P: Relation, Q: Relation> Relation for Union<'_, P, Q> {
	fn get_domain(&self) -> (&Set, &Set) {
		self.p.get_domain()
	}
	fn eval_at(&self, ix: usize, iy: usize) -> bool {
		self.p.eval_at(ix, iy) || self.q.eval_at(ix, iy)
	}
}

#[cfg(test)]
pub mod tests {
	use super::*;

	pub fn relation_property_test<R>(r: &R)
	where R: Relation + std::fmt::Debug
	{
		assert_eq!(r.is_homogeneous(), !r.is_heterogeneous());

		assert_eq!(r.is_bijective(), r.is_injective() && r.is_surjective());
		assert_eq!(r.is_function(), r.is_functional() && r.is_lefttotal());
	}

	pub fn complement<R>(r: &R)
	where R: Relation + std::fmt::Debug
	{
		// involutivity
		assert!(eq(&R::complement(&R::complement(r)), r));
	}

	pub fn concatenation<R, S, T>(a: &R, b: &S, c: &T)
	where R: Relation + std::fmt::Debug,
	      S: Relation + std::fmt::Debug,
	      T: Relation + std::fmt::Debug,
	{
		debug_assert_eq!(a.get_domain().1, b.get_domain().0);
		debug_assert_eq!(b.get_domain().1, c.get_domain().0);

		let r = a;

//		let neutral_left  = &R::identity(r.get_domain().0);
//		let neutral_right = &R::identity(r.get_domain().1);
		let absorbing = &R::empty(r.get_domain());

		// concatenation: neutral element
//		assert!(eq(&R::concatenation(neutral_left, r), r));
//		assert!(eq(&R::concatenation(r, neutral_right), r));
		// concatenation: absorbing element
		assert!(eq(&R::concatenation(r, absorbing), absorbing));
		// concatenation: associativity
		assert!(eq(
			&R::concatenation(a, &R::concatenation(b, c)),
			&R::concatenation(&R::concatenation(a, b), c),
		));
		// concatenation: no commutativity
	}

	pub fn distributivity_concatenation<R, S, T>(a: &R, b: &S, c: &T)
	where R: Relation + std::fmt::Debug,
	      S: Relation + std::fmt::Debug,
	      T: Relation + std::fmt::Debug,
	{
		// left distributivity of concatenation over union
		assert!(eq(
			&R::concatenation(a, &R::union(b, c)),
			&R::union(&R::concatenation(a, b), &R::concatenation(a, c)),
		));
		// right distributivity of concatenation over union
		assert!(eq(
			&R::concatenation(&R::union(a, b), c),
			&R::union(&R::concatenation(a, c), &R::concatenation(b, c)),
		));
	}

	pub fn converse<R>(r: &R)
	where R: Relation + std::fmt::Debug
	{
		// involutivity
		assert!(eq(&R::converse(&R::converse(r)), r));
	}

	pub fn distributivity_converse<R, S>(a: &R, b: &S)
	where R: Relation + std::fmt::Debug,
	      S: Relation + std::fmt::Debug,
	{
		// antidistributivity of converse over concatenation
		assert!(eq(
			&R::converse(&R::concatenation(a, b)),
			&R::concatenation(&R::converse(b), &R::converse(a)),
		));
		// distributivity of converse over union
		assert!(eq(
			&R::converse(&R::union(a, b)),
			&R::union(&R::converse(a), &R::converse(b)),
		));
		// distributivity of converse over intersection
		assert!(eq(
			&R::converse(&R::intersection(a, b)),
			&R::intersection(&R::converse(a), &R::converse(b)),
		));
	}

	pub fn union<R, S, T>(a: &R, b: &S, c: &T)
	where R: Relation + std::fmt::Debug,
	      S: Relation + std::fmt::Debug,
	      T: Relation + std::fmt::Debug,
	{
		let r = a;
		assert!(r.is_homogeneous());

		let neutral = &R::empty(r.get_domain());
		let absorbing = &R::universal(r.get_domain());

		// union: neutral element
		assert!(eq(&R::union(r, neutral), r));
		// union: absorbing element
		assert!(eq(&R::union(r, absorbing), absorbing));
		// union: idempotence
		assert!(eq(&R::union(r, r), r));
		// union: associativity
		assert!(eq(
			&R::union(a, &R::union(b, c)),
			&R::union(&R::union(a, b), c),
		));
		// union: commutativity
		assert!(eq(&R::union(a, b), &R::union(b, a)));
	}

	pub fn intersection<R, S, T>(a: &R, b: &S, c: &T)
	where R: Relation + std::fmt::Debug,
	      S: Relation + std::fmt::Debug,
	      T: Relation + std::fmt::Debug,
	{
		let r = a;
		assert!(r.is_homogeneous());

		let neutral = &R::universal(r.get_domain());
		let absorbing = &R::empty(r.get_domain());

		// intersection: neutral element
		assert!(eq(&R::intersection(r, neutral), r));
		// intersection: absorbing element
		assert!(eq(&R::intersection(r, absorbing), absorbing));
		// intersection: idempotence
		assert!(eq(&R::intersection(r, r), r));
		// intersection: associativity
		assert!(eq(
			&R::intersection(a, &R::intersection(b, c)),
			&R::intersection(&R::intersection(a, b), c)
		));
		// intersection: commutativity
		assert!(eq(&R::intersection(a, b), &R::intersection(b, a)));
	}

	pub fn distributivity_union<R, S, T>(a: &R, b: &S, c: &T)
	where R: Relation + std::fmt::Debug,
	      S: Relation + std::fmt::Debug,
	      T: Relation + std::fmt::Debug,
	{
		// left distributivity of union over intersection
		assert!(eq(
			&R::union(a, &R::intersection(b, c)),
			&R::intersection(&R::union(a, b), &R::union(a, c)),
		));
		// right distributivity of union over intersection
		assert!(eq(
			&R::union(&R::intersection(a, b), c),
			&R::intersection(&R::union(a, c), &R::union(b, c)),
		));
	}

	pub fn distributivity_intersection<R, S, T>(a: &R, b: &S, c: &T)
	where R: Relation + std::fmt::Debug,
	      S: Relation + std::fmt::Debug,
	      T: Relation + std::fmt::Debug,
	{
		// left distributivity of intersection over union
		assert!(eq(
			&R::intersection(a, &R::union(b, c)),
			&R::union(&R::intersection(a, b), &R::intersection(a, c)),
		));
		// right distributivity of intersection over union
		assert!(eq(
			&R::intersection(&R::union(a, b), c),
			&R::union(&R::intersection(a, c), &R::intersection(b, c)),
		));
	}

	pub fn de_morgan<R>(a: &R, b: &R)
	where R: Relation + std::fmt::Debug
	{
		assert!(eq(
			&R::complement(&R::union(a, b)),
			&R::intersection(&R::complement(a), &R::complement(b)),
		));
		assert!(eq(
			&R::complement(&R::intersection(a, b)),
			&R::union(&R::complement(a), &R::complement(b)),
		));
	}
}
