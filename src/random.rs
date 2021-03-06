use rand::distributions::{Bernoulli, Distribution};

use crate::set::Set;
use crate::relation::RelationVec;

/// Generate a random [`Relation`] for the given domain.
/// `p` specifies the density of the created incidence matrix.
///
/// # Examples
///
/// Create a `Relation` with 50% density.
///
/// ```
/// let s: relax::Set = (1..3).collect();
/// let r: relax::RelationVec = relax::random::generate_random((s.clone(), s.clone()), 0.5);
/// ```
///
/// `p = 0` creates the empty `Relation`.
///
/// ```
/// use relax::Relation;
/// let s: relax::Set = (1..3).collect();
/// let r: relax::RelationVec = relax::random::generate_random((s.clone(), s.clone()), 0.0);
/// assert!(relax::relation::eq(&r, &relax::RelationVec::empty((&s, &s))));
/// ```
///
/// `p = 1` creates the universal `Relation`.
///
/// ```
/// use relax::Relation;
/// let s: relax::Set = (1..3).collect();
/// let r: relax::RelationVec = relax::random::generate_random((s.clone(), s.clone()), 1.0);
/// assert!(relax::relation::eq(&r, &relax::RelationVec::universal((&s, &s))));
/// ```
pub fn generate_random(domain: (Set, Set), p: f64) -> RelationVec {
	let d = Bernoulli::new(p);
	let mut rng = rand::thread_rng();
	let v: Vec<bool> = d.sample_iter(&mut rng)
		.take(domain.0.cardinality() * domain.1.cardinality())
		.collect();
	RelationVec::new(domain, v)
}
