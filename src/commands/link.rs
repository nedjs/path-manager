use config::Config;
use config::Link;
use utils;
use std;
use std::ascii::AsciiExt;

pub const DESCRIPTION: &'static str = "Installs a new link, a singular connection from one file to another (or dir)";

pub fn print_usage() {
	// TODO: this help text fields kinda weird, maybe re-write this?
	println!("Usage: pman link <args> \n\
			  \0possible args are:\n\
			  \0   -link,-l <name> <path>\n\
 			  \0              Adds a link to the group, if an existing link exists under the same <name> that\n\
 			  \0              links path will be updated to match the <path> param. If this links <name> collides with any link groups\n\
 			  \0              all actions will be void and the application will exit with a non 0 status code.\n\
			  \0   -unlink,-u <name> \n\
			  \0              Removes a link by name, if no link by that name exists then this operation does nothing.\n\
			  \0              \n\
			  ");
}

pub fn run(mut config: Config, args: &[String]) {
	if args.len() < 1 {
		print_usage();
		std::process::exit(1);
	}
	
	utils::loop_args(args.iter(), |mut it, val| {
			match val.to_ascii_lowercase().as_ref() {
				"-l" | "-link" => {
					let name = it.next().unwrap_or_else(|| {
						println!("Invalid '{}' argument, expected <name> <path> but ran out of arguments", val);
						std::process::exit(1);
					});
					let path = it.next().unwrap_or_else(|| {
						println!("Invalid '{}' argument, expected <name> <path> but ran out of arguments", val);
						std::process::exit(1);
					});
					add_link(&mut config, name, path);
				},
				"-u" | "-unlink" => {
					let name = it.next().unwrap_or_else(|| {
						println!("Invalid '{}' argument, expected <name> <path> but ran out of arguments", val);
						std::process::exit(1);
					});
					remove_link(&mut config, name);
				},
				_ => {
					// unknown argument
					println!("Unexpected or unknown argument '{}'", val);
					std::process::exit(1);
				}
			}
		});
//		
//	let cmd = &args[0];
//	
//	if cmd == "add" {
//		if args.len() < 3 {
//			println!("Too few arguments for add subcommand.");
//			std::process::exit(1);
//		}
//		let name = &args[1];
//		
//		add_link(&mut config, name, &args[2], &args[2..]);
//	} else if cmd == "remove" {
//		if args.len() < 2 {
//			println!("Too few arguments for remove command.");
//			std::process::exit(1);
//		}
//		let name = &args[1];
//		remove_link(&mut config, name);
//	} else {
//		print_usage();
//		std::process::exit(1);
//	}
	
}

fn remove_link(config: &mut Config, name: &String) {
	let exit_status: i32;
	let cmd_dir = config.cmd_dir();
	if let Some(link) = config.get_link(&name) {
		match link.remove_link(&cmd_dir, &cmd_dir) {
			Err(e) => {
				println!("Failed to remove link \"{}\". {}", name, e);
				exit_status = 1;
			},
			Ok(r_path) => {
				println!("Link removed \"{}\" => \"{}\"", name, r_path.to_str().unwrap_or("<unknown>"));
				exit_status = 0;
			}
		}
	} else {
		println!("No link named \"{}\" found", name);
		exit_status = 1;
	}
	
	config.remove_link(name);
	if let Err(e) = config.write() {
		println!("Failed to persist data, link was removed but config couldnt be saved! {}", e);
		std::process::exit(1);
	}
	std::process::exit(exit_status);
}


fn add_link(config: &mut Config, name: &String, path: &String) {
	let cmd_dir = config.cmd_dir();
	
	if config.has_link(&name) {
		// try and remove the old link, only println when failed
		// should always be Some() due to our previous check
		if let Some(link) = config.get_link(&name) {
			if let Err(e) = link.remove_link(&cmd_dir, &cmd_dir) {
				println!("Failed to remove link \"{}\". {}", name, e);
			}
		}
		
		config.remove_link(name); // remove the link from the config
	}
	
	let link = Link::new(name.clone(), path.clone());
	match link.create_link(&cmd_dir, &cmd_dir) {
		Err(e) => {
			println!("Failed to create link, aborting. {}", e);
			std::process::exit(1);
		},
		Ok(r_path) => {
			println!("Link created \"{}\" => \"{}\"", link.name, r_path.to_str().unwrap_or("<unknown>"));
		}
	}
	
	config.add_link(link);
	if let Err(e) = config.write() {
		println!("Failed to persist data, link was created but config couldnt be saved! {}", e);
		std::process::exit(1);
	}
}