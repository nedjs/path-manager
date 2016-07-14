use std;
use std::io::prelude::*;

/// prompts for user input and returns value into Option result
pub fn prompt(question: &String, default: &String) -> Option<String> {
	print!("{}", question);
	std::io::stdout().flush().ok();
	
	let mut buff = String::new();
	std::io::stdin().read_line(&mut buff).expect("Did not enter a correct string"); // i wonder how they can not enter a correct string.. I think i pulled this off a tutorial
	buff = String::from(buff.trim()); // trim to remove newline chars
	
	if buff == "" {
		if default == "" {
			return None;
		} else {
			return Some(default.clone());
		}
	}
	return Some(buff);
}

/// prompts for user input and places value into mutable parameter, If value is empty then None is returned.
pub fn prompt_mut(question: &String, line: &mut String, default: &String) -> Option<()> {
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
