use config::Config;
use config::LinkGroup;
use std;
use std::io::prelude::*;


pub const DESCRIPTION: &'static str = "Updates the current installation for a configuration group";

pub fn print_usage() {
	println!("Usage: pman swap [name] [priority]\n\
			 \0   name       (required) Name of configuration group\n\
			 \0   priority   (optional) If not set you will be prompted to choose what config to activate\n\
			 \0              however if set then the a config closest to but no less than this param will be selected,\n\
			 \0              eg: cfg.priority>=priority. For the highest priority pass -1, for the lowest pass 0.");
}

pub fn run(mut config: Config, args: &[String]) {
	
	if args.len() == 0 {
		println!("Expected additional argument [name]");
		print_usage();
	} else {
		
		let cfg_to_activate: Option<LinkGroup>;
		if args.len() == 1 {
			cfg_to_activate = prompt_swap(&config, &args[0]);
			if cfg_to_activate.is_none() {
					println!("Invalid selection");
					std::process::exit(1);
			}
		} else {
			// check for 2nd param, priority
			let sel_p = args[1].parse::<i64>().unwrap_or_else(|_| {
					println!("Invalid priority '{}', expected number.", args[1]);
					std::process::exit(1);
			});
			
			let match_result;
			if sel_p < 0 {
				match_result = config.select_highest_group(&args[0]);
			} else {
				match_result = config.select_closest_group(&args[0], sel_p as u64);
			}
			
			match match_result {
				Some(found_cfg) => {
					cfg_to_activate = Some(found_cfg.clone());
				},
				None => {
					println!("No matching configuration found");
					std::process::exit(1);
				}
			}
			
		}
		
		if cfg_to_activate.is_some() {
			{ // umm, borrow config... these fucking borrows are going to be the death of me
				// deactivate our current config if it exists
				let curr_cfg = config.active_config_entry(&args[0]);
				if curr_cfg.is_some() { // deactivate the current config
					curr_cfg.unwrap().deactivate(&config.cmd_dir());
				}
			}
			
			// activate our new config
			let new_cfg = cfg_to_activate.unwrap();
			println!("Swapping to {} - {}", new_cfg.priority, new_cfg.base_path);
			
//			set_active_config(&mut config, new_cfg);
//			let mut m_cfg = &mut config;
			config.set_active(&new_cfg.name, &new_cfg.priority);
			new_cfg.activate(&config.cmd_dir());
			
			if let Err(e) = config.write() {
				println!("Failed to persist data. {}", e);
			}
		}
	}
}

fn prompt_swap(config: &Config, name: &String) -> Option<LinkGroup> {
	let cfg_vec:&Vec<LinkGroup> = config.config_entrys_by_name(&name).unwrap_or_else(||{
		println!("No configs found for '{}'", name);
		std::process::exit(1);
	});
	// prompt swap
	println!("Choose from the selections below. Asterik(*) is the current active configuration.");
	for i in 0..cfg_vec.len() {
		let entry = cfg_vec.get(i).unwrap();
		let mut ast = " ";
		if config.is_active(entry) { ast = "*" }
		println!("{}  {}. {} - {}",ast, (i+1), entry.priority, entry.base_path)
	}
	
	print!("Enter number to swap to or press enter to do nothing: ");
	std::io::stdout().flush().ok();
	let mut line=String::new();
	std::io::stdin().read_line(&mut line).expect("Did not enter a correct string");
	line = String::from(line.trim()); // trim to remove newline chars
	
	if line == "" { // no problem, they dont wanna do anything
		std::process::exit(0);
	}
	
	// parse their selection
	let selection_num = line.parse::<usize>().unwrap_or_else(|_| {
			println!("Invalid selection");
			std::process::exit(1);
	});
	
	if let Some(c) = cfg_vec.get(selection_num-1) {
		return Some(c.clone());
	}
	return None;
}



