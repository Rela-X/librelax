use std;
use cow::LCow;

use set::{Set, SetElement};

pub trait Relation : Clone {
	fn get_domain(&self) -> (&Set, &Set);
	fn eval(&self, x: &SetElement, y: &SetElement) -> bool {
		// TODO O(log n) should be possible here
		let ix = self.get_domain().0.iter().position(|e| e == x).unwrap();
		let iy = self.get_domain().1.iter().position(|e| e == y).unwrap();
		return self.eval_at(ix, iy);
	}
	fn eval_at(&self, ix: usize, iy: usize) -> bool;

	fn ixs(&self) -> std::ops::Range<usize> { 0..self.get_domain().0.cardinality() }
	fn iys(&self) -> std::ops::Range<usize> { 0..self.get_domain().1.cardinality() }

	fn is_homogeneous(&self) -> bool { self.get_domain().0 == self.get_domain().1 }
	fn is_heterogeneous(&self) -> bool { !self.is_homogeneous() }

	fn is_injective(&self) -> bool { /* aka left-unique */
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
	fn is_functional(&self) -> bool { /* aka univalent, right-unique, right-definite */
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
	fn is_lefttotal(&self) -> bool {
		self.ixs().all(                                 // ∀x:
			|ix| self.iys().any(                    // ∃y:
				|iy| self.eval_at(ix, iy)       // (x,y) ∈ self
			)
		)
	}
	fn is_surjective(&self) -> bool { /* aka onto, right-total */
		self.iys().all(                                 // ∀y:
			|iy| self.ixs().any(                    // ∃x:
				|ix| self.eval_at(ix, iy)       // (x,y) ∈ self
			)
		)
	}
	fn is_bijective(&self) -> bool { self.is_injective() && self.is_surjective() }
	fn is_function(&self) -> bool { self.is_functional() && self.is_lefttotal() }

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

	fn complement<R>(r: &R) -> Complement<R>
	where R: Relation,
	{
		Complement::new(r)
	}
	fn concatenation<'a, P, Q>(p: &'a P, q: &'a Q) -> Concatenation<'a, P, Q>
	where P: Relation,
	      Q: Relation,
	{
		Concatenation::new(p, q)
	}
	fn converse<R>(r: &R) -> Converse<R>
	where R: Relation,
	{
		Converse::new(r)
	}
	fn intersection<'a, P, Q>(p: &'a P, q: &'a Q) -> Intersection<'a, P, Q>
	where P: Relation,
	      Q: Relation,
	{
		Intersection::new(p, q)
	}
	fn union<'a, P, Q>(p: &'a P, q: &'a Q) -> Union<'a, P, Q>
	where P: Relation,
	      Q: Relation,
	{
		Union::new(p, q)
	}
}

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
