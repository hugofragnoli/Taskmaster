mod config;

use config::parser::parse_config;

fn main() {
	let taskmaster = parse_config();

	println!("{:#?}", taskmaster);
}
