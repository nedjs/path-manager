use utils;

pub const DESCRIPTION: &'static str = "Installs a new configuration group";

pub fn print_usage() {
	// TODO: uhh this way of printing text kinda stinks, i guess theres a nice macro called indoc which looks good. IDK tho
	println!("I havent written this yet, edit the config yourself");
}

pub fn run(config: utils::config::Config, args: &[String]) {
	print_usage();
//	config.get_config_entry(&args[0]);
////	config.g(&args[0]);
//	println!("Running install");
//	for i in 0..args.len() {
//		println!("{}", &args[i]);
//	}
}