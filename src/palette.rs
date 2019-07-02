use sdl2::pixels::Color;
use std::ops::Index;

pub struct Palette(pub [Color; 256]);

impl Index<usize> for Palette {
	type Output = Color;

	fn index(&self, index: usize) -> &Color {
		&self.0[index]
	}
}
