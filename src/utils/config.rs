
use std::env;
use std;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::path::Path;
use std::io::BufReader;
use std::io;
use std::collections::HashMap;
use std::os::windows::fs::symlink_dir;

use std::io::prelude::*;

/// the configuration for the entire path manager, saves to file
pub struct Config {
	config_map: HashMap<String, Vec<ConfigEntry>>,
	active_configs: HashMap<String, String>, // (entry.name, entry.path)
	links: Vec<Link>,
	cmd_dir_str: String
}

impl Config {
	
	pub fn new(cmd_dir: String) -> Config {
		return Config {
				active_configs: HashMap::new(),
				config_map: HashMap::new(),
				links: Vec::new(),
				cmd_dir_str: String::from("")
		};
	}
	
	/// gets the command directory as a string literal, may not be a full absolute path 
	pub fn cmd_dir_str(&self) -> &String {
		return &self.cmd_dir_str;
	}
	/// set command directory as string literal, may be a relative path
	pub fn set_cmd_dir_str(&mut self, val: String) {
		self.cmd_dir_str = val;
	}
	
	/// the command directory as absolute path. 
	pub fn cmd_dir(&self) -> PathBuf {
		let mut cmd_dir = PathBuf::from(&self.cmd_dir_str);
		
		// check if it needs to be relative
		if cmd_dir.is_relative() {
			let mut exe_dir = env::current_exe().unwrap();
			exe_dir.pop(); // remove the executable name
			cmd_dir = exe_dir.join(cmd_dir);
		}
		
		return cmd_dir;
	}
	
	/// configuration entries by string name
	pub fn config_entrys_by_name(&self, name: &String) -> Option<&Vec<ConfigEntry>> {
		return self.config_map.get(name);
	}
	
	/// the Hashmap of all configuration entries, these are held by name -> vec
	pub fn config_map(&self) -> &HashMap<String, Vec<ConfigEntry>> {
		return &self.config_map;
	}
	
	/// all standalone links
	pub fn links(&self) -> &Vec<Link> {
		return &self.links;
	}
	
	/// all active configurations
	pub fn active_configs(&self) -> &HashMap<String, String> {
		return &self.active_configs;
	}
	
	/// retrives the current ConfigEntry for the given name
	pub fn active_config_entry(&self, name: &String) -> Option<&ConfigEntry> {
		let cfg_vec = self.config_map.get(name);
		if cfg_vec.is_none() {
			return None;
		}
		
		for cfg_entry in cfg_vec.unwrap().iter() {
			if self.is_active(cfg_entry) {
				return Some(cfg_entry);
			}
		}
		
		return None;
	}
	
	/// checks if the ConfigEntry is seen as active
	pub fn is_active(&self, cfg_entry: &ConfigEntry) -> bool {
		self.config_map.keys();
		match self.active_configs.get(&cfg_entry.name) {
			Some(a) => return cfg_entry.base_path.eq(a),
			None => return false
		}
	}
	
	/// changes the current active configuration
	pub fn set_active(&mut self, name: &String, base_path: &String) {
		self.active_configs.insert(name.clone(), base_path.clone());
	}
	pub fn add_link(&mut self, link: Link) {
		self.links.push(link);
	}
	pub fn has_link(&self, name: &String) -> bool {
		for link in &self.links {
			if link.name.eq(name) {
				return true;
			}
		}
		
		return false;
	}
	pub fn remove_link(&mut self, name: &String) -> bool {
		for i in 0..self.links.len() {
			if name.eq(&self.links[i].name) {
				&self.links.remove(i);
				return true;
			}
		}
		return false;
	}
	
	pub fn get_link(&self, name: &String) -> Option<&Link> {
		for link in &self.links {
			if link.name.eq(name) {
				return Some(&link);
			}
		}
		
		return None;
	}
	
	pub fn cfg_path() -> Result<PathBuf, io::Error> {
		let mut config_path = env::current_exe().unwrap();
		let mut config_name: String = String::from(".");
		// i think i need to keep this as a OsString, but i cant fucking figure out how to prepend a "." to it... so fuck it, convert it to UTF-8 string
		config_name.push_str(config_path.file_stem().unwrap().to_str().unwrap()); 
		
		config_path.pop(); // remove the executable name
		config_path.push(Path::new(&config_name)); // add in config name which is ".{exe_name}"
		
		return Ok(config_path);
	}
	
