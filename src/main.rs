use std::env;
use std::ascii::AsciiExt;
use commands::*;
use utils::config::{Config};

mod commands;

mod utils {
	pub mod config;
}

fn main() {
	
	// TODO: handle errors in here
	let mut config = Config::read().unwrap();
	
	let args:Vec<_> = env::args().collect();
	if args.len() < 2 {
		println!("Not enough parameters supplied. Minimum of 1 parameters required");
		help::print_help();
		std::process::exit(1);
	} else {
		let (goal_args, addl_args) = args.split_at(2);
		let ref goal = goal_args[1];

		if "install".eq_ignore_ascii_case(goal) {
			install::install(&config, addl_args);
		} else if "list".eq_ignore_ascii_case(goal) {
			list::list(&config, addl_args);
		} else if "swap".eq_ignore_ascii_case(goal) {
			swap::swap(&mut config, addl_args);
		} else if "link".eq_ignore_ascii_case(goal) {
			link::link(&mut config, addl_args);
		} else if "rebuild".eq_ignore_ascii_case(goal) {
			rebuild::rebuild(&mut config, addl_args);
		} else if "help".eq_ignore_ascii_case(goal) {
			help::help(&config, addl_args);
		} else {
			println!("Unknown argument {:?}", goal);
			help::print_help();
			std::process::exit(2);
		}
	}
}
