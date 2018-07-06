extern crate relax;

use relax::Relation;

fn relation_property_test<T: Relation>(r: &T) {
	assert_eq!(r.is_homogeneous(), !r.is_heterogeneous());
	assert_eq!(r.is_reflexive() && r.is_irreflexive(), false);

	assert_eq!(r.is_symmetric() && r.is_antisymmetric(), false);
	assert_eq!(r.is_asymmetric(), r.is_irreflexive() && r.is_antisymmetric());

	assert_eq!(r.is_preorder(), r.is_reflexive() && r.is_transitive());
	assert_eq!(r.is_partial_order(), r.is_preorder() && r.is_antisymmetric());
	assert_eq!(r.is_equivalent(), r.is_preorder() && r.is_symmetric());

	assert_eq!(r.is_bijective(), r.is_injective() && r.is_surjective());
	assert_eq!(r.is_function(), r.is_functional() && r.is_lefttotal());
}

/*
fn union<T: RelationTabular + std::fmt::Debug>(neutral: &T, absorbing: &T, a: &T, b: &T, c: &T) {
	let r = a;
	assert!(r.is_homogeneous());

	// union: neutral element
	assert_eq!(Relation::union(r, neutral), r);
	// union: absorbing element
	assert_eq!(Relation::union(r, absorbing), absorbing);
	// union: idempotence
	assert_eq!(Relation::union(r, r), r);
	// union: associativity
	assert_eq!(
		Relation::union(
			a,
			Relation::union(b, c),
		),
		Relation::union(
			Relation::union(a, b),
			c,
		),
	);
	// union: commutativity
	assert_eq!(
		Relation::union(a, b),
		Relation::union(b, a),
	);
}

fn intersection<T: Relation + std::fmt::Debug>(neutral: &T, absorbing: &T, a: &T, b: &T, c: &T) {
	let r = a;
	assert!(r.is_homogeneous());

	// intersection: neutral element
	assert_eq!(Relation::intersection(r, neutral), r);
	// intersection: absorbing element
	assert_eq!(Relation::intersection(r, absorbing), absorbing);
	// intersection: idempotence
	assert_eq!(Relation::intersection(r, r), r);
	// intersection: associativity
	assert_eq!(
		Relation::intersection(
			a,
			Relation::intersection(b, c),
		),
		Relation::intersection(
			Relation::intersection(a, b),
			c,
		),
	);
	// intersection: commutativity
	assert_eq!(
		Relation::intersection(a, b),
		Relation::intersection(b, a),
	);
}

fn distributivity_union_intersection<T: Relation + std::fmt::Debug>(a: &T, b: &T, c: &T) {
	// left distributivity (union, intersection)
	assert_eq!(
		Relation::intersection(
			a,
			Relation::union(b, c)
		),
		Relation::union(
			Relation::intersection(a, b),
			Relation::intersection(a, c),
		),
	);
	// right distributivity (union, intersection)
	assert_eq!(
		Relation::intersection(
			Relation::union(a, b),
			c,
		),
		Relation::union(
			Relation::intersection(a, b),
			Relation::intersection(a, c),
		),
	);
}
*/
