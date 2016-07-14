use std::path::PathBuf;
use config::*;

impl LinkGroup {
	
	pub fn new(name: String, priority: u64) -> LinkGroup {
		return LinkGroup {
			name: name,
			priority: priority,
			base_path: String::new(),
			links: Vec::new()
		}
	}
	
	pub fn set_base_path(&mut self, base_path: String) {
		self.base_path = base_path;
	}
	
	pub fn get_link(&self, name: &String) -> Option<Link> {
		for i in 0..self.links.len() {
			if self.links[i].name.eq(name) {
				return Some(self.links[i].to_owned());
			}
		}
		return None;
	}
	
	/// removes the link from this groups configuration
	pub fn remove_link(&mut self, name: &String) -> bool {
		for i in 0..self.links.len() {
			if self.links[i].name.eq(name) {
				self.links.remove(i);
				return true;
			}
		}
		return false;
	}
	
	/// adds the link to this group, will override any existing link with the same name in this group
	/// return true if the link was overwritten
	pub fn add_link(&mut self, link: Link) -> bool {
		let did_remove = self.remove_link(&link.name);
		self.links.push(link);
		return did_remove;
	}
	
	pub fn activate(&self, in_dir: &PathBuf) {
		let source_path = PathBuf::from(&self.base_path);
		for link in self.links.iter() {
			if let Err(e) = link.create_link(&in_dir, &source_path) {
				println!("Unable to create link: \"{}\", {}", link.name, e)
			}
		}
	}
	
	pub fn deactivate(&self, in_dir: &PathBuf) {
		let source_path = PathBuf::from(&self.base_path);
		for link in self.links.iter() {
			if let Err(e) = link.remove_link(&in_dir, &source_path) {
				println!("Unable to remove link: \"{}\", {}", link.name, e)
			}
		}
	}
}