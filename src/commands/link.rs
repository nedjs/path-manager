use utils;
use utils::config::Config;
use utils::config::Link;
use std;

pub fn link(config: &mut utils::config::Config, args: &[String]) {
	if args.len() < 1 {
		print_list_help();
		std::process::exit(1);
	}
	let cmd = &args[0];
	
	if cmd == "add" {
		if args.len() < 3 {
			println!("Too few arguments for add command. Expected:\n\0 pack link add <name> <path> [-f]");
			std::process::exit(1);
		}
		let name = &args[1];
		
		let (goal_args, addl_args) = args.split_at(2);
		add_link(config, name, &args[2], addl_args);
	} else if cmd == "remove" {
		if args.len() < 2 {
			println!("Too few arguments for remove command. Expected:\n\0 pack link remove <name>");
			std::process::exit(1);
		}
		let name = &args[1];
		remove_link(config, name);
	} else {
		print_list_help();
		std::process::exit(1);
	}
	
}

fn print_list_help() {
	// using \0 to stop the whitespace trimmer, dont think it will mess up consoles?!
	println!("Invalid command. Expected:\n\
		\0 pack link add <name> <path> [-f]\n\
		\0 pack link remove <name>");
}

fn remove_link(config: &mut utils::config::Config, name: &String) {
	let cmd_dir = config.cmd_dir();
	if let Some(link) = config.get_link(&name) {
		match link.remove_link(&cmd_dir, &cmd_dir) {
			Err(e) => {
				println!("Failed to remove link {:?}. {}", name, e);
			},
			Ok(r_path) => {
				println!("Link removed {:?} => {:?}", name, r_path);
			}
		}
	} else {
		println!("No link named {:?} found", name);
	}
	
	config.remove_link(name);
	if let Err(e) = config.write() {
		println!("Failed to persist data, link was removed but config couldnt be saved! {}", e);
		std::process::exit(1);
	}
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