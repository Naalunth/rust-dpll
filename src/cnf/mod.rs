use std::collections::HashSet;
use std::collections::BTreeSet;

#[derive(Debug)]
pub struct Cnf {
	pub clauses: HashSet<BTreeSet<Literal>>
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Literal(pub u64, pub bool);

impl Cnf {
	pub fn new() -> Cnf {
		Cnf {
			clauses: HashSet::new()
		}
	}
}

impl Literal {
	pub fn negate(&self) -> Literal {
		match *self {
			Literal(x, b) => Literal(x, !b)
		}
	}
}
