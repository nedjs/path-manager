use utils::config::Config;
use utils::config::ConfigEntry;

pub const DESCRIPTION: &'static str = "Lists registered links and configuration groups";

pub fn print_usage() {
	println!("Usage: pman list [name]\n\
			 \0   name       (optional) Name of configuration group to list, \n\
			 \0              if ommited then all links and configurations will be listed");
}

pub fn run(config: Config, args: &[String]) {
	
	if args.len() == 0 { // list everything
		println!("{} standalone link(s)", config.links().len());
		for link in config.links() {
			println!("   {} - {}", link.name, link.path);
		}
		println!("{} config group(s)", config.config_map().len());
		for key in config.config_map().keys() {
			println!("  {}", key);
			list_entrys(&config, &key);
		}
	} else {
		list_entrys(&config, &args[0]); // just entries related to 1st argument
	}
}

fn list_entrys(config: &Config, name:&String) {
	let n_cfg:Option<&Vec<ConfigEntry>> = config.config_entrys_by_name(&name);
	match n_cfg {
		Some(cfg_vec) => {
			for entry in cfg_vec {
				let mut ast = " ";
				if config.is_active(entry) { ast = "*" }
				println!("    {} {} - {}", ast, entry.priority, entry.base_path)
			}
		},
		None => println!("No configs found for '{}'", name)
	}
}