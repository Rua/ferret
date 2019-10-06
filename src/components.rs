use byteorder::{NetworkEndian as NE, ReadBytesExt, WriteBytesExt};
use nalgebra::Vector3;
use specs::{Component, FlaggedStorage, VecStorage};
use std::io::{Read, Write};


/*pub trait Component: Downcast + std::fmt::Debug {
	fn read_delta(&mut self, reader: &mut dyn Read) -> std::io::Result<()>;
	fn write_delta(&self, other: &dyn Component, writer: &mut dyn Write) -> std::io::Result<()>;
	fn eq(&self, other: &dyn Component) -> bool;
}

impl_downcast!(Component);*/

#[derive(Debug)]
pub struct NetworkComponent {
}

impl Component for NetworkComponent {
	type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}


#[derive(Debug)]
pub struct TransformComponent {
	pub position: Vector3<f32>,
	pub rotation: Vector3<f32>,
}

impl Component for TransformComponent {
	type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

impl Default for TransformComponent {
	fn default() -> TransformComponent {
		TransformComponent {
			position: Vector3::new(0.0, 0.0, 0.0),
			rotation: Vector3::new(0.0, 0.0, 0.0),
		}
	}
}

impl TransformComponent {
	pub fn write_delta(&self, writer: &mut dyn Write) -> std::io::Result<()> {
		writer.write_u8(1)?;
		writer.write_f32::<NE>(self.position[0])?;
		writer.write_f32::<NE>(self.position[1])?;
		writer.write_f32::<NE>(self.position[2])?;

		writer.write_u8(2)?;
		writer.write_f32::<NE>(self.rotation[0])?;
		writer.write_f32::<NE>(self.rotation[1])?;
		writer.write_f32::<NE>(self.rotation[2])?;

		writer.write_u8(0)?;

		Ok(())
	}

	pub fn read_delta(&mut self, reader: &mut dyn Read) -> std::io::Result<()> {
		loop {
			match reader.read_u8()? {
				1 => {
					self.position[0] = reader.read_f32::<NE>()?;
					self.position[1] = reader.read_f32::<NE>()?;
					self.position[2] = reader.read_f32::<NE>()?;
				},
				2 => {
					self.rotation[0] = reader.read_f32::<NE>()?;
					self.rotation[1] = reader.read_f32::<NE>()?;
					self.rotation[2] = reader.read_f32::<NE>()?;
				},
				_ => break,
			}
		}

		Ok(())
	}
}

/*impl Component for TransformComponent {
	fn write_delta(&self, other: &dyn Component, writer: &mut dyn Write) -> std::io::Result<()> {
		if let Some(other) = other.downcast_ref::<Self>() {
			if self.position != other.position {
				writer.write_u8(1)?;
				writer.write_f32::<NE>(self.position[0])?;
				writer.write_f32::<NE>(self.position[1])?;
				writer.write_f32::<NE>(self.position[2])?;
			}

			if self.rotation != other.rotation {
				writer.write_u8(2)?;
				writer.write_f32::<NE>(self.rotation[0])?;
				writer.write_f32::<NE>(self.rotation[1])?;
				writer.write_f32::<NE>(self.rotation[2])?;
			}

			writer.write_u8(0)?;
		}

		Ok(())
	}

	fn eq(&self, other: &dyn Component) -> bool {
		if let Some(other) = other.downcast_ref::<Self>() {
			*self == *other
		} else {
			false
		}
	}
}*/
