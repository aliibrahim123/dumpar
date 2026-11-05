use std::{
	env::current_dir,
	fs::{create_dir_all, read_to_string, write},
	io::prelude::Write,
	path::{Path, PathBuf},
	process::{Command, Stdio},
	str::FromStr,
};

use clap::Parser;
use proc_macro2::TokenStream;
use quote::ToTokens;
use scripts::{Error, err, walk_dir};
use syn::{
	ImplItem, ItemImpl, Variant, Visibility, parse_file,
	token::Pub,
	visit_mut::{self, VisitMut, visit_item_impl_mut, visit_visibility_mut},
};

/// malke all items public in the given rust source
#[derive(Parser)]
struct Args {
	/// path to source directory
	src: PathBuf,
	/// path to destination directory
	dest: PathBuf,
}

struct PublicVis;
impl VisitMut for PublicVis {
	fn visit_visibility_mut(&mut self, vis: &mut Visibility) {
		*vis = Visibility::Public(Pub::default())
	}
	fn visit_item_impl_mut(&mut self, i: &mut ItemImpl) {
		visit_item_impl_mut(self, i);
		if i.trait_.is_some() {
			for item in &mut i.items {
				match item {
					ImplItem::Fn(f) => f.vis = Visibility::Inherited,
					ImplItem::Const(c) => c.vis = Visibility::Inherited,
					ImplItem::Type(t) => t.vis = Visibility::Inherited,
					_ => {}
				}
			}
		}
	}

	fn visit_variant_mut(&mut self, v: &mut Variant) {
		visit_mut::visit_variant_mut(self, v);
		for f in v.fields.iter_mut() {
			f.vis = Visibility::Inherited;
		}
	}
}

fn transform(src: &Path, dest: &Path) -> Result<(), Error> {
	let src = read_to_string(src)?;

	let mut root = parse_file(&src)?;
	PublicVis.visit_file_mut(&mut root);
	let output = root.to_token_stream().to_string();

	create_dir_all(dest.parent().unwrap())?;
	write(dest, output)?;

	Ok(())
}

fn main() -> Result<(), Error> {
	let Args { src, dest } = Args::parse();
	let dest = current_dir()?.join(dest);
	if !src.exists() {
		return Err(err!("source \"{}\" not found", src.display()));
	}

	if src.is_file() && src.extension().is_some_and(|ext| ext == "rs") {
		transform(&src, &dest)?;
	} else if src.is_dir() {
		for file in walk_dir(&src)? {
			let file = file?;
			if file.is_file() && file.extension().is_some_and(|ext| ext == "rs") {
				transform(&file, &dest.join(file.strip_prefix(&src)?))?;
			}
		}
	}

	Ok(())
}
