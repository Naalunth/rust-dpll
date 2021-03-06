use cnf;
use std::collections::BTreeSet;
use std::collections::BTreeMap;

pub fn get_solving_assignment(cnf: &cnf::Cnf) -> Option<Vec<cnf::Literal>> {
	get_solving_assignment_helper(&without_trivial_clauses(cnf))
}

fn get_solving_assignment_helper(cnf: &cnf::Cnf) -> Option<Vec<cnf::Literal>> {
	if cnf.clauses.len() == 0 {
		Some(Vec::new())
	} else if cnf.clauses.iter().any(|c| c.len() == 0) {
		None
	} else {
		let ret_val =
		if let Some(l) = get_single_literal(&cnf).or_else(|| get_pure_literal(&cnf)) {
			get_solving_assignment_helper(&up(cnf, l)).map(|a| (a, l))
		} else {
			cnf.clauses.iter()
				.flat_map(|c| c.iter())
				.collect::<BTreeSet<_>>().iter()
				.filter_map(|&l| get_solving_assignment_helper(&up(cnf, *l)).map(|a| (a, *l)))
				.next()
		};
		match ret_val {
			Some((mut a, l)) => {
				a.push(l);
				Some(a)
			},
			None => None
		}
	}
}

fn without_trivial_clauses(cnf: &cnf::Cnf) -> cnf::Cnf {
	cnf::Cnf {
		clauses: cnf.clauses.iter()
			.cloned()
			.filter(|c| !c.iter()
				.fold(BTreeMap::new(), |mut map: BTreeMap<u64, u8>, l| {
					*map.entry(l.0).or_insert(0) |= if l.1 {0b01} else {0b10};
					map
				}).iter()
				.any(|e| *e.1 == 0b11))
			.collect()
	}
}

fn up(clause_set: &cnf::Cnf, literal: cnf::Literal) -> cnf::Cnf {
	cnf::Cnf {
		clauses: clause_set.clauses.iter()
			.filter(|ref c| !c.iter().any(|&l| l == literal))
			.map(|ref c| c.iter()
				.cloned()
				.filter(|&l| l != !literal)
				.collect())
			.collect()
	}
}

fn get_single_literal(cnf: &cnf::Cnf) -> Option<cnf::Literal> {
	cnf.clauses.iter()
		.filter(|c| c.len() == 1)
		.next()
		.and_then(|c| c.iter().cloned().next())
}

fn get_pure_literal(cnf: &cnf::Cnf) -> Option<cnf::Literal> {
	use cnf::Literal;
	cnf.clauses.iter()
		.flat_map(|c| c.iter())
		.fold(BTreeMap::new(), |mut map: BTreeMap<u64, u8>, l| {
			*map.entry(l.0).or_insert(0) |= if l.1 {0b01} else {0b10};
			map
		}).iter()
		.filter_map(|e| match e {
			(&x, &0b01) => Some(Literal(x, true)),
			(&x, &0b10) => Some(Literal(x, false)),
			_ => None
		})
		.next()
}
