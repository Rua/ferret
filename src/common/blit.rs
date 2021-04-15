pub fn blit<F, S, D>(
	mut func: F,
	src_data: &[S],
	src_size: [usize; 2],
	dst_data: &mut [D],
	dst_size: [usize; 2],
	dst_offset: [isize; 2],
) where
	F: FnMut(&S, &mut D),
{
	if -dst_offset[0] >= src_size[0] as isize
		|| -dst_offset[1] >= src_size[1] as isize
		|| dst_offset[0] >= dst_size[0] as isize
		|| dst_offset[1] >= dst_size[1] as isize
	{
		// Entirely out of range
		return;
	}

	let src_offset = [
		std::cmp::max(0, -dst_offset[0]) as usize,
		std::cmp::max(0, -dst_offset[1]) as usize,
	];

	let blit_size = [
		std::cmp::min(src_size[0], (dst_size[0] as isize - dst_offset[0]) as usize) - src_offset[0],
		std::cmp::min(src_size[1], (dst_size[1] as isize - dst_offset[1]) as usize) - src_offset[1],
	];

	let dst_offset = [
		(src_offset[0] as isize + dst_offset[0]) as usize,
		(src_offset[1] as isize + dst_offset[1]) as usize,
	];

	let src_rows = src_data[src_offset[1] * src_size[0]..][..blit_size[1] * src_size[0]]
		.chunks_exact(src_size[0]);
	let dst_rows = dst_data[dst_offset[1] * dst_size[0]..][..blit_size[1] * dst_size[0]]
		.chunks_exact_mut(dst_size[0]);
	debug_assert_eq!(src_rows.len(), blit_size[1]);
	debug_assert_eq!(dst_rows.len(), blit_size[1]);

	for (src_row, dst_row) in src_rows.zip(dst_rows) {
		let src_iter = src_row[src_offset[0]..src_offset[0] + blit_size[0]].iter();
		let dst_iter = dst_row[dst_offset[0]..dst_offset[0] + blit_size[0]].iter_mut();
		debug_assert_eq!(src_iter.len(), blit_size[0]);
		debug_assert_eq!(dst_iter.len(), blit_size[0]);

		for (src, dst) in src_iter.zip(dst_iter) {
			func(src, dst);
		}
	}
}
