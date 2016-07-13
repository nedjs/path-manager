use utils::config::Config;

pub const DESCRIPTION: &'static str = "rebuilds the command directory by making all links again";

pub fn print_usage() {
	println!("Usage: pman rebuild");
}

pub fn rebuild(config: &Config, args: &[String]) {
	let cmd_dir = config.cmd_dir();
	for link in config.links() {
		if let Err(e) = link.create_link(&cmd_dir, &cmd_dir) {
			println!("Failed to create standalone link {}", link.name);
		}
	}
	for key in config.active_configs().keys() {
		if let Some(active_entry) = config.active_config_entry(key) {
			// activate entry
			active_entry.activate(&cmd_dir);
		}
	}
}