extern crate rand;

use random::rand::distributions::{Bernoulli, Distribution};

use Set;
use relation::RelationVec;

pub fn generate_random(domain: (Set, Set), p: f64) -> RelationVec {
	let d = Bernoulli::new(p);
	let mut rng = rand::thread_rng();
	let v: Vec<bool> = d.sample_iter(&mut rng)
		.take(domain.0.cardinality() * domain.1.cardinality())
		.collect();
	RelationVec::new(domain, v)
}
