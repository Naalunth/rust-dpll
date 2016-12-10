#![feature(test)]

pub mod cnf;
pub mod dpll;
pub mod resolution;
pub mod parser;

extern crate test;


#[cfg(test)]
mod tests {
	use super::*;
	use test::Bencher;

	static INPUT: &'static str = "(B|~E|~D|~A)&(B|D|A|~E)&(A|~E|~B|~D)&(C|~A)&(E|~D|~A)&(D|~E|~A)&(E|D|~C|~A)&(~E|~A)&(A)";
	//static INPUT: &'static str = "(B|~E|~D|~A)&(B|D|A|~E)&(A|~E|~B|~D)&(C|~A)";

	#[bench]
	fn bench_resolver_non_batched(b: &mut Bencher) {
		let (cnf, _) = parser::create_cnf_checked(INPUT);
		b.iter(|| resolution::is_unsolvable(&cnf));
	}
}
