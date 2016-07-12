use utils::config::Config;
use utils::config::ConfigEntry;

pub fn list(config: &Config, args: &[String]) {
	
	if args.len() == 0 { // list everything
		println!("{} standalone link(s)", config.links().len());
		for link in config.links() {
			println!("   {} - {}", link.name, link.path);
		}
		println!("");
		println!("{} config group(s)", config.config_map().len());
		for (key, vec) in config.config_map() {
			println!("{}", key);
			list_entrys(&config, &key);
		}
	} else {
		list_entrys(&config, &args[0]);
	}
}

fn list_entrys(config: &Config, name:&String) {
	let n_cfg:Option<&Vec<ConfigEntry>> = config.config_entry(&name);
	match n_cfg {
		Some(cfg_vec) => {
			for entry in cfg_vec {
				let mut ast = " ";
				if config.is_active(entry) { ast = "*" }
				println!("{}  {} - {}", ast, entry.priority, entry.base_path)
			}
		},
		None => println!("No configs found for '{}'", name)
	}
}