use std::fmt;
use std::iter;

use crate::relation::relation::Relation;

pub struct TeXWrapper<'a, R: Relation>(&'a R);

pub trait ToTex<R: Relation> {
	fn to_tex(&self) -> TeXWrapper<R>;
}

// Implement ToTex for every Relation
impl<R: Relation> ToTex<R> for R {
	/// Create a wrapper-object that prints the [`Relation`]
	/// in LaTeX format.
	/// The `Relation` is formatted using the `array`-environment
	/// with `\true` and `\false` cells to allow for customized
	/// display of boolean values within the LaTeX document.
	///
	/// # Examples
	///
	/// ```
	/// use relax::tex::ToTex;
	///
	/// let r = relax::RelationVec::from_predicate(&(1..5).collect::<Vec<_>>(), |(x, y)| x < y);
	/// // write the following output to stdout:
	/// //     \begin{array}{c|cccc}
	/// //       & 1      & 2      & 3      & 4 \hline \\
	/// //     1 & \false & \true  & \true  & \true  \\
	/// //     2 & \false & \false & \true  & \true  \\
	/// //     3 & \false & \false & \false & \true  \\
	/// //     4 & \false & \false & \false & \false
	/// //     \end{array}
	/// println!("{}", r.to_tex());
	/// ```
	fn to_tex(&self) -> TeXWrapper<R> {
		TeXWrapper(self)
	}
}

impl<'a, R: Relation> fmt::Display for TeXWrapper<'a, R> {
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
