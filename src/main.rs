use std::env;
use std::ascii::AsciiExt;
use commands::*;
use utils::config::{Config, ConfigEntry};

mod commands;

mod utils {
	pub mod config;
}

fn main() {
//	let dir_str = "D:\\workdir_rust\\pack\\target\\debug\\path";
//	let dir = std::path::Path::new(dir_str);
//	let path = std::path::Path::new("..\\");
//	let new_path: std::path::PathBuf = dir.join(&path);
//	
//	
//	println!("{}", dir.to_str().unwrap());
//	println!("is_relative {}", dir.is_relative());
//	println!("is_dir {}", dir.is_dir());
//	println!("exists {}", dir.exists());
//	
//	println!("{}", path.to_str().unwrap());
//	println!("is_relative {}", path.is_relative());
//	println!("is_dir {}", path.is_dir());
//	println!("exists {}", path.exists());
//	
//	println!("{}", new_path.to_str().unwrap());
//	println!("is_relative {}", new_path.is_relative());
//	println!("is_dir {}", new_path.is_dir());
//	println!("exists {}", new_path.exists());
//	
	
	
//	std::process::exit(0);
//	 
//	let mut config = Config::read(Config::def_config_path().unwrap()).unwrap();
//	
//	let mut cfg_entry: Option<ConfigEntry> = None;
//	if let Some(e) = config.get_active_config_entry(&String::from("java")).clone() {
//		cfg_entry = Some(e.clone());
//	}
//	
//	cfg_entry.as_mut().unwrap().activate(&Config::def_cmd_path().unwrap());
//	
//	config.set_active(&cfg_entry.unwrap());
	
	
	// TODO: handle errors in here
	let mut config = Config::read().unwrap();
//	
//	let key = "HELLO";
//	env::set_var(key, "WORLD");
//	
//	println!("{:?}", env::var("foo"));
//	println!("{:?}", env::var("HELLO"));
	
	let args:Vec<_> = env::args().collect();
	if args.len() < 2 {
		println!("Not enough parameters supplied. Minimum of 2 parameters required");
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
		} else {
			println!("Unknown argument '{}'", goal);
			help::print_help();
			std::process::exit(2);
		}
	}
}
