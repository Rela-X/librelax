extern crate relax;

use relax::Set;
use relax::relation;
use relax::relation::{Relation, Endorelation, RelationVec};

const ALPHABET: [char; 26] = [
	'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
	'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
	'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
	'y', 'z',
];

#[test]
fn is_reflexive() {
	let set: Set = ALPHABET[0..3].iter().collect();
	let table = vec![
		true,  false, false,
		true,  true,  false,
		false, false, true,
	];
	let r1 = RelationVec::new((set.clone(), set.clone()), table);
	assert!(r1.is_reflexive());
}
#[test]
fn is_irreflexive() {
	let set: Set = ALPHABET[0..4].iter().collect();
	let table = vec![
		false, true,  true,  true,
		false, false, true,  true,
		true,  true,  false, true,
		false, true,  false, false,
	];
	let r1 = RelationVec::new((set.clone(), set.clone()), table);
	assert!(r1.is_irreflexive());
}
#[test]
fn is_antisymmetric() {
	let set: Set = ALPHABET[0..4].iter().collect();
	let table = vec![
		true,  false, false, false,
		false, true,  true,  false,
		true,  false, false, false,
		false, true,  false, true,
	];
	let r1 = RelationVec::new((set.clone(), set.clone()), table);
	assert!(r1.is_antisymmetric());
}
#[test]
fn is_transitive() {
	let set: Set = ALPHABET[0..4].iter().collect();
	let table = vec![
		false, false, false, false,
		true,  false, true,  false,
		true,  false, false, false,
		true,  true,  true,  false,
	];
	let r1 = RelationVec::new((set.clone(), set.clone()), table);
	assert!(r1.is_transitive());
}

#[test]
fn is_symmetric() {
	let set: Set = ALPHABET[0..4].iter().collect();
	let table = vec![
		true,  false, true,  false,
		false, true,  true,  true,
		true,  true,  false, false,
		false, true,  false, true,
	];
	let r1 = RelationVec::new((set.clone(), set.clone()), table);
	assert!(r1.is_symmetric());
}
#[test]
fn is_asymmetric() {
	let set: Set = ALPHABET[0..4].iter().collect();
	let table = vec![
		false, false, false, false,
		false, false, true,  false,
		true,  false, false, false,
		false, true,  false, false,
	];
	let r1 = RelationVec::new((set.clone(), set.clone()), table);
	assert!(r1.is_asymmetric());
}

#[test]
fn is_preorder() {
	let set: Set = ALPHABET[0..5].iter().collect();
	let table = vec![
		true,  false, true,  true,  false,
		false, true,  false, false, false,
		false, false, true,  true,  false,
		false, false, true,  true,  false,
		false, false, false, false, true,
	];
	let r1 = RelationVec::new((set.clone(), set.clone()), table);
	assert!(r1.is_reflexive());
	assert!(r1.is_transitive());
	assert!(!r1.is_symmetric());
	assert!(r1.is_preorder());
	assert!(!r1.is_equivalent());
}
/*
fn is_partial_order() {}
*/
#[test]
fn is_equivalent() {
	let set: Set = ALPHABET[0..5].iter().collect();
	let table = vec![
		true,  false, true,  true,  false,
		false, true,  false, false, false,
		true,  false, true,  true,  false,
		true,  false, true,  true,  false,
		false, false, false, false, true,
	];
	let r1 = RelationVec::new((set.clone(), set.clone()), table);
	assert!(r1.is_reflexive());
	assert!(r1.is_transitive());
	assert!(r1.is_symmetric());
	assert!(r1.is_preorder());
	assert!(r1.is_equivalent());
}

#[test]
fn is_difunctional() {
	let set: Set = ALPHABET[0..10].iter().collect();
	let table = vec![
		false, false, true,  false, false, true,  false, true,  true,  true,
		false, false, false, true,  true,  false, false, false, false, false,
		false, false, false, true,  true,  false, false, false, false, false,
		false, false, true,  false, false, true,  false, true,  true,  true,
		false, false, false, false, false, false, false, false, false, false,
		true,  false, false, false, false, false, false, false, false, false,
		true,  false, false, false, false, false, false, false, false, false,
		false, true,  false, false, false, false, false, false, false, false,
		false, false, false, true,  true,  false, false, false, false, false,
		false, false, false, false, false, false, false, false, false, false,
	];
	let r1 = RelationVec::new((set.clone(), set.clone()), table);
	assert!(r1.is_difunctional());
}

/*
fn is_lattice() {}
fn is_sublattice() {}

fn is_injective() {}
fn is_functional() {}
fn is_lefttotal() {}
fn is_surjective() {}
fn is_bijective() {}
fn is_function() {}
*/

#[test]
fn relation_mod8_equal() {
	let n32: Vec<u8> = (1..=32).collect();
	let r = RelationVec::from_predicate(&n32, |(&x, &y)| x % 8 == y % 8);
	// TODO verify
	assert!(r.is_reflexive());
	assert!(!r.is_irreflexive());
	assert!(!r.is_antisymmetric());
	assert!(r.is_symmetric());
	assert!(!r.is_asymmetric());
	assert!(r.is_transitive());
	assert!(r.is_lefttotal());
	assert!(!r.is_injective());
	assert!(r.is_surjective());
}
#[test]
fn relation_divisible() {
	let n32: Vec<u8> = (1..=32).collect();
	let r = RelationVec::from_predicate(&n32, |(&x, &y)| y % x == 0);
	assert!(r.is_reflexive());
	assert!(!r.is_irreflexive());
	assert!(r.is_antisymmetric());
	assert!(!r.is_symmetric());
	assert!(!r.is_asymmetric());
	assert!(r.is_transitive());
	assert!(r.is_lefttotal());
	assert!(!r.is_injective());
	assert!(r.is_surjective());
}


