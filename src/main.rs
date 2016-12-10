extern crate sat_solver as solver;


use std::collections::HashMap;
use std::vec::Vec;

fn format_assignment(assignment: Vec<solver::cnf::Literal>, map: HashMap<u64, String>) -> String {
	let mut res: String = assignment.iter()
		.map(|l| format!("{}\\{}, ", map.get(&l.0).unwrap(), l.1))
		.collect();
	if res.len() >= 2 {
		let len = res.len() - 2;
		res.truncate(len);
	}
	res
}

fn main() {
	//let input = "(B|~E|~D|~A)&(B|D|A|~E)&(A|~E|~B|~D)&(C|~A)&(E|~D|~A)&(D|~E|~A)&(E|D|~C|~A)&(~E|~A)";
	let input = "(B|~E|~D|~A)&(B|D|A|~E)&(A|~E|~B|~D)&(C|~A)&(E|~D|~A)&(D|~E|~A)&(E|D|~C|~A)&(~E|~A)&(A)";
	//let input = "(A|B)&(A|~B)&(~A|B)&(~A|~B)";
	println!("input: {:?}", input);
	let (cnf, map) = solver::parser::create_cnf_checked(input);
	println!("cnf: {:?}", cnf);
	let a = solver::dpll::dpll3::get_solving_assignment(&cnf);
	println!("assignment: {:?}; map: {:?}", a, map);
	if a.is_some() {
		println!("assignment pretty: {}", format_assignment(a.unwrap(), map));
	}
	let resolution = solver::resolution::is_unsolvable(&cnf);
	println!("resolution: is_unsolvable: {:?}", resolution);
}
