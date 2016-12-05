use std::collections::BTreeSet;
use std::collections::HashMap;

use solver::cnf;


pub fn create_cnf_unchecked(descr: &str) -> (cnf::Cnf, HashMap<u64, String>) {
	let mut clauses = BTreeSet::new();
	let mut clause = BTreeSet::new();
	let mut negative_flag = false;
	let mut buffer = String::new();
	let mut label_map: HashMap<String, u64> = HashMap::new();
	let mut reverse_map: HashMap<u64, String> = HashMap::new();
	let mut label_iter = 0..;
	for c in descr.chars() {
		match c {
			'!' | '~' | '-' => {
				negative_flag = true;
			}
			'|' => {
				let v = *label_map.entry(buffer.clone())
					.or_insert_with(|| {
						let k = label_iter.next().unwrap();
						reverse_map.insert(k, buffer.clone());
						k
					});
				let l = cnf::Literal(v, !negative_flag);
				clause.insert(l);
				buffer = String::new();
			}
			'&' => {
				let v = *label_map.entry(buffer.clone())
					.or_insert_with(|| {
						let k = label_iter.next().unwrap();
						reverse_map.insert(k, buffer.clone());
						k
					});
				let l = cnf::Literal(v, !negative_flag);
				clause.insert(l);
				buffer = String::new();
				clauses.insert(clause);
				clause = BTreeSet::new();
			}
			' ' | '(' | ')' => {
			}
			a => {
				buffer.push(a);
			}
		}
	}
	let v = *label_map.entry(buffer.clone())
		.or_insert_with(|| {
			let k = label_iter.next().unwrap();
			reverse_map.insert(k, buffer.clone());
			k
		});
	let l = cnf::Literal(v, !negative_flag);
	clause.insert(l);
	clauses.insert(clause);
	(cnf::Cnf{ clauses: clauses }, reverse_map)
}
