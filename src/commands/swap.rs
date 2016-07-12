use utils;
use commands::help;
use utils::config::Config;
use utils::config::ConfigEntry;
use std;
use std::io::prelude::*;  

pub fn swap(mut config: &mut Config, args: &[String]) {
	
	if args.len() == 0 {
		println!("Expected additional argument <name>");
		help::print_help();
	} else {
		
		let cfg_to_activate: Option<ConfigEntry>;
		if args.len() == 1 {
			{			
				cfg_to_activate = prompt_swap(&config, &args[0]);
				if cfg_to_activate.is_none() {
						println!("Invalid selection");
						std::process::exit(1);
				}
			}
		} else {
			let cfg_vec:&Vec<ConfigEntry> = config.config_entry(&args[0]).unwrap_or_else(||{
				println!("No configs found for '{}'", args[0]);
				std::process::exit(1);
			});
			// check for 2nd param priority
			
			// parse priority as signed then convert to unsigned. This will cause negative numbers to be very large. (eg: -1 becomes max value)
			// this way someone can choose priority -1 to be the highest and 0 to be the lowest
			let sel_p = args[1].parse::<i64>().unwrap_or_else(|e| {
					println!("Invalid priority '{}', expected number.", args[1]);
					std::process::exit(1);
			}) as u64;
			
			let mut m_ix: usize = 0;
			for i in 0..cfg_vec.len() {
				let p = cfg_vec.get(i).unwrap().priority;
				// choose config if none is chosen yet 
				// or if the chosen config has a higher priority than the current selected one 
				// (narrow down to closest to >= to sel)
				if p>=sel_p && cfg_vec.get(m_ix).unwrap().priority>p {
					m_ix = i;
				}
			}
			
			if m_ix > cfg_vec.len() {
					println!("No matching configuration found");
					std::process::exit(1);
			}
			cfg_to_activate = Some(cfg_vec.get(m_ix).unwrap().clone());
			
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
			config.set_active(&new_cfg.name, &new_cfg.base_path);
			new_cfg.activate(&config.cmd_dir());
			
			if let Err(e) = config.write() {
				println!("Failed to persist data. {}", e);
			}
		}
	}
}

fn prompt_swap(config: &Config, name: &String) -> Option<ConfigEntry> {
	let cfg_vec:&Vec<ConfigEntry> = config.config_entry(&name).unwrap_or_else(||{
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
	let selection_num = line.parse::<usize>().unwrap_or_else(|e| {
			println!("Invalid selection");
			std::process::exit(1);
	});
	
	if let Some(c) = cfg_vec.get(selection_num-1) {
		return Some(c.clone());
	}
	return None;
}


