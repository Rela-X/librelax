extern crate relax;

use relax::{Set, SetElement};
use relax::relation::{Relation, Endorelation, RelationVec};

const ALPHABET: [char; 26] = [
	'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
	'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
	'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
	'y', 'z',
];

/*
#[test]
fn is_reflexive() {
	let domain = &ALPHABET[0..3];
	let r1 = RelationVec {
		domain: (domain, domain),
		table: vec![
			true,  false, false,
			true,  true,  false,
			false, false, true,
		],
	};
	assert!(r1.is_reflexive());
}
#[test]
fn is_irreflexive() {
	let domain = &ALPHABET[0..4];
	let r1 = RelationVec {
		domain: (domain, domain),
		table: vec![
			false, true,  true,  true,
			false, false, true,  true,
			true,  true,  false, true,
			false, true,  false, false,
		],
	};
	assert!(r1.is_irreflexive());
}
#[test]
fn is_antisymmetric() {
	let domain = &ALPHABET[0..4];
	let r1 = RelationVec {
		domain: (domain, domain),
		table: vec![
			true,  false, false, false,
			false, true,  true,  false,
			true,  false, false, false,
			false, true,  false, true,
		],
	};
	assert!(r1.is_antisymmetric());
}
#[test]
fn is_transitive() {
	let domain = &ALPHABET[0..4];
	let r1 = RelationVec {
		domain: (domain, domain),
		table: vec![
			false, false, false, false,
			true,  false, true,  false,
			true,  false, false, false,
			true,  true,  true,  false,
		]
	};
	assert!(r1.is_transitive());
}

#[test]
fn is_symmetric() {
	let domain = &ALPHABET[0..4];
	let r1 = RelationVec {
		domain: (domain, domain),
		table: vec![
			true,  false, true,  false,
			false, true,  true,  true,
			true,  true,  false, false,
			false, true,  false, true,
		],
	};
	assert!(r1.is_symmetric());
}
#[test]
fn is_asymmetric() {
	let domain = &ALPHABET[0..4];
	let r1 = RelationVec {
		domain: (domain, domain),
		table: vec![
			false, false, false, false,
			false, false, true,  false,
			true,  false, false, false,
			false, true,  false, false,
		]
	};
	assert!(r1.is_asymmetric());
}

#[test]
fn is_preorder() {
	let domain = &ALPHABET[0..5];
	let r1 = RelationVec {
		domain: (domain, domain),
		table: vec![
			true,  false, true,  true,  false,
			false, true,  false, false, false,
			false, false, true,  true,  false,
			false, false, true,  true,  false,
			false, false, false, false, true,  
		],
	};
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
	let domain = &ALPHABET[0..5];
	let r1 = RelationVec {
		domain: (domain, domain),
		table: vec![
			true,  false, true,  true,  false,
			false, true,  false, false, false,
			true,  false, true,  true,  false, 
			true,  false, true,  true,  false, 
			false, false, false, false, true,  
		],
	};
	assert!(r1.is_reflexive());
	assert!(r1.is_transitive());
	assert!(r1.is_symmetric());
	assert!(r1.is_preorder());
	assert!(r1.is_equivalent());
}

#[test]
fn is_difunctional() {
	let domain = &ALPHABET[0..10];
	let r1 = RelationVec {
		domain: (domain, domain),
		table: vec![
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
		],
	};
	assert!(r1.is_difunctional());
}
*/

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
	let n32: Vec<u32> = (1..=32).collect::<Vec<_>>();
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
	let n32: Vec<u32> = (1..=32).collect::<Vec<_>>();
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
	let n8: Vec<usize> = (1..=8).collect();
	let s8: Set = (1..=8).map(|e| SetElement::from(e.to_string())).collect(); // oh boy
	let top = RelationVec::from_predicate(&n8, |(&x, &y)| x >= y);
	let bot = RelationVec::from_predicate(&n8, |(&x, &y)| x <= y);

	assert_eq!(
		RelationVec::union(
			&top,
			&bot,
		),
		RelationVec::universal(&s8)
	);
	assert_eq!(
		RelationVec::intersection(
			&RelationVec::from_predicate(&n8, |(&x, &y)| x <= y),
			&RelationVec::from_predicate(&n8, |(&x, &y)| x >= y),
		),
		RelationVec::identity(&s8)
	);

	assert_eq!(
		RelationVec::union(
			&RelationVec::complement(&top),
			&RelationVec::identity(&s8),
		),
		bot
	);
	assert_eq!(
		RelationVec::union(
			&RelationVec::complement(&bot),
			&RelationVec::identity(&s8),
		),
		top
	);

	assert_eq!(
		RelationVec::converse(&top),
		bot,
	);
	assert_eq!(
		RelationVec::converse(&bot),
		top,
	);
}

#[test]
fn foobar() {
	let n8: Vec<u8> = (1..=8).collect();
	let s8: Set = (1..=8).map(|e| SetElement::from(e.to_string())).collect(); // oh boy

	let empty = RelationVec::empty(&s8);
	let universal = RelationVec::universal(&s8);
	let div = RelationVec::from_predicate(&n8, |(&x, &y)| y % x == 0);
	let le = RelationVec::from_predicate(&n8, |(&x, &y)| x <= y);
	let ge = RelationVec::from_predicate(&n8, |(&x, &y)| x >= y);
	relax::relation::relation_tabular::tests::union(&empty, &universal, &div, &le, &ge);
	relax::relation::relation_tabular::tests::intersection(&universal, &empty, &div, &le, &ge);
	relax::relation::relation_tabular::tests::distributivity_union_intersection(&div, &le, &ge);
	relax::relation::relation_tabular::tests::de_morgan(&div, &le);

	let lt = RelationVec::from_predicate(&n8, |(&x, &y)| x < y);
	assert_eq!(
		RelationVec::complement(&ge),
		lt
	);
	assert_eq!(
		RelationVec::intersection(&RelationVec::identity(&s8), &RelationVec::universal(&s8)),
		RelationVec::union(&RelationVec::identity(&s8), &RelationVec::empty(&s8))
	);
}
