use std::io;
use std::io::BufRead;
use std::thread::Builder;

use crate::commands::CommandSender;

// Spawns a thread to read commands from stdin asynchronously
pub fn spawn(stdin_sender: CommandSender) -> Result<(), io::Error> {
	Builder::new()
		.name("stdin".to_owned())
		.spawn(move|| {
			let stdin = std::io::stdin();
			let stdin_buf = stdin.lock();
			
			for line_result in stdin_buf.lines() {
				match line_result {
					Ok(line) => {
						stdin_sender.send(&line);
					},
					Err(e) => {
						error!("Error: {}", e);
						break;
					},
				};
			}
		})?;
	
	Ok(())
}
