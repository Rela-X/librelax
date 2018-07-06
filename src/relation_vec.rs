use std;
use Relation;
use relation_tabular::RelationTabular;

#[derive(Debug,PartialEq,Eq)]
pub struct RelationVec<'a, P: 'a, Q: 'a> {
	pub domain: (&'a[P], &'a[Q]),
	pub table: Vec<bool>,
}

macro_rules! cross {
	($p:expr, $q:expr) => ($p.flat_map(|e| std::iter::repeat(e).zip($q.clone())))
}
/*
macro_rules! cross_uniq {
	($p:expr, $q:expr) => ($p.enumerate().flat_map(|(i, e)| std::iter::repeat(e).zip($q.skip(i+1).clone())))
}
*/

/* Endorelation */
impl<'a, T> RelationVec<'a, T, T>
where T: Eq
{
	pub fn new_empty(set: &'a[T]) -> Self {
		return RelationVec {
			domain: (set, set),
			table: vec![false; set.len().pow(2)],
		};
	}
	pub fn new_full(set: &'a[T]) -> Self {
		return RelationVec {
			domain: (set, set),
			table: vec![true; set.len().pow(2)],
		};
	}
	pub fn new_id(set: &'a[T]) -> Self {
		let mut r = Self::new_empty(set);
		for i in 0..set.len() {
			let idx = r.get_table_index(i, i);
			r.table[idx] = true;
		}
		return r;
	}
	/* inclusive */
	pub fn new_top(set: &'a[T]) -> Self {
		let mut r = Self::new_full(set);
		for i0 in 0..set.len() {
			for i1 in 0..i0 {
				let idx = r.get_table_index(i0, i1);
				r.table[idx] = false;
			}
		}
		return r;
	}
	/* inclusive */
	pub fn new_bottom(set: &'a[T]) -> Self {
		let mut r = Self::new_full(set);
		for i0 in 0..set.len() {
			for i1 in (i0+1)..set.len() {
				let idx = r.get_table_index(i0, i1);
				r.table[idx] = false;
			}
		}
		return r;
	}

	pub fn from_predicate<P>(set: &'a[T], predicate: P) -> Self
	where P: FnMut((&T, &T)) -> bool {
		return RelationVec {
			domain: (set, set),
			table: (0..set.len().pow(2))
				.map(|i| (i / set.len(), i % set.len())) // ? cross!()
				.map(|(ix, iy)| (&set[ix], &set[iy]))
				.map(predicate)
				.collect::<Vec<_>>(),
		};
	}

	pub fn new_union(p: Self, q: Self) -> Self {
		// TODO if p.domain != q.domain { error }
		return RelationVec {
			domain: p.domain,
			table: p.table.iter().zip(q.table.iter()).map(|(&bq, &bp)| bq || bp).collect::<Vec<_>>(),
		};
	}
	pub fn new_intersection(p: Self, q: Self) -> Self {
		// TODO if p.domain != q.domain { error }
		return RelationVec {
			domain: p.domain,
			table: p.table.iter().zip(q.table.iter()).map(|(&bq, &bp)| bq && bp).collect::<Vec<_>>(),
		};
	}
	pub fn new_complement(r: Self) -> Self {
		return RelationVec {
			domain: r.domain,
			table: r.table.iter().map(|&b| !b).collect::<Vec<_>>(),
		};
	}
	pub fn new_converse(r: Self) -> Self {
		// TODO if !r.is_homogeneous { error }
		let mut new = Self::new_empty(r.domain.0);
		// transpose r.table
		for (ix, iy) in cross!(0..r.domain.0.len(), 0..r.domain.1.len()) {
			new.table[r.get_table_index(ix, iy)] = r.eval_at(iy, ix);
		}
		return new;

	}
	// TODO ? "composition"
	pub fn new_concatenation(p: Self, q: Self) -> Self {
		// TODO if p.domain != q.domain { error }
		let mut r = Self::new_empty(p.domain.0);
		// FIXME validate this!
		for (ix, iy) in cross!(0..p.domain.0.len(), 0..p.domain.1.len()) {
			if !p.eval_at(ix, iy) { continue; }
			for iz in 0..q.domain.1.len() {
				if !q.eval_at(iy, iz) { continue; }
				let idx = r.get_table_index(ix, iz);
				r.table[idx] = true;
			}
		}
		return r;
	}
//	fn new_subsetleq(rf_Set *domain) -> RelationVec<'a, P, Q> {}
}

impl<'a, P, Q> RelationVec<'a, P, Q>
where P: Eq + PartialEq<Q>,
      Q: Eq + PartialEq<P>,
{
	fn get_table_index(&self, ix: usize, iy: usize) -> usize {
		ix * self.domain.0.len() + iy
	}
}

impl<'a, P, Q> RelationTabular for RelationVec<'a, P, Q>
where P: Eq + PartialEq<Q>,
      Q: Eq + PartialEq<P>,
{
	type X = P;
	type Y = Q;

	fn get_domain(&self) -> (&[P], &[Q]) { self.domain }

	fn eval_at(&self, ix: usize, iy: usize) -> bool {
		let i = self.get_table_index(ix, iy);
		return self.table[i];
	}
}

/*
use std::cmp::Ordering;
fn foobar<P, Q>(r: &RelationVec<P, Q>, p: &P, q: &Q) -> Ordering
where P: Eq + PartialEq<Q>,
      Q: Eq + PartialEq<P>,
{
	if(p == q) {
		return Ordering::Equal;
	} else if(r.eval(p, q)) {
		return Ordering::Less;
	} else {
		return Ordering::Greater;
	}
}
fn foobar2<P, Q>(r: &RelationVec<P, Q>) -> impl Fn(&P, &Q) -> Ordering
where P: Eq + PartialEq<Q>,
      Q: Eq + PartialEq<P>,
{
	|p, q|
	if(p == q) {
		Ordering::Equal
	} else if(r.eval(p, q)) {
		Ordering::Less
	} else {
		Ordering::Greater
	}
}
*/

#[cfg(test)]
mod tests {
	use super::*;

	const ALPHABET: [char; 26] = [
		'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
		'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
		'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
		'y', 'z',
	];

	#[test]
	fn eval() {
		let r1 = RelationVec {
			domain: (&ALPHABET[0..3], &ALPHABET[0..3]),
			table: vec![
				true, false, false,
				false, true, false,
				false, false, true
			],
		};
		assert!( r1.eval(&'a', &'a')); assert!(!r1.eval(&'a', &'b')); assert!(!r1.eval(&'a', &'c'));
		assert!(!r1.eval(&'b', &'a')); assert!( r1.eval(&'b', &'b')); assert!(!r1.eval(&'b', &'c'));
		assert!(!r1.eval(&'c', &'a')); assert!(!r1.eval(&'c', &'b')); assert!( r1.eval(&'c', &'c'));
	}
	#[test]
	fn homogeneous() {
		let s1 = &ALPHABET[0..3];
		let s2 = ['a', 'b', 'c'];
		let r1 = RelationVec {
			domain: (&s1, &s2),
			table: vec![],
		};
		assert!(r1.is_homogeneous());

		let s3 = vec!['x', 'y', 'z'];
		let r2 = RelationVec {
			domain: (&s1, &s3),
			table: vec![],
		};
		assert!(!r2.is_homogeneous());
	}
	#[test]
	fn new_empty() {
		let domain = &ALPHABET[0..3];
		let r1 = RelationVec::new_empty(domain);
		assert!(r1.table.iter().all(|&b| !b));
	}
	#[test]
	fn new_full() {
		let domain = &ALPHABET[0..3];
		let r1 = RelationVec::new_full(domain);
		assert!(r1.table.iter().all(|&b| b));
	}
	#[test]
	fn new_id() {
		let domain = &ALPHABET[0..3];
		let r1 = RelationVec::new_id(domain);
		for (i0, e0) in r1.domain.0.iter().enumerate() {
			for (i1, e1) in r1.domain.1.iter().enumerate() {
				assert_eq!(r1.eval(e0, e1), i0 == i1);
			}
		}
	}
	#[test]
	fn new_top() {
		let domain = &ALPHABET[0..3];
		let r1 = RelationVec::new_top(domain);
		for (i0, e0) in r1.domain.0.iter().enumerate() {
			for (i1, e1) in r1.domain.1.iter().enumerate() {
				assert_eq!(r1.eval(e0, e1), i0 <= i1);
			}
		}
	}
	#[test]
	fn new_bottom() {
		let domain = &ALPHABET[0..3];
		let r1 = RelationVec::new_bottom(domain);
		for (i0, e0) in r1.domain.0.iter().enumerate() {
			for (i1, e1) in r1.domain.1.iter().enumerate() {
				assert_eq!(r1.eval(e0, e1), i0 >= i1);
			}
		}
	}
}
