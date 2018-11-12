use std::fmt;
use std::iter;

use Relation;

pub struct TeXWrapper<'a, R: 'a + Relation>(&'a R);

pub trait ToTex<R: Relation> {
	fn to_tex(&self) -> TeXWrapper<R>;
}

// Implement ToTex for every Relation
impl<R: Relation> ToTex<R> for R {
	/// Create a wrapper-object that prints the [`Relation`]
	/// in LaTeX format using the array-Environment.
	///
	/// # Examples
	///
	/// ```
	/// use relax::tex::ToTex;
	///
	/// let r = relax::RelationVec::from_predicate(&(1..5).collect::<Vec<_>>(), |(x, y)| x < y);
	/// println!("{}", r.to_tex());
	/// ```
	fn to_tex(&self) -> TeXWrapper<R> {
		TeXWrapper(self)
	}
}

impl<'a, R: 'a + Relation> fmt::Display for TeXWrapper<'a, R> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let table_width = self.0.get_domain().0.cardinality();
		write!(f, "{}", r"\begin{array}")?;
		writeln!(f, "{{c|{:c^width$}}}", "", width = table_width)?;
		for y in self.0.get_domain().1.iter() {
			write!(f, " & {}", y)?;
		}
		write!(f, " {}", r"\hline")?;
		let fn_eval = |(ix, iy)| self.0.eval_at(ix, iy);
		for (ix, x) in self.0.get_domain().0.iter().enumerate() {
			writeln!(f, " {}", r"\\")?;
			write!(f, "{}", x)?;
			for b in iter::repeat(ix).zip(self.0.iys()).map(fn_eval) {
				write!(f, " & {}", if b { r"\true " } else { r"\false" })?;
			}
		}
		writeln!(f, "")?;
		write!(f, "{}", r"\end{array}")
	}
}