#[test]
fn new_endorelation() {
	let n8: Vec<u8> = (1..=8).collect();
	let s8: (&Set, &Set) = (
		&(1..=8).map(|i| i.to_string()).collect(),
		&(1..=8).map(|i| i.to_string()).collect(),
	);
	let top = RelationVec::from_predicate(&n8, |(&x, &y)| x >= y);
	let bot = RelationVec::from_predicate(&n8, |(&x, &y)| x <= y);

	assert!(relation::eq(
		&RelationVec::union(
			&top,
			&bot,
		),
		&RelationVec::universal(s8)
	));
	assert!(relation::eq(
		&RelationVec::intersection(
			&RelationVec::from_predicate(&n8, |(&x, &y)| x <= y),
			&RelationVec::from_predicate(&n8, |(&x, &y)| x >= y),
		),
		&RelationVec::identity(s8)
	));

	assert!(relation::eq(
		&RelationVec::union(
			&RelationVec::complement(&top),
			&RelationVec::identity(s8),
		),
		&bot
	));
	assert!(relation::eq(
		&RelationVec::union(
			&RelationVec::complement(&bot),
			&RelationVec::identity(s8),
		),
		&top
	));

	assert!(relation::eq(
		&RelationVec::converse(&top),
		&bot,
	));
	assert!(relation::eq(
		&RelationVec::converse(&bot),
		&top,
	));
}

pub fn union<R, S, T>(a: &R, b: &S, c: &T)
where R: Endorelation + std::fmt::Debug,
      S: Endorelation + std::fmt::Debug,
      T: Endorelation + std::fmt::Debug,
{
	let r = a;
	assert!(r.is_homogeneous());

	let neutral = &R::empty(r.get_domain());
	let absorbing = &R::universal(r.get_domain());

	// union: neutral element
	assert!(relation::eq(&R::union(r, neutral), r));
	// union: absorbing element
	assert!(relation::eq(&R::union(r, absorbing), absorbing));
	// union: idempotence
	assert!(relation::eq(&R::union(r, r), r));
	// union: associativity
	assert!(relation::eq(
		&R::union(a, &R::union(b, c)),
		&R::union(&R::union(a, b), c),
	));
	// union: commutativity
	assert!(relation::eq(&R::union(a, b), &R::union(b, a)));
}

pub fn intersection<R, S, T>(a: &R, b: &S, c: &T)
where R: Endorelation + std::fmt::Debug,
      S: Endorelation + std::fmt::Debug,
      T: Endorelation + std::fmt::Debug,
{
	let r = a;
	assert!(r.is_homogeneous());
	
	let neutral = &R::universal(r.get_domain());
	let absorbing = &R::empty(r.get_domain());

	// intersection: neutral element
	assert!(relation::eq(&R::intersection(r, neutral), r));
	// intersection: absorbing element
	assert!(relation::eq(&R::intersection(r, absorbing), absorbing));
	// intersection: idempotence
	assert!(relation::eq(&R::intersection(r, r), r));
	// intersection: associativity
	assert!(relation::eq(
		&R::intersection(a, &R::intersection(b, c)),
		&R::intersection(&R::intersection(a, b), c)
	));
	// intersection: commutativity
	assert!(relation::eq(&R::intersection(a, b), &R::intersection(b, a)));
}

pub fn distributivity_union_intersection<R, S, T>(a: &R, b: &S, c: &T)
where R: Endorelation + std::fmt::Debug,
      S: Endorelation + std::fmt::Debug,
      T: Endorelation + std::fmt::Debug,
{
	// left distributivity (union, intersection)
	assert!(relation::eq(
		&R::intersection(a, &R::union(b, c)),
		&R::union(&R::intersection(a, b), &R::intersection(a, c)),
	));
	// right distributivity (union, intersection)
	assert!(relation::eq(
		&R::intersection(&R::union(a, b), c),
		&R::union(&R::intersection(a, c), &R::intersection(b, c)),
	));
}

pub fn de_morgan<R>(a: &R, b: &R)
where R: Relation + std::fmt::Debug
{
	assert!(relation::eq(
		&R::complement(&R::union(a, b)),
		&R::intersection(&R::complement(a), &R::complement(b)),
	));
	assert!(relation::eq(
		&R::complement(&R::intersection(a, b)),
		&R::union(&R::complement(a), &R::complement(b)),
	));
}

#[test]
fn relation_property_test() {
	let n8: Vec<u8> = (1..=8).collect();
	let s8: (&Set, &Set) = (
		&(1..=8).map(|i| i.to_string()).collect(),
		&(1..=8).map(|i| i.to_string()).collect(),
	);

	let div = RelationVec::from_predicate(&n8, |(&x, &y)| y % x == 0);
	let le = RelationVec::from_predicate(&n8, |(&x, &y)| x <= y);
	let ge = RelationVec::from_predicate(&n8, |(&x, &y)| x >= y);

	union(&div, &le, &ge);
	intersection(&div, &le, &ge);
	distributivity_union_intersection(&div, &le, &ge);
	de_morgan(&div, &le);

	let lt = RelationVec::from_predicate(&n8, |(&x, &y)| x < y);
	assert!(relation::eq(
		&RelationVec::complement(&ge),
		&lt
	));
	assert!(relation::eq(
		&RelationVec::intersection(&RelationVec::identity(s8), &RelationVec::universal(s8)),
		&RelationVec::union(&RelationVec::identity(s8), &RelationVec::empty(s8))
	));
}
