pub mod relation;
pub mod endorelation;
pub mod relation_tabular;
pub mod relation_vec;

mod union;
mod intersection;
mod complement;
mod converse;
mod concatenation;

mod empty;
mod universal;
mod identity;

pub use relation::relation::Relation;
pub use relation::endorelation::Endorelation;
pub use relation::relation_tabular::RelationTabular;
pub use relation::relation_vec::RelationVec;
