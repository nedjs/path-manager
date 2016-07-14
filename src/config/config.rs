
use std::env;
use std;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::path::Path;
use std::io::BufReader;
use std::io;
use std::collections::HashMap;

use std::io::prelude::*;

use config::*;


impl Config {
	
	pub fn new(cmd_dir: String) -> Config {
		return Config {
				active_configs: HashMap::new(),
				config_map: HashMap::new(),
				links: Vec::new(),
				cmd_dir_str: cmd_dir.to_owned()
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
	
	pub fn insert_config_entry(&mut self, entry: LinkGroup) -> bool {
		let name = entry.name.to_owned();
		if !self.config_map.contains_key(&name) {
			self.config_map.insert(name.clone(), Vec::new());
		}
		let mut m_vec = self.config_map.get(&name).unwrap().to_owned();
		
		// check for duplicate priority
		for e in &m_vec {
			if e.priority == entry.priority {
				return false;
			}
		}
		m_vec.push(entry);
		self.config_map.insert(name.to_owned(), m_vec);
		return true;
	}
	
	/// removes a config entry by name and priority
	/// returns true if it was found and removed
	pub fn remove_config_entry(&mut self, name: &String, priority: u64) -> bool {
		if !self.config_map.contains_key(name) {
			return false;
		}
		if let Some(m_vec) = self.config_map.get_mut(name) {
			for i in 0..m_vec.len() {
				if m_vec[i].priority == priority {
					m_vec.remove(i);
					return true;
				}
			}
		}
		
		
		return false;
	}
	
	/// configuration entries by string name
	pub fn config_entrys_by_name(&self, name: &String) -> Option<&Vec<LinkGroup>> {
		return self.config_map.get(name);
	}
	
	/// the Hashmap of all configuration entries, these are held by name -> vec
	pub fn config_map(&self) -> &HashMap<String, Vec<LinkGroup>> {
		return &self.config_map;
	}
	
	/// all standalone links
	pub fn links(&self) -> &Vec<Link> {
		return &self.links;
	}
	
	/// all active configurations
	pub fn active_configs(&self) -> &HashMap<String, u64> {
		return &self.active_configs;
	}
	
	/// retrives the current LinkGroup for the given name
	pub fn active_config_entry(&self, name: &String) -> Option<&LinkGroup> {
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
	
	/// checks if the LinkGroup is seen as active
	pub fn is_active(&self, cfg_entry: &LinkGroup) -> bool {
		self.config_map.keys();
		match self.active_configs.get(&cfg_entry.name) {
			Some(p) => return cfg_entry.priority.eq(p),
			None => return false
		}
	}
	
	
	pub fn select_group(&self, name: &String, pri: u64) -> Option<LinkGroup> {
		if let Some(cfg_vec) = self.config_entrys_by_name(&name) {
			
			for i in 0..cfg_vec.len() {
				if cfg_vec[i].priority == pri {
					return Some(cfg_vec[i].clone());
				}
			}
			return None;
		} else{
			return None;
		}
	}
	
	
	/// selects the group with the highest priority
	pub fn select_highest_group(&self, name: &String) -> Option<LinkGroup> {
		if let Some(cfg_vec) = self.config_entrys_by_name(&name) {
			let mut sel_grp: Option<&LinkGroup> = None; // matched group
			
			for i in 0..cfg_vec.len() {
				if sel_grp.is_none() || cfg_vec[i].priority > sel_grp.unwrap().priority {
					sel_grp = cfg_vec.get(i);
				}
			}
			
			return sel_grp.cloned();
		}
		return None;
	}
	
	/// selects the best configuration available for the given name and priority where ret.priority>=pri.
	pub fn select_closest_group(&self, name: &String, pri: u64) -> Option<LinkGroup> {
		if let Some(cfg_vec) = self.config_entrys_by_name(&name) {
			let mut sel_grp: Option<&LinkGroup> = None; // matched group
			
			for i in 0..cfg_vec.len() {
				// gotta be greater than the pri param and have a smaller priority than our current chosen group
				if cfg_vec[i].priority>=pri && (sel_grp.is_none() || cfg_vec[i].priority<=sel_grp.unwrap().priority) {
					sel_grp = cfg_vec.get(i);
				}
			}
			
			return sel_grp.cloned();
		}
		return None;
		
	}
	
	
	/// changes the current active configuration
	pub fn set_active(&mut self, name: &String, priority: &u64) {
		self.active_configs.insert(name.clone(), priority.clone());
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
			try!(file.write_all(b"\n\n"));// 2 more newline between entries
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
		let cmd_dir_str;
		
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
	
	

	fn _read_active_configs(map: &mut HashMap<String, u64>, reader: &mut BufReader<File>) -> io::Result<()> {
		let mut name = String::new();
		
		if let Err(e) = Config::_read_to_empty_line(reader, &mut |line| {
			if name == "" {
				name = line.clone();
			} else {
				// name is set, then we have path
				
				let priority = line.parse::<u64>().unwrap_or_else(|e| {
					println!("Invalid priority \"{}\" while parsing configuration \"{}\".\n\"{}\"", line, name, e);
					std::process::exit(1);
				});
				map.insert(name.clone(), priority);
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
	fn _read_configs(configs: &mut HashMap<String, Vec<LinkGroup>>, reader: &mut BufReader<File>) -> io::Result<()> {
		
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
	fn _read_config(name: String, reader: &mut BufReader<File>) -> Result<Option<LinkGroup>, io::Error> {
		let mut line = String::new();
	
		let mut base_path = String::new();
		let mut priority: Option<u64> = None;
		let mut links = Vec::new();
		let mut link_name = String::new();
		

		// TODO: rewrite this garbage to not use line numbers. We always know the first 2 lines, then use a loop on the rest
		// keeping this error thing here cause im gonna need it to throw if the first 2 lines arent there, maybe, I need to return some error but IO seems wrong
//		io::Error::new(io::ErrorKind::Other, "");
		
		let mut line_num = 0;
		while try!(reader.read_line(&mut line)) > 0 {
			line = String::from(line.trim()); // trim to remove newline chars
			
			
	//		println!("\"{}\"", line);
			if line_num == 0 {
				base_path = line.clone();
			} else {
				// at this point new line signals end of config
				if line == "" { break; }
				if line_num == 1 {
					priority = Some(line.parse::<u64>().unwrap_or_else(|e| {
							println!("Invalid priority \"{}\" while parsing configuration \"{}\".\n\"{}\"", line, name, e);
							std::process::exit(1);
					}));
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
			}
			
		    line.clear();
		    line_num+=1;
		}
		
		if priority.is_none() {
			return Ok(None);
		} else {
			return Ok(Some(LinkGroup {
				name: name,
				base_path: base_path,
				priority: priority.unwrap(),
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