	pub fn write(&self) -> io::Result<()> {
		
		let config_path = try!(Config::cfg_path());
		
		let mut file = try!(File::create(config_path));
		try!(file.write_all(self.cmd_dir_str.as_bytes()));
		try!(file.write_all(b"\n"));// new line
		
		// active configs
		for (name, path) in &self.active_configs {
			try!(file.write_all(format!("{}\n{}\n", name, path).as_bytes()));
		}
		try!(file.write_all(b"\n"));// new line
		
		// standalone links 
		for link in &self.links {
			try!(file.write_all(format!("{}\n{}\n", link.name, link.path).as_bytes()));
		}
		try!(file.write_all(b"\n"));// new line
		
		// configs
		for (name, cfg_entries) in &self.config_map {
			try!(file.write_all(format!("{}\n", name).as_bytes()));
			for cfg_entry in cfg_entries {
				try!(file.write_all(format!("{}\n{}\n", cfg_entry.base_path, cfg_entry.priority).as_bytes()));
				for link in &cfg_entry.links {
					try!(file.write_all(format!("{}\n{}\n", link.name, link.path).as_bytes()));
				}
				try!(file.write_all(b"\n"));// new line
			}
			try!(file.write_all(b"\n"));// 1 more newline between entries
		}
		
		try!(file.sync_all());
		return Ok(());
	}
	
	/**
	Reads the config file and returns a config object
	*/
	pub fn read() -> Result<Option<Config>, io::Error> {
		
		let config_path = try!(Config::cfg_path());
		
		let mut active_configs = HashMap::new();
		let mut links = Vec::new();
		let mut configs = HashMap::new(); // config objects which will be returned
		let mut cmd_dir_str;
		
		if config_path.exists() {
			let file = try!(File::open(config_path));
			let mut reader = BufReader::new(file);
			let mut line: String = String::new();
			if try!(reader.read_line(&mut line)) <= 0 {
				println!("Unexpected EOF while reading config");
				std::process::exit(1);
			}
			cmd_dir_str = String::from(line.trim());
			
			try!(Config::_read_active_configs(&mut active_configs, &mut reader));
			try!(Config::_read_links(&mut links, &mut reader));
			try!(Config::_read_configs(&mut configs, &mut reader));
		} else {
			return Ok(None);
		}
		
		let config = Config {
				active_configs: active_configs,
				config_map: configs,
				links: links,
				cmd_dir_str: cmd_dir_str
		};
		
		
		// make the commands directory if it doesnt exist
		let cmd_dir = config.cmd_dir();
		if !cmd_dir.exists() {
			try!(fs::create_dir_all(&cmd_dir));
		}
		
		
		return Ok(Some(config));
	}
	
	

	fn _read_active_configs(map: &mut HashMap<String, String>, reader: &mut BufReader<File>) -> io::Result<()> {
		let mut name = String::new();
		
		if let Err(e) = Config::_read_to_empty_line(reader, &mut |line| {
			if name == "" {
				name = line.clone();
			} else {
				// name is set, then we have path
				map.insert(name.clone(), line.clone());
			}
			return Ok(());
		}) {
			// error reading config
			println!("Error occured while reading active configuration section. {}", e);
			std::process::exit(1);
		}
		
		return Ok(());
	}
	
	fn _read_links(links: &mut Vec<Link>, reader: &mut BufReader<File>) -> io::Result<()> {
		
		let mut name = String::new();
		
		if let Err(e) = Config::_read_to_empty_line(reader, &mut |line| {
				if name == "" {
					name = line.clone();
				} else {
					// name is set, then we have path
					links.push(Link {
						name: name.clone(), 
						path: line.clone()
					});
					name.clear();
				}
				return Ok(());
			}) {
			// error reading config
			println!("Error occured while reading active links section. {}", e);
			std::process::exit(1);
		}
		
		return Ok(());
	}
	
