use utils;
use utils::config::Link;
use std;

pub const DESCRIPTION: &'static str = "Installs a new link, a singular connection from one file to another (or dir)";

pub fn print_usage() {
	// TODO: this help text fields kinda weird, maybe re-write this?
	println!("Usage: pman link [add|remove] \n\
			  \0   add - Adds a new command, will create either a batch file or symlink depending on destination type.\n\
			  \0   remove - Removes a command link, deletes the symlink or batch file and the configuration entry for the link.\n\
			  \n\
			  Subcommand specific syntax:\n\
			  \0   add [name] [dest] [-options]\n\
			  \0       name       (required) Name of link, must be unique to other links and not be used in any active configurations.\n\
			  \0       dest       (required) Where the link points to, can be a directory or executable.\n\
			  \0       -options include:\n\
			  \0           -f      Force create, will override any existing link that exists\n\
			  \n\
			  \0   remove [name]\n\
			  \0       name       (required) Name of link.");
}

pub fn run(mut config: utils::config::Config, args: &[String]) {
	if args.len() < 1 {
		print_usage();
		std::process::exit(1);
	}
	let cmd = &args[0];
	
	if cmd == "add" {
		if args.len() < 3 {
			println!("Too few arguments for add subcommand. Expected:\n\0 pack link add <name> <path> [-f]");
			std::process::exit(1);
		}
		let name = &args[1];
		
		add_link(&mut config, name, &args[2], &args[2..]);
	} else if cmd == "remove" {
		if args.len() < 2 {
			println!("Too few arguments for remove command. Expected:\n\0 pack link remove <name>");
			std::process::exit(1);
		}
		let name = &args[1];
		remove_link(&mut config, name);
	} else {
		print_usage();
		std::process::exit(1);
	}
	
}

fn remove_link(config: &mut utils::config::Config, name: &String) {
	let exit_status: i32;
	let cmd_dir = config.cmd_dir();
	if let Some(link) = config.get_link(&name) {
		match link.remove_link(&cmd_dir, &cmd_dir) {
			Err(e) => {
				println!("Failed to remove link {:?}. {}", name, e);
				exit_status = 1;
			},
			Ok(r_path) => {
				println!("Link removed {:?} => {:?}", name, r_path);
				exit_status = 0;
			}
		}
	} else {
		println!("No link named {:?} found", name);
		exit_status = 1;
	}
	
	config.remove_link(name);
	if let Err(e) = config.write() {
		println!("Failed to persist data, link was removed but config couldnt be saved! {}", e);
		std::process::exit(1);
	}
	std::process::exit(exit_status);
}


fn add_link(config: &mut utils::config::Config, name: &String, path: &String, addl_args: &[String]) {
	let mut forced = false;
	let cmd_dir = config.cmd_dir();
	
	if addl_args.contains(&String::from("-f")) {
		forced = true;
	}
	
	if config.has_link(&name) {
		if forced {
			// try and remove the old link, only println when failed
			// should always be Some() due to our previous check
			if let Some(link) = config.get_link(&name) {
				if let Err(e) = link.remove_link(&cmd_dir, &cmd_dir) {
					println!("Failed to remove link {:?}. {}", name, e);
				}
			}
			
			config.remove_link(name); // remove the link from the config
		} else {
			println!("Link {:?} already exists, use option -f to override it or remove it before creating it", name);
			std::process::exit(1);
		}
	}
	
	let link = Link::new(name.clone(), path.clone());
	match link.create_link(&cmd_dir, &cmd_dir) {
		Err(e) => {
			println!("Failed to create link, aborting. {}", e);
			std::process::exit(1);
		},
		Ok(r_path) => {
			println!("Link created {:?} => {:?}", link.name, r_path);
		}
	}
	
	config.add_link(link);
	if let Err(e) = config.write() {
		println!("Failed to persist data, link was created but config couldnt be saved! {}", e);
		std::process::exit(1);
	}
}