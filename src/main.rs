use std::env;
use std::ascii::AsciiExt;
use commands::*;
use std::collections::HashMap;
use utils::config::{Config};

mod commands;

mod utils {
	pub mod config;
}

fn main() {
	let args: Vec<_> = env::args().collect();
	
	match Config::read() {
		Ok(cfg_opt) => {
			if args.len() < 2 {
				println!("Not enough parameters supplied. Minimum of 1 parameters required");
				help::print_help();
				std::process::exit(1);
			}
			let (goal_args, addl_args) = args.split_at(2);
			run_pman(&goal_args[1], cfg_opt, addl_args);
		},
		Err(e) => {
			println!("Error occured while reading configuration, {}", e);
		}
	}
}
fn run_pman(goal: &String, config: Option<Config>, addl_args: &[String]) {
	
	// these 2 goals dont need configuration to run, check for those
	if "help".eq_ignore_ascii_case(goal) {
		help::run(addl_args);
	} else if "configure".eq_ignore_ascii_case(goal) {
		configure::run(config, addl_args);
	} else {
		// these goals require configuration to exist. Make sure it does
		if config.is_none() {
			println!("pman is not configured yet!\nRun 'pman help configure' for more information");
			std::process::exit(1);
		}
		
		match goal.to_ascii_lowercase().as_ref() {
			"install" => install::run(config.unwrap(), addl_args),
			"list" => list::run(config.unwrap(), addl_args),
			"swap" => swap::run(config.unwrap(), addl_args),
			"link" => link::run(config.unwrap(), addl_args),
			"rebuild" => rebuild::run(config.unwrap(), addl_args),
			_ => {
				println!("Unknown argument {:?}", goal);
				help::print_help();
				std::process::exit(2);
			}
		}
	}
}
