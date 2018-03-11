extern crate curl;
extern crate libarchive;
extern crate reqwest;
extern crate treexml;
extern crate node2object;
extern crate serde_json;

pub mod import;

use std::env;
use std::path::PathBuf;
// use std::path::Path;
use import::{
	// remote_import_one_file, 
	// remote_download_into_dir, 
	local_import
};

fn main() {
	// remote_import_one_file("legi_20170622-212435.tar.gz");
	// let file_path = Path::new("*legi_*");
	let mut tar_dir = PathBuf::new();
	tar_dir.push(env::home_dir().unwrap());
	tar_dir.push("Téléchargements/legi_20170622-212435");
	println!("{:?}", tar_dir);
	// remote_download_into_dir(&file_path, &tar_dir);
	local_import(&tar_dir.as_path());
}