	/// reads all configuration entries from the reader, expected to be at the line before the next entries
	fn _read_configs(configs: &mut HashMap<String, Vec<ConfigEntry>>, reader: &mut BufReader<File>) -> io::Result<()> {
		
		let mut name = String::new();
		while try!(reader.read_line(&mut name)) > 0 {
			name = String::from(name.trim()); // trim out newlines
			
			let mut cfg_vec = Vec::new();
			while let Some(c) = try!(Config::_read_config(name.clone(), reader)) {
		//		println!("Add config {}", c.base_path);
				cfg_vec.push(c);
			}
			// sort by priority, we want highest first
			cfg_vec.sort_by(|a, b| b.priority.cmp(&a.priority));
			configs.insert(name.clone(), cfg_vec);
			
			name.clear();
		}
		
		return Ok(());
	}
	
	
	/// Reads the config from a buffered reader
	fn _read_config(name: String, reader: &mut BufReader<File>) -> Result<Option<ConfigEntry>, io::Error> {
		let mut line = String::new();
	
		let mut base_path = String::new();
		let mut priority = 0;
		let mut links = Vec::new();
		let mut link_name = String::new();
		
		let mut line_num = 0;
		while try!(reader.read_line(&mut line)) > 0 {
			line = String::from(line.trim()); // trim to remove newline chars
			
			// new line signals end of config
			if line == "" {
				break;
			}
	//		println!("{:?}", line);
			if line_num == 0 {
				base_path = line.clone();
			} else if line_num == 1 {
				priority = line.parse::<u64>().unwrap_or_else(|e| {
						panic!("Invalid priority {:?} while parsing configuration {:?}.\n{:?}", line, name, e)
				});
			} else {
				
				if line_num%2 == 0 {
					// is link name declaration
					link_name = line.clone();
				} else {
					links.push(Link {
						name: link_name.clone(),
						path: line.clone()
					});
				}
			}
			
		    line.clear();
		    line_num+=1;
		}
		
		if base_path == "" {
			return Ok(None);
		} else {
			return Ok(Some(ConfigEntry {
				name: name,
				base_path: base_path,
				priority: priority,
				links: links
			}));
		}
	}
	
	/// reads line from reader until a blank line is read, where it will stop parsing.
	/// used in our configuration
	fn _read_to_empty_line<F>(reader: &mut BufReader<File>, acc_fn: &mut F) -> io::Result<()>
		where F: FnMut(&String)-> io::Result<()> {
			
		let mut line = String::new();
		while try!(reader.read_line(&mut line)) > 0 {
			line = String::from(line.trim()); // trim to remove newline chars
			
			// new line signals end of config
			if line == "" {
				break;
			}
			
			try!(acc_fn(&line));
			
			line.clear();
		}
		
		return Ok(());
	}
	
}


#[derive(Clone, Debug)]
pub struct ConfigEntry {
	pub name: String,
	pub base_path: String,
	pub links: Vec<Link>,
	pub priority: u64
}

impl ConfigEntry {
	
	pub fn activate(&self, in_dir: &PathBuf) {
		let source_path = PathBuf::from(&self.base_path);
		for link in self.links.iter() {
			if let Err(e) = link.create_link(&in_dir, &source_path) {
				println!("Unable to create link: {:?}, {}", link.name, e)
			}
		}
	}
	
	pub fn deactivate(&self, in_dir: &PathBuf) {
		let source_path = PathBuf::from(&self.base_path);
		for link in self.links.iter() {
			if let Err(e) = link.remove_link(&in_dir, &source_path) {
				println!("Unable to remove link: {:?}, {}", link.name, e)
			}
		}
	}
	
