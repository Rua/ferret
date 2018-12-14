use byteorder::{LE, ReadBytesExt};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Cursor, Read, Seek, SeekFrom};
use std::path::Path;
use std::str;
use std::string::String;
use std::vec::Vec;

struct Lump {
	file: String,
	name: String,
	offset: u32,
	size: u32,
}

pub struct WadLoader {
	files: HashMap<String, BufReader<File>>,
	lumps: Vec<Lump>,
}

impl WadLoader {
	pub fn new() -> WadLoader {
		WadLoader {
			files: HashMap::new(),
			lumps: Vec::new(),
		}
	}
	
	pub fn add(&mut self, filename: &str) -> Result<(), Box<dyn Error>> {
		let path = Path::new(filename).canonicalize()?;
		let file = match File::open(&path) {
			Ok(val) => val,
			Err(err) => {
				error!("Could not open \"{}\": {}", path.to_string_lossy(), err);
				return Err(Box::from(err));
			}
		};
		let mut file = BufReader::new(file);
		
		let mut signature = [0u8; 4];
		file.read_exact(&mut signature)?;
		
		if !(&signature == b"IWAD" || &signature == b"PWAD") {
			panic!("No IWAD or PWAD signature found.");
		}
		
		// Read WAD header, reserve space for new entries
		let dir_length = file.read_u32::<LE>()?;
		let dir_offset = file.read_u32::<LE>()?;
		self.lumps.reserve(dir_length as usize);
		
		// Read lump directory
		file.seek(SeekFrom::Start(dir_offset as u64))?;
		
		for _ in 0..dir_length {
			let lump_offset = file.read_u32::<LE>()?;
			let lump_size = file.read_u32::<LE>()?;
			let mut lump_name = [0u8; 8];
			file.read_exact(&mut lump_name)?;
			
			let mut lump_name = String::from(str::from_utf8(&lump_name)?.trim_right_matches('\0'));
			lump_name.make_ascii_uppercase();
			
			self.lumps.push(Lump {
				file: String::from(filename),
				name: lump_name,
				offset: lump_offset,
				size: lump_size,
			});
		}
		
		self.files.insert(String::from(filename), file);
		Ok(())
	}
	
	pub fn read_lump(&mut self, number: usize) -> Result<Cursor<Vec<u8>>, Box<dyn Error>> {
		let lump = &self.lumps[number];
		let file = self.files.get_mut(&lump.file).expect("File referenced but not loaded");
		
		// Read lump
		let mut data = vec![0; lump.size as usize];
		file.seek(SeekFrom::Start(lump.offset as u64))?;
		file.read_exact(&mut data)?;
		
		Ok(Cursor::new(data))
	}
	
	pub fn index_for_name(&self, name: &str) -> Option<usize> {
		let name = name.to_ascii_uppercase();
		
		// Iterate in reverse, so that lumps added later are used first
		for (i, ref lump) in self.lumps.iter().enumerate().rev() {
			if lump.name == name {
				return Some(i);
			}
		}
		
		None
	}
	
	pub fn find_with_prefix(&self, prefix: &str) -> Vec<(String, usize)> {
		let mut names = Vec::new();
		
		// Iterate in reverse, so that lumps added later are used first
		for (i, ref lump) in self.lumps.iter().enumerate().rev() {
			if lump.name.starts_with(prefix) && !names.iter().any(|n: &(String, usize)| n.0 == lump.name) {
				names.push((lump.name.clone(), i));
			}
		}
		
		// Reverse the list back when we're done
		names.reverse();
		names
	}
}
