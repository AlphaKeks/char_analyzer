use std::{
	collections::{BTreeMap, HashMap},
	env,
	fs::{self, File, OpenOptions},
	io::{ErrorKind, Write},
	process,
};

const NAME: &'static str = "char_analyzer";

fn main() {
	let mut args: Vec<String> = env::args().collect();

	let (input, output) = if args.len() > 1 {
		let input = args.remove(1);

		let output = if args.len() > 1 {
			args.remove(1)
		} else {
			return show_usage();
		};

		(input, output)
	} else {
		return show_usage();
	};

	let result = scan_file(scan_input(input));

	let json = serde_json::to_string_pretty(&result).expect("Failed to parse to JSON.");

	let mut file = match OpenOptions::new().write(true).append(true).open(&output) {
		Ok(file) => file,
		Err(why) => match why.kind() {
			ErrorKind::NotFound => File::create(output).expect("Failed to create file: {:#?}"),
			_ => panic!("Failed to open file: {:#?}", why),
		},
	};

	writeln!(&mut file, "{}", json).expect("Failed to write to file.");
}

fn show_usage() {
	println!("Usage: {NAME} [INPUT DIR] [OUTPUT DIR]");
	println!("Count characters in files.");
	process::exit(0);
}

fn scan_file(content: Vec<u8>) -> BTreeMap<u32, char> {
	let mut map = HashMap::new();

	for c in content {
		let c = c as char;

		if map.contains_key(&c) {
			*map.get_mut(&c).unwrap() += 1;
		} else {
			map.insert(c, 1);
		}
	}

	let map: BTreeMap<u32, char> = map.into_iter().map(|(k, v)| (v, k)).collect();
	map
}

fn scan_input(input: String) -> Vec<u8> {
	match fs::read(&input) {
		Ok(content) => return content,
		Err(_) => match fs::read_dir(input) {
			Ok(files) => {
				let mut contents = vec![];

				for file in files {
					if let Ok(file) = file {
						let mut result = scan_input(file.path().display().to_string());
						contents.append(&mut result);
					}
				}

				return contents;
			}
			Err(why) => panic!("{:#?}", why),
		},
	}
}