	// i dont wanna delete this quite yet, good reference material
//	
//	fn iter_link_paths<F>(&self, in_dir: &PathBuf, accept_fn: F) 
//		where F: Fn(&PathBuf, &PathBuf) {
//		
//		let cfg_base_path = PathBuf::from(&self.base_path);
//		for link in self.links.iter() {
//			let mut source_path = PathBuf::from(&link.path);
//			let is_file = !source_path.is_dir(); // for some reason is_file is always returning false... so use !is_dir
//			
//			// relativize the links path to the base directory (if its relative)
//			if source_path.is_relative() {
//				source_path = cfg_base_path.join(&source_path);
//			}
//			
//			// if the destination is a file then our source is a bat file
//			// else if the the destination is a dir then our source is a sym link
//			if is_file {
//				// make the link path of {in_dir}/{link.name}.bat
//				let mut link_name: String = link.name.clone();
//				link_name.push_str(".bat");
//				
//				let link_path = in_dir.join(&link_name);
//				
//				accept_fn(&link_path, &source_path);
//			} else {
//				let link_path: PathBuf = in_dir.join(&link.name);
//				accept_fn(&link_path, &source_path);
//			}
//		}
//	}
	
//	fn create_command_link(&self, link_path: &PathBuf, source_path: &PathBuf) -> io::Result<()> {
////		println!("Create cmd link {:?} -> {:?}", link_path,  source_path.to_str());
//		let mut file = try!(File::create(&link_path));
//		try!(file.write_all(format!("@{:?} %*", source_path.to_str().unwrap()).as_bytes()));
//		
//		return Ok(());
//	}
//	
//	fn create_sym_link(&self, link_path: &PathBuf, source_path: &PathBuf) -> io::Result<()> {
////		println!("Create SYM link {:?} -> {:?}", link_path,  source_path.to_str());
//		if link_path.exists() {
//			try!(fs::remove_dir(&link_path));
//		}
//		try!(symlink_dir(source_path, link_path));
//		
//		return Ok(());
//	}
}

#[derive(Clone, Debug)]
pub struct Link {
	pub name: String,
	pub path: String
}

impl Link {
	
	pub fn new(name: String, path: String) -> Link {
		return Link {
				name: name,
				path: path
		};
	}
	
	/// removes this link in the in_dir directory. Some links are relative, they are relativized using source_rel_path
	/// returns a PathBuf to the linked file (the one the link points to)
	pub fn remove_link(&self, in_dir: &PathBuf, source_rel_path: &PathBuf) -> io::Result<PathBuf> {
		let (link_path, source_path) = self._rel_link(in_dir, source_rel_path);
		if source_path.is_dir() {
			// create sym link
			if link_path.exists() {
				try!(fs::remove_dir(&link_path));
			}
		} else {
			try!(fs::remove_file(&link_path));
		}
		return Ok(source_path);
	}
	
	/// creates a link in the in_dir directory. Some links are relative, if so then they are relativized using source_rel_path
	/// returns a PathBuf to the linked file (the one the link points to)
	pub fn create_link(&self, in_dir: &PathBuf, source_rel_path: &PathBuf) -> io::Result<PathBuf> {
		let (link_path, source_path) = self._rel_link(in_dir, source_rel_path);
		if source_path.is_dir() {
			// create sym link
			if link_path.exists() {
				try!(fs::remove_dir(&link_path));
			}
			try!(symlink_dir(source_path.clone(), link_path));
			
			return Ok(source_path);
		} else {
			// create batch link
			let mut file = try!(File::create(&link_path));
			try!(file.write_all(format!("@{:?} %*", source_path.to_str().unwrap()).as_bytes()));
			
			return Ok(source_path);
		}
	}
	
	/// Returns actual paths to the files which this link is referencing to,
	/// in_dir - where the link file is to be placed
	/// source_rel_dir - where the links source directory is relative to, is not used in all cases. For instance if the link has an absolute path.
	/// returns (link_path, source_path), where link_path is where the link file should be. source_path is where the link points to
	fn _rel_link(&self, in_dir: &PathBuf, source_rel_path: &PathBuf) -> (PathBuf, PathBuf) {
		let mut source_path = PathBuf::from(&self.path);
		let is_file = !source_path.is_dir(); // for some reason is_file is always returning false... so use !is_dir
		
		// relativize the links path to the base directory (if its relative)
		if source_path.is_relative() {
			source_path = source_rel_path.join(&source_path);
		}
			
		// if the destination is a file then our source is a bat file
		// else if the the destination is a dir then our source is a sym link
		if is_file {
			// make the link path of {in_dir}/{link.name}.bat
			let mut link_name: String = self.name.clone();
			link_name.push_str(".bat");
			
			let link_path = in_dir.join(&link_name);
			
			return (link_path, source_path);
		} else {
			let link_path: PathBuf = in_dir.join(&self.name);
			return (link_path, source_path);
		}
	}
}
