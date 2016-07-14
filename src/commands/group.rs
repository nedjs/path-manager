use utils;
use config::{Config, LinkGroup, Link};
use std;
use std::path::PathBuf;
use std::ascii::AsciiExt;

pub const DESCRIPTION: &'static str = "Adds, removes and modifies link groups.";

pub fn print_usage() {
	println!("Usage: pman group [name] [priority] <args>\n\
			 \0  name     (required) Name of configuration group to be be matched.\n\
			 \0  priority (required) If not set you will be prompted to choose what group you wish to configure\n\
			 \0           however if set then the a group closest to but no less than this param will be selected,\n\
			 \0           eg: group.priority>=priority. For the highest priority pass -1, for the lowest pass 0.\n\
			 \0possible args are:\n\
			 \0  -exact,-e\n\
			 \0           Forces group matching to require an exact match on the priority instead of best selection.\n\
			 \n\
			 \0  -add\n\
			 \0           Adds a new link group if it doesnt exist with the <priority>, you will be prompted for\n\
			 \0           confirmation if a group already exists at that same priority unless the -force param is present.\n\
			 \0           Additionally using -add will cause priority matching to be exact instead of closest.\n\
			 \0  -remove\n\
			 \0           Removes the group entirely, if this option has the highest priority. If this group was\n\
			 \0           active the highest priority group will be automatically activated.\n\
			 \n\
			 \0  -dir,-d <path>\n\
			 \0           The directory to set the new group to, not required to be specified but all link <path>'s\n\
			 \0           are relative to this directory. You should also keep in mind that relative link paths are\n\
			 \0           stored as such and wont need updating if you change the directory.\n\
			 \n\
			 \0  -link,-l <name> <path>\n\
			 \0           Adds a link to the group, if an existing link exists under the same <name> that\n\
			 \0           links path will be updated to match the <path> param.\n\
			 \0  -unlink,-u <name>\n\
			 \0           Removes a link from the group.\n\
			 \n\
			 \0  -force,-f\n\
			 \0           Operates on the actions quietly and does not prompt for any confirmations.\n\
			 \0           The application will exit with a non 0 status code if:\n\
			 \0            - the [priority] argument is not present\n\
			 \0            - no matching group could be matched\n\
			 \0            - the group failed to be removed for other unspecfied reasons.\n\
			 ");
}


