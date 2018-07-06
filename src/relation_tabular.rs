use std;
use Relation;
use Union;
use Intersection;
use Complement;
use Concatenation;
use Converse;

pub trait RelationTabular
where Self::X: Eq + PartialEq<Self::Y>,
      Self::Y: Eq + PartialEq<Self::X>,
{
	type X;
	type Y;

	fn get_domain(&self) -> (&[Self::X], &[Self::Y]);

	fn eval_at(&self, ix: usize, iy: usize) -> bool;
}

macro_rules! cross {
	($p:expr, $q:expr) => ($p.flat_map(|e| std::iter::repeat(e).zip($q.clone())))
}
macro_rules! cross_uniq {
	($p:expr, $q:expr) => ($p.enumerate().flat_map(|(i, e)| std::iter::repeat(e).zip($q.skip(i+1).clone())))
}

impl<T, X, Y> Relation for T
where T: RelationTabular<X=X, Y=Y>,
      X: PartialEq<Y> + Eq,
      Y: PartialEq<X> + Eq,
{
	type X = X;
	type Y = Y;

	fn eval(&self, x: &Self::X, y: &Self::Y) -> bool {
		let ix = self.get_domain().0.iter().position(|e| e == x).unwrap();
		let iy = self.get_domain().1.iter().position(|e| e == y).unwrap();
		return self.eval_at(ix, iy);
	}

	fn is_homogeneous(&self) -> bool { self.get_domain().0 == self.get_domain().1 }

	fn is_reflexive(&self) -> bool {
		self.is_homogeneous() && (0..self.get_domain().0.len()).all(|i| self.eval_at(i, i))
	}
	fn is_irreflexive(&self) -> bool {
		self.is_homogeneous() && (0..self.get_domain().0.len()).all(|i| !self.eval_at(i, i))
	}
	fn is_antisymmetric(&self) -> bool {
		self.is_homogeneous() && cross_uniq!(0..self.get_domain().0.len(), 0..self.get_domain().1.len()).all(
			|(ix, iy)| !self.eval_at(ix, iy) || !self.eval_at(iy, ix)
		)
		/*
		if !self.is_homogeneous() { return false; }
		for i0 in 1..self.get_domain().0.len() {
			for i1 in 0..i0 {
				if self.eval_at(i0, i1) && self.eval_at(i1, i0) {
					return false;
				}
			}
		}
		return true;
		*/
	}
	fn is_transitive(&self) -> bool {
		self.is_homogeneous() && cross!(0..self.get_domain().0.len(), 0..self.get_domain().1.len())
			.filter(|(ix, iy)| ix != iy)
			.filter(|(ix, iy)| self.eval_at(*ix, *iy))
			.all(|(ix, iy)| (0..self.get_domain().0.len())
				.filter(|&iz| self.eval_at(iy, iz))
				.all(|iz| self.eval_at(ix, iz))
			)
		/*
		if !self.is_homogeneous() { return false; }
		for i0 in 0..self.get_domain().0.len() {
			for i1 in 0..self.get_domain().1.len() {
				if i0 == i1 { continue; }
				if !self.eval_at(i0, i1) { continue; } // no xRy
				for i2 in 0..self.get_domain().0.len() {
					if !self.eval_at(i1, i2) { continue; } // no yRz
					if !self.eval_at(i0, i2) { return false; } // no xRz
				}
			}
		}
		return true;
		*/
	}

	fn is_symmetric(&self) -> bool {
		self.is_homogeneous() && cross_uniq!(0..self.get_domain().0.len(), 0..self.get_domain().1.len()).all(
			|(ix, iy)| self.eval_at(ix, iy) == self.eval_at(iy, ix)
		)
	}
	//fn is_asymmetric(&self) -> bool;

	//fn is_preorder(&self) -> bool;
	//fn is_equivalent(&self) -> bool;
	//fn is_partial_order(&self) -> bool;

	fn is_difunctional(&self) -> bool {
		if !self.is_homogeneous() { return false; }
		for (ix, iy) in cross!(0..self.get_domain().0.len(), 0..self.get_domain().1.len()) {
			if !self.eval_at(ix, iy) { continue; }
			for iz in (ix+1)..self.get_domain().0.len() {
				if !self.eval_at(iz, iy) { continue; }
				for iw in 0..self.get_domain().0.len() {
					if self.eval_at(ix, iw) != self.eval_at(iz, iw) {
						return false;
					}
				}
			}
		}
		return true;
	}

/*
	fn is_lattice(&self) -> bool {
		if !self.is_homogeneous() { return false; } // TODO Error
		if !self.is_partial_order() { return false; } // TODO Error
		// TODO
		false
	}
	fn is_sublattice(&self, other: &Relation) -> bool {
		false
	}
*/

	fn is_injective(&self) -> bool {
		for i1 in 0..self.get_domain().1.len() {
			let mut found_one = false;
			for i0 in 0..self.get_domain().0.len() {
				if self.eval_at(i0, i1) {
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
	fn is_functional(&self) -> bool {
		/*
		(0..self.get_domain().0.len()).all(
			|ix| (0..self.get_domain().1.len())
				.filter(|iy| self.eval_at(ix, iy))
				.take(2)
				.len() == 1
		)
		*/
		for i0 in 0..self.get_domain().0.len() {
			let mut found_one = false;
			for i1 in 0..self.get_domain().1.len() {
				if self.eval_at(i0, i1) {
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
		(0..self.get_domain().0.len()).all(                     // ∀x:
			|ix| (0..self.get_domain().1.len()).any(        // ∃y:
				|iy| self.eval_at(ix, iy)               // (x,y) ∈ self
			)
		)
	}
	fn is_surjective(&self) -> bool {
		(0..self.get_domain().1.len()).all(                     // ∀y:
			|iy| (0..self.get_domain().0.len()).any(        // ∃x:
				|ix| self.eval_at(ix, iy)               // (x,y) ∈ self
			)
		)
	}
	//fn is_bijective(&self) -> bool;
	//fn is_function(&self) -> bool;
}

impl<P, Q, X, Y> RelationTabular for Union<P, Q, X, Y>
where P: Relation<X=X, Y=Y>,
      Q: Relation<X=X, Y=Y>,
      X: PartialEq<Y> + Eq,
      Y: PartialEq<X> + Eq,
{
	type X = X;
	type Y = Y;
	fn get_domain(&self) -> (&[Self::X], &[Self::Y]) { self.p.get_domain() }
	fn eval_at(&self, ix: usize, iy: usize) -> bool { self.p.eval_at(ix, iy) || self.q.eval_at(ix, iy) }
}
//impl<T: RelationTabular> PartialEq<T> for Union<T> {
impl<T, P, Q, X, Y> PartialEq<T> for Union<P, Q, X, Y>
where T: RelationTabular,
      P: Relation<X=X, Y=Y>,
      Q: Relation<X=X, Y=Y>,
      X: PartialEq<Y> + Eq,
      Y: PartialEq<X> + Eq,
{
	fn eq(&self, other: &T) -> bool {
		self.get_domain() == other.get_domain() && cross!(0..self.get_domain().0.len(), 0..self.get_domain().1.len()).all(
			|(ix, iy)| self.eval_at(ix, iy) == other.eval_at(ix, iy)
		)
	}
}

impl<T: RelationTabular> RelationTabular for Intersection<T> {
	type X = T::X;
	type Y = T::Y;
	fn get_domain(&self) -> (&[Self::X], &[Self::Y]) { self.p.get_domain() }
	fn eval_at(&self, ix: usize, iy: usize) -> bool { self.p.eval_at(ix, iy) && self.q.eval_at(ix, iy) }
}
impl<T: RelationTabular> PartialEq<T> for Intersection<T> {
	fn eq(&self, other: &T) -> bool {
		self.get_domain() == other.get_domain() && cross!(0..self.get_domain().0.len(), 0..self.get_domain().1.len()).all(
			|(ix, iy)| self.eval_at(ix, iy) == other.eval_at(ix, iy)
		)
	}
}

impl<T: RelationTabular> RelationTabular for Complement<T> {
	type X = T::X;
	type Y = T::Y;
	fn get_domain(&self) -> (&[Self::X], &[Self::Y]) { self.r.get_domain() }
	fn eval_at(&self, ix: usize, iy: usize) -> bool { !self.r.eval_at(ix, iy) }
}
impl<T: RelationTabular> PartialEq<T> for Complement<T> {
	fn eq(&self, other: &T) -> bool {
		self.get_domain() == other.get_domain() && cross!(0..self.get_domain().0.len(), 0..self.get_domain().1.len()).all(
			|(ix, iy)| self.eval_at(ix, iy) == other.eval_at(ix, iy)
		)
	}
}

impl<T: RelationTabular> RelationTabular for Converse<T> {
	type X = T::X;
	type Y = T::Y;
	fn get_domain(&self) -> (&[Self::X], &[Self::Y]) { self.r.get_domain() }
	fn eval_at(&self, ix: usize, iy: usize) -> bool { self.r.eval_at(iy, ix) }
}
impl<T: RelationTabular> PartialEq<T> for Converse<T> {
	fn eq(&self, other: &T) -> bool {
		self.get_domain() == other.get_domain() && cross!(0..self.get_domain().0.len(), 0..self.get_domain().1.len()).all(
			|(ix, iy)| self.eval_at(ix, iy) == other.eval_at(ix, iy)
		)
	}
}

impl<T: RelationTabular> RelationTabular for Concatenation<T> {
	type X = T::X;
	type Y = T::Y;
	fn get_domain(&self) -> (&[Self::X], &[Self::Y]) { self.p.get_domain() }
	fn eval_at(&self, ix: usize, iy: usize) -> bool {
		self.p.eval_at(ix, iy) && (0..self.q.get_domain().1.len()).any(|iz| self.q.eval_at(iy, iz))
	}
}
impl<T: RelationTabular> PartialEq<T> for Concatenation<T> {
	fn eq(&self, other: &T) -> bool {
		self.get_domain() == other.get_domain() && cross!(0..self.get_domain().0.len(), 0..self.get_domain().1.len()).all(
			|(ix, iy)| self.eval_at(ix, iy) == other.eval_at(ix, iy)
		)
	}
}

