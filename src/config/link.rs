use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::io;
use std::os::windows::fs::symlink_dir;

use std::io::prelude::*;

use config::*;

impl Link {
	
	pub fn new(name: String, path: String) -> Link {
		return Link {
				name: name,
				path: path
		};
	}
	
	/// removes this link in the in_dir directory. Some links are relative, they are relativized using source_rel_path
	/// returns a PathBuf to the linked file (the one the link points to)
	pub fn remove_link(&self, in_dir: &PathBuf, source_rel_path: &PathBuf) -> io::Result<PathBuf> {
		let (link_path, source_path) = self._rel_link(in_dir, source_rel_path);
		if source_path.is_dir() {
			// create sym link
			if link_path.exists() {
				try!(fs::remove_dir(&link_path));
			}
		} else {
			try!(fs::remove_file(&link_path));
		}
		return Ok(source_path);
	}
	
	/// creates a link in the in_dir directory. Some links are relative, if so then they are relativized using source_rel_path
	/// returns a PathBuf to the linked file (the one the link points to)
	pub fn create_link(&self, in_dir: &PathBuf, source_rel_path: &PathBuf) -> io::Result<PathBuf> {
		let (link_path, source_path) = self._rel_link(in_dir, source_rel_path);
		if source_path.is_dir() {
			// create sym link
			if link_path.exists() {
				try!(fs::remove_dir(&link_path));
			}
			try!(symlink_dir(source_path.clone(), link_path));
			
			return Ok(source_path);
		} else {
			// create batch link
			let mut file = try!(File::create(&link_path));
			try!(file.write_all(format!("@\"{}\" %*", source_path.to_str().unwrap()).as_bytes()));
			
			return Ok(source_path);
		}
	}
	
	/// Returns actual paths to the files which this link is referencing to,
	/// in_dir - where the link file is to be placed
	/// source_rel_dir - where the links source directory is relative to, is not used in all cases. For instance if the link has an absolute path.
	/// returns (link_path, source_path), where link_path is where the link file should be. source_path is where the link points to
	fn _rel_link(&self, in_dir: &PathBuf, source_rel_path: &PathBuf) -> (PathBuf, PathBuf) {
		let mut source_path = PathBuf::from(&self.path);
		let is_file = !source_path.is_dir(); // for some reason is_file is always returning false... so use !is_dir
		
		// relativize the links path to the base directory (if its relative)
		if source_path.is_relative() {
			source_path = source_rel_path.join(&source_path);
		}
			
		// if the destination is a file then our source is a bat file
		// else if the the destination is a dir then our source is a sym link
		if is_file {
			// make the link path of {in_dir}/{link.name}.bat
			let mut link_name: String = self.name.clone();
			link_name.push_str(".bat");
			
			let link_path = in_dir.join(&link_name);
			
			return (link_path, source_path);
		} else {
			let link_path: PathBuf = in_dir.join(&self.name);
			return (link_path, source_path);
		}
	}
}