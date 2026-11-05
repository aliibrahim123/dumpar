use clap::Parser;
use scripts::{Error, copy_all};
use std::{
	fs::{copy, create_dir_all, remove_dir_all, remove_file},
	path::PathBuf,
};

const DUMP_DIR: &str = "/disk2/dump-files/dumpar";
const DEV_DIR: &str = "/home/ali/dev";

#[derive(Debug, Clone, Copy, PartialEq)]
enum Origin {
	Dev,
}
#[derive(Debug, Clone, PartialEq)]
struct Entry {
	src: &'static str,
	dest: &'static str,
	origin: Origin,
}

const ENTRIES: &[Entry] = &[
	Entry { src: "gramex/tests", dest: "tests/gramex", origin: Origin::Dev },
	Entry { src: "mep/tests", dest: "tests/mep", origin: Origin::Dev },
	Entry { src: "neocomp/test", dest: "tests/neocomp", origin: Origin::Dev },
	Entry { src: "neoview/tests", dest: "tests/neoview", origin: Origin::Dev },
	Entry { src: "scripts", dest: "scripts", origin: Origin::Dev },
	Entry { src: "structom/tests", dest: "tests/structom", origin: Origin::Dev },
	Entry { src: "ts-madness/test", dest: "tests/ts-madness", origin: Origin::Dev },
];
/*
const dirs = {
	'tests/gramex': 'gramex/tests',
	'tests/mep': 'mep/tests',
	'tests/neocomp': 'neocomp/test',
	'tests/neoview': 'neoview/tests',
	scripts: 'scripts',
	'tests/structom': 'structom/tests',
	'tests/ts-madness': 'ts-madness/test',
	'uni-hw': 'uni-hw',
};
 */

#[derive(Parser)]
struct Args {
	#[arg(long)]
	dump_dir: Option<PathBuf>,
	#[arg(long)]
	dev_dir: Option<PathBuf>,
}

fn main() -> Result<(), Error> {
	let Args { dump_dir, dev_dir } = Args::parse();
	let dump_dir = dump_dir.unwrap_or_else(|| PathBuf::from(DUMP_DIR));
	let dev_dir = dev_dir.unwrap_or_else(|| PathBuf::from(DEV_DIR));

	for Entry { src, dest, origin } in ENTRIES {
		let origin = match origin {
			Origin::Dev => &dev_dir,
		};
		let src = origin.join(src).canonicalize()?;
		let dest = dump_dir.join(dest);

		if dest.exists() {
			if dest.is_dir() {
				remove_dir_all(&dest)?;
			} else if dest.is_file() {
				remove_file(&dest)?;
			}
		}

		create_dir_all(dest.parent().unwrap())?;
		if src.is_dir() {
			copy_all(src, dest)?;
		} else if src.is_file() {
			copy(src, dest)?;
		}
	}

	Ok(())
}
