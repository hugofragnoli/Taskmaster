use std::env;

mod config;

use config::parser::parse_config;

fn main() {
	let args: Vec<String> = env::args().collect();
	let taskmaster = parse_config(args[1].clone());

	println!("{:#?}", taskmaster);
}
