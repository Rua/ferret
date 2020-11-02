mod context;
pub mod definition;
mod draw;
mod present;

pub use {
	context::RenderContext,
	draw::{DrawContext, DrawTarget},
	present::PresentTarget,
};

pub trait AsBytes {
	fn as_bytes(&self) -> &[u8];
}

impl<T> AsBytes for Vec<T> {
	fn as_bytes(&self) -> &[u8] {
		let slice = self.as_slice();
		unsafe {
			std::slice::from_raw_parts(slice.as_ptr() as _, std::mem::size_of::<T>() * slice.len())
		}
	}
}
