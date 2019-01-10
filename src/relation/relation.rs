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
	/// the source is defined as the set `{ x ∈ X: ∃y ∈ Y: xRy }`
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
	/// the range is defined as the set `{ y ∈ Y: ∃x ∈ X: xRy }`
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
	/// Return the image of the relation.
	///
	/// Given a relation `R` over the sets `X, Y`,
	/// the image is defined as the set `{ x ∈ X: ∀y ∈ Y: xRy }`
	fn image(&self) -> Set {
		self.get_domain().0.iter().enumerate()
			.filter(
				|&(ix, _)| self.iys().all(
					|iy| self.eval_at(ix, iy)
				)
			)
			.map(|(_, x)| x)
			.cloned()
			.collect()
	}
	/// Return the pre-image of the relation.
	///
	/// Given a relation `R` over the sets `X, Y`,
	/// the pre-image is defined as the set `{ y ∈ Y: ∀x ∈ X: xRy }`
	fn preimage(&self) -> Set {
		self.get_domain().1.iter().enumerate()
			.filter(
				|&(iy, _)| self.ixs().all(
					|ix| self.eval_at(ix, iy)
				)
			)
			.map(|(_, y)| y)
			.cloned()
			.collect()
	}

	/// The complement of a relation.
	///
	/// `xSy ⇔ not xRy`
	fn complement<R>(r: &R) -> Complement<R>
	where R: Relation,
	{
		Complement::new(r)
	}
	/// The concatenation of two relations.
	///
	/// `S ; R`, also denoted `R ∘ S` (or `R ∘ S`), defined as `S ; R = { (x, z) | ∃y ∈ Y: (x, y) ∈ R ∧ (y, z) ∈ S }`
	fn concatenation<'a, P, Q>(p: &'a P, q: &'a Q) -> Concatenation<'a, P, Q>
	where P: Relation,
	      Q: Relation,
	{
		Concatenation::new(p, q)
	}
	/// The converse of a relation.
	///
	/// `R^T = { (y, x) | (x, y) ∈ R }`
	fn converse<R>(r: &R) -> Converse<R>
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

#[derive(Clone, Debug)]
pub struct Complement<'a, R: 'a + Relation> {
	r: LCow<'a, R>,
}

impl<'a, R: 'a + Relation> Complement<'a, R> {
	fn new<T: Into<LCow<'a, R>>>(t: T) -> Self {
		Complement { r: t.into() }
	}
}

impl<'a, R: 'a + Relation> Relation for Complement<'a, R> {
	fn get_domain(&self) -> (&Set, &Set) {
		self.r.get_domain()
	}
	fn eval_at(&self, ix: usize, iy: usize) -> bool {
		!self.r.eval_at(ix, iy)
	}
}

#[derive(Clone, Debug)]
pub struct Concatenation<'a, P: 'a + Relation, Q: 'a + Relation> {
	p: LCow<'a, P>,
	q: LCow<'a, Q>,
}

impl<'a, P: 'a + Relation, Q: 'a + Relation> Concatenation<'a, P, Q> {
	pub fn new<S: Into<LCow<'a, P>>, T: Into<LCow<'a, Q>>>(s: S, t: T) -> Self {
		Concatenation { p: s.into(), q: t.into() }
	}
}

impl<'a, P: 'a + Relation, Q: 'a + Relation> Relation for Concatenation<'a, P, Q> {
	fn get_domain(&self) -> (&Set, &Set) {
		self.p.get_domain()
	}
	fn eval_at(&self, ix: usize, iy: usize) -> bool {
		self.p.eval_at(ix, iy) && (self.q.iys()).any(|iz| self.q.eval_at(iy, iz))
	}
}

#[derive(Clone, Debug)]
pub struct Converse<'a, R: 'a + Relation> {
	r: LCow<'a, R>,
}

impl<'a, R: 'a + Relation> Converse<'a, R> {
	fn new<T: Into<LCow<'a, R>>>(t: T) -> Self {
		Converse { r: t.into() }
	}
}

impl<'a, R: 'a + Relation> Relation for Converse<'a, R> {
	fn get_domain(&self) -> (&Set, &Set) {
		self.r.get_domain()
	}
	fn eval_at(&self, ix: usize, iy: usize) -> bool {
		self.r.eval_at(iy, ix)
	}
}

#[derive(Clone, Debug)]
pub struct Intersection<'a, P: 'a + Relation, Q: 'a + Relation> {
	p: LCow<'a, P>,
	q: LCow<'a, Q>,
}

impl<'a, P: 'a + Relation, Q: 'a + Relation> Intersection<'a, P, Q> {
	pub fn new<S: Into<LCow<'a, P>>, T: Into<LCow<'a, Q>>>(s: S, t: T) -> Self {
		Intersection { p: s.into(), q: t.into() }
	}
}

impl<'a, P: 'a + Relation, Q: 'a + Relation> Relation for Intersection<'a, P, Q> {
	fn get_domain(&self) -> (&Set, &Set) {
		self.p.get_domain()
	}
	fn eval_at(&self, ix: usize, iy: usize) -> bool {
		self.p.eval_at(ix, iy) && self.q.eval_at(ix, iy)
	}
}

#[derive(Clone, Debug)]
pub struct Union<'a, P: 'a + Relation, Q: 'a + Relation> {
	p: LCow<'a, P>,
	q: LCow<'a, Q>,
}

impl<'a, P: 'a + Relation, Q: 'a + Relation> Union<'a, P, Q> {
	pub fn new<S: Into<LCow<'a, P>>, T: Into<LCow<'a, Q>>>(s: S, t: T) -> Self {
		Union { p: s.into(), q: t.into() }
	}
}

impl<'a, P: 'a + Relation, Q: 'a + Relation> Relation for Union<'a, P, Q> {
	fn get_domain(&self) -> (&Set, &Set) {
		self.p.get_domain()
	}
	fn eval_at(&self, ix: usize, iy: usize) -> bool {
		self.p.eval_at(ix, iy) || self.q.eval_at(ix, iy)
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