pub fn run(mut config: &mut Config, args: &[String]) {
	if args.len() < 2 {
		println!("Too few arguments for group command.");
		print_usage();
	} else {
		
		let mut args_itr = args.iter();
		
		// we know name and priority are there cause we did the check above. Another way to write this is to unwrap_or_else, but i dont think that adds much readability
		let name = args_itr.next().unwrap();
		let priority: i64 = args_itr.next().unwrap().parse::<i64>().unwrap_or_else(|_| {
									println!("Invalid priority '{}', expected number.", args[1]);
									std::process::exit(1);
							}).clone();
		let mut force = false;
		let mut exact = false;
		let mut remove = false;
		let mut add = false;
		let mut new_dir_opt: Option<String> = None;
		let mut new_links = Vec::<(String, String)>::new();
		let mut rem_links = Vec::<String>::new();
		
		utils::loop_args(args_itr, |mut it, val| {
				match val.to_ascii_lowercase().as_ref() {
					"-f" | "-force" => {
						force = true;
					},
					"-e" | "-exact" => {
						exact = true;
					},
					"-remove" => {
						remove = true;
					},
					"-a" | "-add" => {
						add = true;
						exact = true; // exact matching
					},
					"-d" | "-dir" => {
						new_dir_opt = Some(it.next().unwrap_or_else(|| {
							println!("Invalid '{}' argument, expected <path> but ran out of arguments", val);
							std::process::exit(1);
						}).clone());
					},
					"-l" | "-link" => {
						let name = it.next().unwrap_or_else(|| {
							println!("Invalid '{}' argument, expected <name> <path> but ran out of arguments", val);
							std::process::exit(1);
						});
						let path = it.next().unwrap_or_else(|| {
							println!("Invalid '{}' argument, expected <name> <path> but ran out of arguments", val);
							std::process::exit(1);
						});
						new_links.push((name.to_owned(), path.to_owned()));
					},
					"-u" | "-unlink" => {
						let name = it.next().unwrap_or_else(|| {
							println!("Invalid '{}' argument, expected <name> but ran out of arguments", val);
							std::process::exit(1);
						});
						rem_links.push(name.to_owned());
					},
					_ => {
						// unknown argument
						println!("Unexpected or unknown argument '{}'", val);
						std::process::exit(1);
					}
				}
			});
		// skip_n
		let mut link_group_opt: Option<LinkGroup>;
		if exact {
			link_group_opt = config.select_group(&name, priority as u64);
		} else {
			if priority < 0 {
				link_group_opt = config.select_highest_group(&name);
			} else {
				link_group_opt = config.select_closest_group(&name, priority as u64);
			}
		}
		
		let did_create; // yuk, i dont like this
		if add {
			// uhoh existing group exists, prompt for removal?
			if !force && link_group_opt.is_some() {
				let link_group = link_group_opt.as_ref().unwrap();
				let prompt_res = utils::prompt(&format!("You may press enter to say no.\nReplace existing group {}/{}?\n(y/n): ", link_group.name, link_group.priority), &String::from("n"));
				if let Some(v) = prompt_res {
					if !v.eq_ignore_ascii_case("y") {
						// dont continue, just exit
						std::process::exit(0);
					}
				}
			}
			// create  a new group!
			link_group_opt = Some(LinkGroup::new(name.to_owned(), priority as u64));
			did_create = true;
		} else {
			did_create = false;
		}
		
		match link_group_opt {
			None => {
				println!("No matching link group found for {}/{}", name, priority);
				std::process::exit(1);
			},
			Some(mut link_group) => {
				if did_create {
					println!("Created new group {}/{}", name, priority);
				} else {
					println!("Matched to group {}/{}", link_group.name, link_group.priority);
				}
				if remove {
					if !force {
						// prompt for confirmation
						if let Some(v) = utils::prompt(&format!("You may press enter to say no.\nRemove group {}/{}?\n(y/n): ", link_group.name, link_group.priority), &String::from("n")) {
							if !v.eq_ignore_ascii_case("y") {
								// dont continue, just exit
								std::process::exit(0);
							}
						}
					}
					
					
					if config.is_active(&link_group) {
						// deactivate
						link_group.deactivate(&config.cmd_dir());
						println!("Group {}-{} was deactivated", link_group.name, link_group.priority);
					}
					config.remove_config_entry(&link_group.name, link_group.priority);
					
					if let Some(new_group) = config.select_highest_group(&name) {
						new_group.activate(&config.cmd_dir());
						println!("Group {}-{} was activated in response", new_group.name, new_group.priority);
					}
					println!("Group {}-{} was deleted", link_group.name, link_group.priority);
				} else {
					// not deleted, just modifying or adding
					let cmd_dir = config.cmd_dir();
					let cfg_source_path = PathBuf::from(&link_group.base_path);
					// do removing of links
					for lnk_name in rem_links.iter() {
						if let Some(lnk) = link_group.get_link(lnk_name) {
							// try and remove it, remove it right away instead of later
							if let Err(e) = lnk.remove_link(&cmd_dir, &cfg_source_path) {
								println!("Error while removing link on file system \"{}\", {}.", lnk.name, e);
							}
							println!("Removing link \"{}\"", lnk.name);
							link_group.remove_link(lnk_name);
						} else {
							// no link found by that name, i dont think we want to fail or anything
						}
					}
					
					// do adding of links
					// TODO: learn what the fuck are these 'ref' things needed for in this loop, & doesnt work. I had thought ref==& but i guess im wrong
					for &(ref lnk_name, ref lnk_path) in new_links.iter() {
						println!("Adding link \"{}\" => \"{}\"", lnk_name, lnk_path);
						let new_link = Link::new(lnk_name.to_owned(), lnk_path.to_owned());
						link_group.add_link(new_link);
					}
					
					// update directory
					if let Some(new_dir) = new_dir_opt {
						println!("Setting base directory \"{}\"", new_dir);
						link_group.set_base_path(new_dir);
					}
					
					if config.is_active(&link_group) {
						println!("Group is active, refreshing links to make everything up to date.");
						link_group.activate(&cmd_dir);
					}
				
					// TODO: I kinda stink at these borrows, make it so i am mutating the underlying entry which has already been added, or make a nice method which replaces the entry
					// remove before insert, therefore there should be no problems.
					config.remove_config_entry(&link_group.name, link_group.priority);
					if !config.insert_config_entry(link_group) {
						panic!("Could not insert config entry due to duplicate entry. You should never see this message");
					}
				}
			}
		}
		
		// finally save the configuration
		if let Err(e) = config.write() {
			println!("Failed to write to configuration file, {}. Group was not removed", e);
		}
		
	}
}



