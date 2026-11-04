use std::{
	env::current_dir,
	fmt::Write as _,
	fs::{read, read_to_string},
	io::prelude::Write as _,
	path::PathBuf,
	process::{Command, Stdio},
};

use clap::Parser;
use regex::Regex;
use scripts::{Error, err, walk_dir};

/// copy the content of a directory into the clipboard
#[derive(Parser)]
struct Args {
	/// the source file
	src: Option<PathBuf>,
	/// include files whose path satisfy this regex
	#[arg(short, long)]
	include: Option<String>,
	/// exclude files whose path satisfy this regex
	#[arg(short, long)]
	exclude: Option<String>,
}

fn main() -> Result<(), Error> {
	let Args { src, include, exclude } = Args::parse();
	let src = current_dir()?.join(src.unwrap_or(PathBuf::new()));
	let include = include
		.map(|pat| Regex::new(&pat).map_err(|e| err!("invalid include: {e}")))
		.transpose()?;
	let exclude = exclude
		.map(|pat| Regex::new(&pat).map_err(|e| err!("invalid exclude: {e}")))
		.transpose()?;

	if !src.exists() {
		return Err(err!("source \"{}\" doesnt exist", src.display()));
	}

	let mut res = String::new();
	let mut files = Vec::new();
	for file in walk_dir(&src)? {
		let file = file?;
		let Some(path) = file.to_str() else { return Err(err!("invalid path")) };
		if include.as_ref().is_none_or(|pat| pat.is_match(path))
			&& !exclude.as_ref().is_some_and(|pat| pat.is_match(path))
		{
			files.push(path.to_string());
			let content = String::from_utf8(read(path)?)
				.map_err(|_| err!("file \"{path}\" is not valid utf8"))?;
			write!(res, "==== file: ${path} ====\n{content}\n\n")?;
		}
	}

	let mut child = Command::new("wl-copy").stdin(Stdio::piped()).spawn()?;
	child.stdin.as_mut().unwrap().write_all(res.as_bytes())?;
	child.wait()?;

	println!("{res}");

	println!("processed files:");
	for file in files {
		println!("{file}")
	}

	Ok(())
}
