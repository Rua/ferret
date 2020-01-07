use crate::assets::DataSource;
use byteorder::{ReadBytesExt, LE};
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
	offset: u64,
	size: usize,
}

#[derive(Default)]
pub struct WadLoader {
	lumps: Vec<Lump>,
	names: HashSet<String>,
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
		let mut reader = BufReader::new(file);

		let mut signature = [0u8; 4];
		reader.read_exact(&mut signature)?;

		if !(signature == *b"IWAD" || signature == *b"PWAD") {
			panic!("No IWAD or PWAD signature found.");
		}

		let dir_length = reader.read_u32::<LE>()? as usize;
		let dir_offset = reader.read_u32::<LE>()? as u64;

		// Read WAD header, reserve space for new entries
		self.lumps.reserve(dir_length);

		// Read lump directory
		reader.seek(SeekFrom::Start(dir_offset))?;

		for _ in 0..dir_length {
			let offset = reader.read_u32::<LE>()? as u64;
			let size = reader.read_u32::<LE>()? as usize;
			let mut lump_name = [0u8; 8];
			reader.read_exact(&mut lump_name)?;

			let mut name = String::from(str::from_utf8(&lump_name)?.trim_end_matches('\0'));
			name.make_ascii_uppercase();

			self.names.insert(name.clone());
			self.lumps.push(Lump {
				file: filename.to_owned(),
				name,
				offset,
				size,
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
		let mut data = vec![0; lump.size];
		file.seek(SeekFrom::Start(lump.offset))?;
		file.read_exact(&mut data)?;

		Ok(data)
	}

	fn names<'a>(&'a self) -> Box<dyn Iterator<Item = &str> + 'a> {
		Box::from(self.names.iter().map(String::as_str))
	}
}
