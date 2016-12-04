use cnf;
use std::collections::BTreeSet;

pub fn get_solving_assignment(cnf: &cnf::Cnf) -> Option<Vec<cnf::Literal>> {
	if cnf.clauses.len() == 0 {
		Some(Vec::new())
	} else if cnf.clauses.iter().all(|c| c.len() == 0) {
		None
	} else {
		match if let Some(c) = cnf.clauses.iter().filter(|c| c.len() == 1).next() {
			let l = *c.iter().next().unwrap();
			get_solving_assignment(&up(cnf, l)).map(|a| (a, l))
		} else if let Some(l) = {
			cnf.clauses.iter()
				.flat_map(|c| c.iter())
				.map(|l| l.0)
				.collect::<BTreeSet<_>>().iter()
				.filter_map(|x| {
					let bools = cnf.clauses.iter()
						.flat_map(|c| c.iter())
						.filter(|l| l.0 == *x)
						.map(|l| l.1)
						.collect::<BTreeSet<_>>();
					if bools.len() == 1 {
						Some(cnf::Literal(*x, *bools.iter().next().unwrap()))
					} else { None }
				})
				.next()
		} {
			get_solving_assignment(&up(cnf, l)).map(|a| (a, l))
		} else {
			match cnf.clauses.iter()
				.flat_map(|c| c.iter())
				.collect::<BTreeSet<_>>().iter()
				.map(|&l| get_solving_assignment(&up(cnf, *l)).map(|a| (a, *l)))
				.find(|r| r.is_some()) {
				Some(r) => r,
				None => None
			}
		} {
			Some((mut a, l)) => {
				a.push(l);
				Some(a)
			},
			None => None
		}
	}
}

pub fn up(clause_set: &cnf::Cnf, literal: cnf::Literal) -> cnf::Cnf {
	cnf::Cnf {
		clauses: clause_set.clauses.iter()
			.filter(|ref c| !c.iter().any(|&l| l == literal))
			.map(|ref c| c.iter()
				.cloned()
				.filter(|&l| l != literal.negate())
				.collect())
			.collect()
	}
}
