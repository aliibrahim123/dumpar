use std::{
	env::current_dir,
	fs::{create_dir, create_dir_all, remove_dir_all, remove_file},
	os::unix::{fs::symlink, prelude::OsStrExt},
	path::{Path, PathBuf},
};

use clap::Parser;
use scripts::{Error, copy_all, err};

const RUST_DUMP: &str = "/disk2/dump-files/rust-target-dump/";

/// move the rust target directory into a build dump on disk2
#[derive(Parser)]
struct Args {
	/// path to target directory
	target_path: Option<PathBuf>,
}

fn main() -> Result<(), Error> {
	let Args { target_path } = Args::parse();
	let src = current_dir()?.join(target_path.unwrap_or(PathBuf::new()));

	let dump_slot = format!(
		"{}-{}",
		src.file_name().unwrap().display(),
		&blake3::hash(src.as_os_str().as_bytes()).to_hex()[..4]
	);

	let output = &Path::new(RUST_DUMP).join(dump_slot);
	if output.exists() {
		remove_dir_all(&output)?;
	}
	create_dir_all(output)?;

	let target = &src.join("target");
	if !target.exists() {
		return Err(err!("directory \"{} not found", target.display()));
	}

	copy_all(target, output)?;
	if target.is_dir() && !target.is_symlink() {
		remove_dir_all(target)?;
	} else {
		remove_file(target)?;
	}
	symlink(output, target)?;

	Ok(())
}
