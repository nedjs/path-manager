use utils::config::Config;
use std;
use std::ascii::AsciiExt;
use std::io::prelude::*;

pub const DESCRIPTION: &'static str = "Utility to help configure pman. Expecially useful when running for the first time";

pub fn print_usage() {
	println!("Usage: pman configure <options>\n\
			 \0possible options are:\n\
			 \0   -d,-dir <val>  Sets the directory to use for all command links. May want to run 'pman rebuild' afterwards.\n\
			 \0                  Also may be used when running pman for the first time to configure the directory.\n\
			 \0   -p,-prompt     Go through a series of prompts to update configuration.");
}


pub fn run(config: Option<Config>, args: &[String]) {
	if args.len() == 0 {
		println!("At least 1 option is required");
		print_usage();
	} else {
		let mut dir = String::new();
		let mut do_prompt = false;
		let mut updated_message = String::new();
		
		let mut skip_n = 0; // used to skip arguments after one was already read
		for i in 0..args.len() {
			if skip_n == 0 {
				let val = &args[i];
				match val.to_ascii_lowercase().as_ref() {
					"-p" | "-prompt" => do_prompt = true,
					"-d" | "-dir" => {
						if i+1 >= args.len() {
							println!("Invalid syntax, option -dir expects value after it");
							std::process::exit(1);
						} else {
							dir = args[i+1].clone();
							skip_n = 1; // skip next argument
							if updated_message.len() > 0 {
								updated_message.push_str(&"\n");
							}
							updated_message.push_str(&format!("Command directory updated to: '{}'", dir));
						}
					},
					_ => {
						// unknown argument... ignore i guess?
					}
				}
			} else {
				skip_n = skip_n - 1; // reduce skip
			}
		}
		
		let mut new_cfg = config.unwrap_or(Config::new(String::new())); // get old config or create new one
		if dir != "" { // if we defined our new directory set it to the new config
			new_cfg.set_cmd_dir_str(dir.clone());
		}
		
		// check if they wanna do prompting stuff
		if do_prompt {
			do_config_prompt(&mut new_cfg);
		} else {
			// no prompt, so instead print out the update message generated
			println!("{}", updated_message);
		}
		
		// persist config
		if let Err(e) = new_cfg.write() {
			println!("Failed to write configuration, {}", e);
		}
	}
}


fn do_config_prompt(config: &mut Config) {
	
	let question = format!("Currently set to ({}): ", config.cmd_dir_str());
	
	println!("Enter new command directory. This may be a relative path, if so it is relative to this binary.\n\
			  You may press enter to leave the current value unchanged.");
	
	let mut new_dir = String::new();
	while let None = prompt(&question, &mut new_dir, &config.cmd_dir_str()) {
		println!("You must enter a value for the command directory.");
	}
	config.set_cmd_dir_str(new_dir);
}

fn prompt(question: &String, line: &mut String, default: &String) -> Option<()> {
	print!("{}", question);
	std::io::stdout().flush().ok();
	line.clear();
	
	let mut buff = String::new();
	std::io::stdin().read_line(&mut buff).expect("Did not enter a correct string"); // i wonder how they can not enter a correct string.. I think i pulled this off a tutorial
	buff = String::from(buff.trim()); // trim to remove newline chars
	
	// i cant figure out how to replace the underlying string structure in regards to mutablity, this will probably work just fine
	line.clear();
	line.push_str(&buff);
	
	if line == "" {
		if default == "" {
			return None;
		} else {
			// i cant figure out how to replace the underlying string structure in regards to mutablity, this will probably work just fine
			line.clear();
			line.push_str(&default.clone());
			return Some(());
		}
	}
	return Some(());
}



