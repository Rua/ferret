use crate::common::assets::DataSource;
use anyhow::{anyhow, ensure};
use arrayvec::ArrayString;
use byteorder::{ReadBytesExt, LE};
use std::{
	collections::HashSet,
	fs::File,
	io::{BufReader, Read, Seek, SeekFrom},
	path::{Path, PathBuf},
	str,
	string::String,
	vec::Vec,
};

struct Lump {
	path: PathBuf,
	name: String,
	offset: u64,
	size: usize,
}

#[derive(Default)]
pub struct WadLoader {
	lumps: Vec<Lump>,
	lump_names: HashSet<String>,
	wads: Vec<PathBuf>,
}

impl WadLoader {
	pub fn new() -> WadLoader {
		WadLoader {
			lumps: Vec::new(),
			lump_names: HashSet::new(),
			wads: Vec::new(),
		}
	}

	pub fn add<P: AsRef<Path>>(&mut self, path: P) -> anyhow::Result<()> {
		let path = path.as_ref();
		let file = File::open(path)?;
		let mut reader = BufReader::new(file);

		log::info!("Adding {}", path.display());
		let mut signature = [0u8; 4];
		reader.read_exact(&mut signature)?;
		ensure!(
			signature == *b"IWAD" || signature == *b"PWAD",
			"No IWAD or PWAD signature found."
		);

		let dir_length = reader.read_u32::<LE>()? as usize;
		let dir_offset = reader.read_u32::<LE>()? as u64;

		// Read WAD header, reserve space for new entries
		self.lumps.reserve(dir_length);

		// Read lump directory
		reader.seek(SeekFrom::Start(dir_offset))?;

		for _ in 0..dir_length {
			let offset = reader.read_u32::<LE>()? as u64;
			let size = reader.read_u32::<LE>()? as usize;
			let name = read_string(&mut reader)?.to_ascii_uppercase();

			self.lump_names.insert(name.clone());
			self.lumps.push(Lump {
				path: path.into(),
				name,
				offset,
				size,
			});
		}

		self.wads.push(path.into());

		Ok(())
	}

	pub fn wads(&self) -> impl Iterator<Item = &Path> {
		self.wads.iter().map(PathBuf::as_path)
	}
}

impl DataSource for WadLoader {
	fn load(&self, path: &str) -> anyhow::Result<Vec<u8>> {
		let path = path.to_ascii_uppercase();

		let (path, offset) = if let Some(index) = path.rfind("/+") {
			let (path, rest) = path.split_at(index);
			(path, rest[2..].parse()?)
		} else {
			(path.as_str(), 0)
		};

		// Find the index of this lump in the list
		let index = self
			.lumps
			.iter()
			.enumerate()
			.rev()
			.filter_map(|(i, lump)| if lump.name == path { Some(i) } else { None })
			.next()
			.ok_or(anyhow!("Lump \"{}\" not found", path))?;

		let lump = &self.lumps[index + offset];

		// Read lump
		let mut file = BufReader::new(File::open(&lump.path)?);
		let mut data = vec![0; lump.size];
		file.seek(SeekFrom::Start(lump.offset))?;
		file.read_exact(&mut data)?;

		Ok(data)
	}

	fn names<'a>(&'a self) -> Box<dyn Iterator<Item = &str> + 'a> {
		Box::from(self.lump_names.iter().map(String::as_str))
	}
}

pub fn read_string<R: Read>(reader: &mut R) -> anyhow::Result<ArrayString<[u8; 8]>> {
	let mut buf = [0u8; 8];
	reader.read_exact(&mut buf)?;
	Ok(ArrayString::from(std::str::from_utf8(&buf)?.trim_end_matches('\0')).unwrap())
}
