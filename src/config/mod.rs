use std::collections::HashMap;

mod config;
mod link_group;
mod link;


/// the configuration for the entire path manager, saves to file
pub struct Config {
	config_map: HashMap<String, Vec<LinkGroup>>,
	active_configs: HashMap<String, u64>, // (entry.name, entry.priority)
	links: Vec<Link>,
	cmd_dir_str: String
}

#[derive(Clone, Debug)]
pub struct LinkGroup {
	pub name: String,
	pub base_path: String,
	pub links: Vec<Link>,
	pub priority: u64
}

#[derive(Clone, Debug)]
pub struct Link {
	pub name: String,
	pub path: String
}