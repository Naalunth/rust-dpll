use std::cmp::Ordering;
use std::collections::BTreeSet;
//use std::collections::HashSet;
//use std::collections::BTreeMap;
use std::collections::BinaryHeap;

use cnf;
use cnf::Literal;

type Clause = BTreeSet<Literal>;
type ClauseSet = BTreeSet<Clause>;

#[derive(Debug, Clone, Eq, PartialEq)]
struct HeapElement {
	clause: Clause
}

impl Ord for HeapElement {
	fn cmp(&self, other: &HeapElement) -> Ordering {
		other.clause.len().cmp(&self.clause.len())
	}
}

impl PartialOrd for HeapElement {
	fn partial_cmp(&self, other: &HeapElement) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

pub fn is_unsolvable(cnf: &cnf::Cnf) -> bool {
	let mut clauses: BTreeSet<Clause> = BTreeSet::new();
	let mut clause_queue: BinaryHeap<HeapElement> = cnf.clauses.iter()
		.cloned()
		.map(|c| HeapElement{clause: c})
		.collect();
	while let Some(HeapElement{clause: heap_elem}) = clause_queue.pop() {
		//println!("size: {:?}, elem: {:?}", clause_queue.len(), heap_elem);
		if heap_elem.len() == 0 {
			return true;
		}
		let resolvents: ClauseSet = clauses.iter()
			.flat_map(|c| get_resolvents(c, &heap_elem))
			.collect();
		let mut resolvents2 = resolvents.into_iter()
			.filter(|c| !clauses.contains(c))
			.filter(|c| !clause_queue.iter().any(|e| e.clause == *c))
			.map(|c| HeapElement{clause: c})
			.collect();
		clause_queue.append(&mut resolvents2);
		clauses.insert(heap_elem);
	}
	return false;
}

fn get_resolvents(a: &Clause, b: &Clause) -> Vec<Clause> {
	a.iter()
		.filter(|&l| b.contains(&!*l))
		.map(|l| &(a - &vec![*l].into_iter().collect()) | &(b - &vec![!*l].into_iter().collect()))
		.collect()
}

/*

fn pop_n(heap: &mut BinaryHeap<HeapElement>, n: u64) -> ClauseSet {
	let mut clauses: ClauseSet = BTreeSet::new();
	let mut i = n;
	while let Some(elem) = heap.pop() {
		clauses.insert(elem.clause);
		if i == 0 {
			break;
		} i -= 1;
	}
	return clauses;
}


fn does_contain_empty_clause(clauses: &ClauseSet) -> bool {
	clauses.iter().any(|c| c.len() == 0)
}


fn without_trivial_clauses(cnf: &cnf::Cnf) -> cnf::Cnf {
	cnf::Cnf {
		clauses: cnf.clauses.iter()
			.cloned()
			.filter(|c| !is_clause_trivial(c))
			.collect()
	}
}

fn is_clause_trivial(c: &Clause) -> bool {
	c.iter()
		.fold(BTreeMap::new(), |mut map: BTreeMap<u64, u8>, l| {
			*map.entry(l.0).or_insert(0) |= if l.1 {0b01} else {0b10};
			map
		}).iter()
		.any(|e| *e.1 == 0b11)
}
*/
