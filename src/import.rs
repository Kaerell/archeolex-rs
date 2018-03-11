//! # Import library
//! 
//! Based on Legilibre/legi.py.

// node2object à la place de xmlJSON
use std::io::{
	stdout, 
	Write, 
};
use std::path::Path;
use std::process::Command;
use curl::easy::{
	Easy, 
};
use libarchive::reader::{Builder, Reader};
use libarchive::archive as archive;
use treexml::Document;
use std::str;
use reqwest;
use std::fs;
use node2object;
use serde_json;


#[derive(Debug)]
struct FTPHandler {
	address: &'static str,
	res_path: Option<String>,
	stack_size: u32
}

/// Download files from JORF source and store it into tar_dir.
/// It is configured to download only newer files.
///
/// # Examples
/// let file_path = Path::new("*legi_*");
///	let tar_dir = Path::new("/home/trucmuche/Téléchargements/legi_20170622-212435");
///	remote_download_into_dir(&file_path, &tar_dir);
pub fn remote_download_into_dir(file_path: &Path, tar_dir: &Path) {
	println!("Start downloading legi");

	// TODO : assert for tar_dir and file_path instead of unwrap
	let output = Command::new("wget")
						  .arg("-c")  // continue
						  .arg("-N")  // enable timestamping mode(overwrite if newer version)
						  .arg("--no-remove-listing")  // keep list of files in ftp folder
						  .arg("-nH")  // don't use host to prefix downloaded files
						  .arg("-P")  // Put in folder tar_dir
						  .arg(tar_dir.to_str().unwrap())
						  .arg(("ftp://legi:open1234@ftp2.journal-officiel.gouv.fr/".to_owned() 
						  	    + file_path.to_str().unwrap())
						  	   .as_str())
						  .output()
						  .expect("Could not download files");
	
	println!("status: {}", output.status);
	println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
	println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

	assert!(output.status.success());
}

/// Import from XML inside GZ archive to couchDB
/// Imported from process_archive in tar2sqlite.py
pub fn local_import(tar_dir: &Path) {
	println!("Start import");
	let dir_iter: fs::ReadDir = fs::read_dir(tar_dir).unwrap();
	for entry in dir_iter {
		if let Ok(entry) = entry {
			let name = entry.file_name().into_string().unwrap();

			if name.ends_with("tar.gz") && name.starts_with("legi_") {
				println!("Import {:?}", entry.path());
				local_import_one_archive(&entry.path().as_path());
				break;
			}
		}
	}
}

/// handle decompression, convert from xml to json and push to couchdb a single file
pub fn local_import_one_archive(filename: &Path) {
	// initialize the reader
	let mut archive_builder = Builder::new();
	archive_builder.support_compression(archive::ReadCompression::Gzip).unwrap();
	archive_builder.support_format(archive::ReadFormat::Tar).unwrap();
	archive_builder.support_format(archive::ReadFormat::Gnutar).unwrap();
	archive_builder.support_filter(archive::ReadFilter::Gzip).unwrap();
	
	println!("Start reading");
	let mut archive_reader = archive_builder.open_file(&filename).unwrap();
	archive_reader.next_header();

	'entries: loop {
		let name;
		let size;
		{
			// the entry ref needs to be dropped at the end of this scope
			// in order to use archive_reader later
			let entry: &archive::Entry = archive_reader.entry();
			size = entry.size();
			if size > 0 {
				name = Some(String::from(entry.pathname()));
			}
			else {
				name = None;
			}
		}
		if let Some(file_name) = name {
			if file_name.ends_with(".xml"){
				let buf = archive_reader.read_all().unwrap();
				let file_content = str::from_utf8(&buf).unwrap();
				let base_name = String::from(file_name.rsplitn(2, '/').nth(0).unwrap());
				let url = format!("http://127.0.0.1:5984/legi/{:0}", base_name);
				println!("(size: {:?}, file:{:?})", size, base_name);
				let doc = Document::parse(file_content.as_bytes()).unwrap();
				let dom_root = doc.root.unwrap();
				let file_json = serde_json::Value::Object(node2object::node2object(&dom_root));
				let json_str = format!("{:0}", file_json);
				let client = reqwest::Client::new();
				let res = client.put(&*url)
					.body(json_str)
					.send();
				match res {
					Ok(response) => {
						println!("Importing in couchDB got result: {:?}", response.status());
						match response.status() {
							reqwest::StatusCode::Created => {}
							reqwest::StatusCode::Conflict => {
								// if the file already exists, update it
								let res_get = client.get(&*url);
								// serialize: res_get.body()
								// get latest _rev
								// add the latest _rev to file_json
								let json_str_post = format!("{:0}", file_json);
								let res_post = client.post(&*url)
									.body(json_str_post)
									.send();
								println!("Update in couchDB got result: {:?}", res_post.unwrap().status());
							}
							_ => {
								println!("Not expected");
							}
						}
					}
					Err(msg) => {
						println!("Failed to import in CouchDB {:0}: {:1}", base_name, msg);
					}
				}
			}
			else {
				// check path to get suppression list (use callback later here)
				// Example: 20171212-195005/liste_suppression.dat
				println!("{:?}", file_name);
			}
		}
		else if archive_reader.header_position() < 0 {
			println!("Error code: {:?}", archive_reader.header_position());
			break 'entries;
		}
		// println!("Pos {:?}", archive_reader.header_position());
		archive_reader.next_header();
	}

	// process_deletion();

	// Notes sur les problèmes à remonter pour libarchive:
	// segfault lorsqu'on ne fait pas un premier next_header()
	// segfault lorsqu'en fin d'archive on accède au pathname d'une entry inexistante
	// header_position = -30 => fin de fichier
}

fn process_deletion() {
	unimplemented!();
	// only in legi base
	// delete dossier, cid and id referenced
	// remove links in liens and sommaires
	// remove duplicates
}

/// Request a page on JORF FTP with curl and push it into couchDB
/// TODO: use curl_multi to download multiple files at once
pub fn remote_import_one_file(filename: &'static str) {
	println!("Starting download");
	let mut curl_handler = Easy::new();
	curl_handler.url(&*("ftp://legi:open1234@ftp2.journal-officiel.gouv.fr/".to_owned()+filename)).unwrap();
	curl_handler.write_function(|data| {
		stdout().write(data).unwrap();
		println!("Decoding now");
		// TODO: decode the stream instead of printing to stdout
		Ok(stdout().write(data).unwrap())
	}).unwrap();
	println!("Perform request");
	curl_handler.perform().unwrap();
	println!("End");
}