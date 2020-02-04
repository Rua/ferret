pub mod video;
pub mod vulkan;

pub trait AsBytes {
	fn as_bytes<'a>(&'a self) -> &'a [u8];
}

impl<T> AsBytes for Vec<T> {
	fn as_bytes<'a>(&'a self) -> &'a [u8] {
		let slice = self.as_slice();
		unsafe {
			std::slice::from_raw_parts(slice.as_ptr() as _, std::mem::size_of::<T>() * slice.len())
		}
	}
}
