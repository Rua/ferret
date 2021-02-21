use crate::common::assets::DataSource;
use anyhow::{bail, ensure};
use arrayvec::ArrayString;
use byteorder::{ReadBytesExt, LE};
use relative_path::RelativePath;
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
			let name = read_string(&mut reader)?;

			self.lump_names.insert(name.as_str().to_owned());
			self.lumps.push(Lump {
				path: path.into(),
				name: name.as_str().to_owned(),
				offset,
				size,
			});
		}

		self.wads.push(path.into());

		Ok(())
	}

	/*pub fn wads(&self) -> impl Iterator<Item = &Path> {
		self.wads.iter().map(PathBuf::as_path)
	}*/

	fn index_for_name(&self, path: &RelativePath) -> anyhow::Result<usize> {
		let lump_name = path.file_stem().unwrap();

		// Find the index of this lump in the list
		let index = match self
			.lumps
			.iter()
			.enumerate()
			.rev()
			.filter_map(|(i, lump)| {
				if lump.name == lump_name {
					Some(i)
				} else {
					None
				}
			})
			.next()
		{
			Some(index) => index,
			None => bail!("Lump \"{}\" not found", lump_name),
		};

		let offset = match path.extension() {
			Some("things") | Some("gl_vert") => 1,
			Some("linedefs") | Some("gl_segs") => 2,
			Some("sidedefs") | Some("gl_ssect") => 3,
			Some("vertexes") | Some("gl_nodes") => 4,
			Some("segs") => 5,
			Some("ssectors") => 6,
			Some("nodes") => 7,
			Some("sectors") => 8,
			Some("reject") => 9,
			Some("blockmap") => 10,
			_ => 0,
		};

		let ret = index + offset;
		let lump = &self.lumps[ret];

		if offset != 0 && path.extension().unwrap() != lump.name {
			bail!(
				"Lump \"{}\" for map \"{}\" not found",
				path.extension().unwrap(),
				lump_name
			);
		}

		Ok(ret)
	}
}

impl DataSource for WadLoader {
	fn load(&self, path: &RelativePath) -> anyhow::Result<Vec<u8>> {
		let index = self.index_for_name(path)?;
		let lump = &self.lumps[index];

		// Read lump
		let mut file = BufReader::new(File::open(&lump.path)?);
		let mut data = vec![0; lump.size];
		file.seek(SeekFrom::Start(lump.offset))?;
		file.read_exact(&mut data)?;

		Ok(data)
	}

	fn exists(&self, path: &RelativePath) -> bool {
		self.index_for_name(path).is_ok()
	}

	fn names<'a>(&'a self) -> Box<dyn Iterator<Item = &str> + 'a> {
		Box::from(self.lump_names.iter().map(String::as_str))
	}
}

pub fn read_string<R: Read>(reader: &mut R) -> anyhow::Result<ArrayString<[u8; 8]>> {
	let mut buf = [0u8; 8];
	reader.read_exact(&mut buf)?;
	let mut string =
		ArrayString::from(std::str::from_utf8(&mut buf)?.trim_end_matches('\0')).unwrap();
	string.make_ascii_lowercase();
	Ok(string)
}

#[derive(Clone, Copy)]
pub struct IWADInfo {
	pub files: &'static [&'static str],
	pub map: &'static str,
	pub weapons: &'static [&'static str],
}
