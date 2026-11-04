use std::{
	fmt::Display,
	fs::{ReadDir, copy, create_dir, read_dir, read_link},
	io,
	os::unix::fs::symlink,
	path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct Error(pub String);

impl<T: std::error::Error> From<T> for Error {
	fn from(value: T) -> Self {
		Self(value.to_string())
	}
}
impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&self.0)
	}
}
#[macro_export]
macro_rules! err {
	($($arg:tt)*) => {
		Error(format!($($arg)*))
	};
}

pub fn copy_all(
	src: impl AsRef<Path>, dst: impl AsRef<Path>,
) -> Result<(), std::io::Error> {
	let dst = dst.as_ref();
	create_dir(dst)?;
	for entry in read_dir(src)? {
		let entry = entry?.path();
		let dst = &dst.join(entry.file_name().unwrap());
		if entry.is_symlink() {
		} else if entry.is_dir() {
			copy_all(entry, dst)?;
		} else if entry.is_file() {
			copy(entry, dst)?;
		} else if entry.is_symlink() {
			symlink(read_link(entry)?, dst)?
		}
	}
	Ok(())
}

pub struct WalkDir {
	stack: Vec<ReadDir>,
}

pub fn walk_dir(path: impl AsRef<Path>) -> Result<WalkDir, io::Error> {
	Ok(WalkDir { stack: vec![read_dir(path)?] })
}

impl Iterator for WalkDir {
	type Item = Result<PathBuf, io::Error>;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			let top = self.stack.last_mut()?;
			match top.next() {
				None => {
					self.stack.pop();
				}
				Some(Err(e)) => return Some(Err(e)),
				Some(Ok(entry)) => {
					let path = entry.path();
					if path.is_dir() {
						match read_dir(&path) {
							Ok(rd) => self.stack.push(rd),
							Err(e) => return Some(Err(e)),
						}
					} else if path.is_file() {
						return Some(Ok(path));
					}
				}
			}
		}
	}
}
