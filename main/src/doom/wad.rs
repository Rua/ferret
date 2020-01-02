use crate::assets::DataSource;
use serde::Deserialize;
use std::{
	collections::HashSet,
	error::Error,
	fs::File,
	io::{BufReader, Read, Seek, SeekFrom},
	str,
	string::String,
	vec::Vec,
};

struct Lump {
	file: String,
	name: String,
	offset: u32,
	size: u32,
}

#[derive(Default)]
pub struct WadLoader {
	lumps: Vec<Lump>,
	names: HashSet<String>,
}

#[derive(Deserialize)]
struct Header {
	signature: [u8; 4],
	dir_length: u32,
	dir_offset: u32,
}

#[derive(Deserialize)]
struct DirEntry {
	lump_offset: u32,
	lump_size: u32,
	lump_name: [u8; 8],
}

impl WadLoader {
	pub fn new() -> WadLoader {
		WadLoader {
			lumps: Vec::new(),
			names: HashSet::new(),
		}
	}

	pub fn add(&mut self, filename: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
		let file = File::open(filename)?;
		let mut file = BufReader::new(file);

		let header: Header = bincode::deserialize_from(&mut file)?;

		if !(header.signature == *b"IWAD" || header.signature == *b"PWAD") {
			panic!("No IWAD or PWAD signature found.");
		}

		// Read WAD header, reserve space for new entries
		self.lumps.reserve(header.dir_length as usize);

		// Read lump directory
		file.seek(SeekFrom::Start(header.dir_offset as u64))?;

		for _ in 0..header.dir_length {
			let dir_entry: DirEntry = bincode::deserialize_from(&mut file)?;

			let mut lump_name =
				String::from(str::from_utf8(&dir_entry.lump_name)?.trim_end_matches('\0'));
			lump_name.make_ascii_uppercase();

			self.names.insert(lump_name.to_owned());
			self.lumps.push(Lump {
				file: filename.to_owned(),
				name: lump_name,
				offset: dir_entry.lump_offset,
				size: dir_entry.lump_size,
			});
		}

		Ok(())
	}
}

impl DataSource for WadLoader {
	fn load(&self, path: &str) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
		let path = path.to_ascii_uppercase();

		let (path, offset) = if let Some(index) = path.rfind("/+") {
			let (path, rest) = path.split_at(index);
			(path, rest[2..].parse()?)
		} else {
			(path.as_str(), 0)
		};

		// Find the index of this lump in the list
		let index =
			self.lumps
				.iter()
				.enumerate()
				.rev()
				.filter_map(|(i, lump)| if lump.name == path { Some(i) } else { None })
				.next()
				.ok_or(Box::from(format!("Lump \"{}\" not found", path))
					as Box<dyn Error + Send + Sync>)?;

		let lump = &self.lumps[index + offset];

		// Read lump
		let mut file = BufReader::new(File::open(&lump.file)?);
		let mut data = vec![0; lump.size as usize];
		file.seek(SeekFrom::Start(lump.offset as u64))?;
		file.read_exact(&mut data)?;

		Ok(data)
	}

	fn names<'a>(&'a self) -> Box<dyn Iterator<Item = &str> + 'a> {
		Box::from(self.names.iter().map(String::as_str))
	}
}
