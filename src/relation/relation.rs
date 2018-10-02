use std;

use set::{Set, SetElement};

pub trait Relation {
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

	fn complement<R>(r: &R) -> Complement<R>
	where R: Relation,
	{
		Complement { r: r }
	}
	fn concatenation<'a, P, Q>(p: &'a P, q: &'a Q) -> Concatenation<'a, P, Q>
	where P: Relation,
	      Q: Relation,
	{
		Concatenation { p: p, q: q }
	}
	fn converse<R>(r: &R) -> Converse<R>
	where R: Relation,
	{
		Converse { r: r }
	}
	fn intersection<'a, P, Q>(p: &'a P, q: &'a Q) -> Intersection<'a, P, Q>
	where P: Relation,
	      Q: Relation,
	{
		Intersection { p: p, q: q }
	}
	fn union<'a, P, Q>(p: &'a P, q: &'a Q) -> Union<'a, P, Q>
	where P: Relation,
	      Q: Relation,
	{
		Union { p: p, q: q }
	}
}

fn eq<P: Relation, Q: Relation>(p: &P, q: &Q) -> bool {
	if p.get_domain() != q.get_domain() { return false; }
	for ix in p.ixs() {
		for iy in p.iys() {
			if p.eval_at(ix, iy) != q.eval_at(ix, iy) { return false; }
		}
	}
	return true;
}

#[derive(Debug)]
pub struct Complement<'a, R: 'a + Relation> {
	r: &'a R,
}

impl<'a, R: 'a + Relation> Relation for Complement<'a, R> {
	fn get_domain(&self) -> (&Set, &Set) {
		self.r.get_domain()
	}
	fn eval_at(&self, ix: usize, iy: usize) -> bool {
		!self.r.eval_at(ix, iy)
	}
}

impl<'a, R: Relation, T: Relation> PartialEq<R> for Complement<'a, T> {
	fn eq(&self, other: &R) -> bool { eq(self, other) }
}

#[derive(Debug)]
pub struct Concatenation<'a, P: 'a + Relation, Q: 'a + Relation> {
	p: &'a P,
	q: &'a Q,
}

impl<'a, P: 'a + Relation, Q: 'a + Relation> Relation for Concatenation<'a, P, Q> {
	fn get_domain(&self) -> (&Set, &Set) {
		self.p.get_domain()
	}
	fn eval_at(&self, ix: usize, iy: usize) -> bool {
		self.p.eval_at(ix, iy) && (self.q.iys()).any(|iz| self.q.eval_at(iy, iz))
	}
}

impl<'a, R: Relation, S: Relation, T: Relation> PartialEq<R> for Concatenation<'a, S, T> {
	fn eq(&self, other: &R) -> bool { eq(self, other) }
}

#[derive(Debug)]
pub struct Converse<'a, R: 'a + Relation> {
	r: &'a R,
}

impl<'a, R: 'a + Relation> Relation for Converse<'a, R> {
	fn get_domain(&self) -> (&Set, &Set) {
		self.r.get_domain()
	}
	fn eval_at(&self, ix: usize, iy: usize) -> bool {
		self.r.eval_at(iy, ix)
	}
}

impl<'a, R: Relation, T: Relation> PartialEq<R> for Converse<'a, T> {
	fn eq(&self, other: &R) -> bool { eq(self, other) }
}

#[derive(Debug)]
pub struct Intersection<'a, P: 'a + Relation, Q: 'a + Relation> {
	p: &'a P,
	q: &'a Q,
}

impl<'a, P: 'a + Relation, Q: 'a + Relation> Relation for Intersection<'a, P, Q> {
	fn get_domain(&self) -> (&Set, &Set) {
		self.p.get_domain()
	}
	fn eval_at(&self, ix: usize, iy: usize) -> bool {
		self.p.eval_at(ix, iy) && self.q.eval_at(ix, iy)
	}
}

impl<'a, R: Relation, S: Relation, T: Relation> PartialEq<R> for Intersection<'a, S, T> {
	fn eq(&self, other: &R) -> bool { eq(self, other) }
}

#[derive(Debug)]
pub struct Union<'a, P: 'a + Relation, Q: 'a + Relation> {
	p: &'a P,
	q: &'a Q,
}

impl<'a, P: 'a + Relation, Q: 'a + Relation> Relation for Union<'a, P, Q> {
	fn get_domain(&self) -> (&Set, &Set) {
		self.p.get_domain()
	}
	fn eval_at(&self, ix: usize, iy: usize) -> bool {
		self.p.eval_at(ix, iy) || self.q.eval_at(ix, iy)
	}
}

impl<'a, R: Relation, S: Relation, T: Relation> PartialEq<R> for Union<'a, S, T> {
	fn eq(&self, other: &R) -> bool { eq(self, other) }
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
