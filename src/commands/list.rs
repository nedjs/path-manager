use config::Config;
use config::LinkGroup;

pub const DESCRIPTION: &'static str = "Lists registered links and configuration groups";

pub fn print_usage() {
	println!("Usage: pman list [name]\n\
			 \0   name       (optional) Name of configuration group to list, \n\
			 \0              if ommited then all link groups and standalone links will be listed");
}

pub fn run(config: Config, args: &[String]) {
	
	if args.len() == 0 { // list everything
		println!("{} standalone link(s)", config.links().len());
		for link in config.links() {
			println!("   {} - {}", link.name, link.path);
		}
		println!("{} link group(s), * asterisk mark active groups", config.config_map().len());
		let mut key_set: Vec<_> = config.config_map().iter().collect();
		key_set.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
		
		for kv in key_set {
			let (key, _) = kv;	
			println!("  {}", key);
			list_entrys(&config, &key);
		}
	} else {
		list_entrys(&config, &args[0]); // just entries related to 1st argument
	}
}

fn list_entrys(config: &Config, name:&String) {
	let n_cfg:Option<&Vec<LinkGroup>> = config.config_entrys_by_name(&name);
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