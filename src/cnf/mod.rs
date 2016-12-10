use std::collections::BTreeSet;
use std::ops::Not;

#[derive(Debug)]
pub struct Cnf {
	pub clauses: BTreeSet<BTreeSet<Literal>>
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Literal(pub u64, pub bool);

impl Cnf {
	pub fn new() -> Cnf {
		Cnf {
			clauses: BTreeSet::new()
		}
	}
}

impl Not for Literal {
	type Output = Literal;
	fn not(self) -> Literal {
		match self {
			Literal(x, b) => Literal(x, !b)
		}
	}
}
