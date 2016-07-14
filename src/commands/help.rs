use commands::*;
use std::ascii::AsciiExt;

pub const DESCRIPTION: &'static str = "Displays help text for all commands or individual commands";

pub fn print_help() {
	// TODO: uhh this way of printing text kinda stinks, i guess theres a nice macro called indoc which looks good. IDK tho
	println!("Usage: pman [command] <args>\n\
			  command options are:\n\
			  \0   configure  {configure}\n\
			  \0   list       {list}\n\
			  \0   group      {group}\n\
			  \0   swap       {swap}\n\
			  \0   link       {link}\n\
			  \0   rebuild    {rebuild}\n\
			  \0   help       {help}\n\
			  \n\
			  Many commands have additional arguments available to them.\n\
			  you may use 'pman help [command]' to display help specfically for a command.",
		configure=configure::DESCRIPTION,
		list=list::DESCRIPTION,
		group=group::DESCRIPTION,
		link=link::DESCRIPTION,
		swap=swap::DESCRIPTION,
		rebuild=rebuild::DESCRIPTION,
		help=help::DESCRIPTION
	);
}

pub fn run(args: &[String]) {
	if args.len() == 0 {
		print_help();
	} else {
		if "configure".eq_ignore_ascii_case(&args[0]) {
			println!("Description: {}", configure::DESCRIPTION);
			configure::print_usage();
		} else if "group".eq_ignore_ascii_case(&args[0]) {
			println!("Description: {}", group::DESCRIPTION);
			group::print_usage();
		} else if "link".eq_ignore_ascii_case(&args[0]) {
			println!("Description: {}", link::DESCRIPTION);
			link::print_usage();
		} else if "list".eq_ignore_ascii_case(&args[0]) {
			println!("Description: {}", list::DESCRIPTION);
			list::print_usage();
		} else if "rebuild".eq_ignore_ascii_case(&args[0]) {
			println!("Description: {}", rebuild::DESCRIPTION);
			rebuild::print_usage();
		} else if "swap".eq_ignore_ascii_case(&args[0]) {
			println!("Description: {}", swap::DESCRIPTION);
			swap::print_usage();
		} else {
			println!("Unkown help target \"{}\"", args[0]);
		}
	}
}