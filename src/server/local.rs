use std::any::Any;
use std::error::Error;
use std::fmt;
use std::io;
use std::mem;
use std::panic;
use std::panic::AssertUnwindSafe;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, mpsc};
use std::sync::mpsc::{Receiver, Sender, SendError};
use std::thread::{Builder, JoinHandle};

use server;

pub enum ControlMessage {
	Start,
	Stop,
	Quit,
}

pub struct LocalServer {
	has_panicked: Arc<AtomicBool>,
	has_quit: bool,
	sender: Sender<ControlMessage>,
	thread_handle: JoinHandle<()>,
}

impl LocalServer {
	pub fn new() -> io::Result<LocalServer> {
		let (sender, receiver) = mpsc::channel();
		let has_panicked = Arc::new(AtomicBool::new(false));
		let has_panicked2 = has_panicked.clone();
		
		let thread_handle = Builder::new()
			.name("server".to_owned())
			.spawn(move || {
				match panic::catch_unwind(AssertUnwindSafe(|| {
					server::server_main(receiver)
				})) {
					Ok(()) => debug!("Server thread terminated."),
					Err(err) => has_panicked2.store(true, Ordering::Relaxed),
				}
			})?;
		
		Ok(LocalServer {
			has_panicked,
			has_quit: false,
			sender,
			thread_handle,
		})
	}
	
	pub fn start(&mut self) -> Result<(), LocalServerError> {
		self.check_panic()?;
		self.sender.send(ControlMessage::Start).ok();
		Ok(())
	}
	
	pub fn stop(&mut self) -> Result<(), LocalServerError> {
		self.check_panic()?;
		self.sender.send(ControlMessage::Stop).ok();
		Ok(())
	}
	
	pub fn quit(&mut self) -> Result<(), LocalServerError> {
		self.check_panic()?;
		self.sender.send(ControlMessage::Quit).ok();
		self.has_quit = true;
		Ok(())
	}
	
	pub fn quit_and_wait(mut self) -> Result<(), LocalServerError> {
		let err = match self.quit() {
			Ok(_) => Ok(()),
			Err(LocalServerError::AlreadyQuit) => Ok(()),
			Err(err) => Err(err),
		};
		
		// We can't move out of `self.thread_handle`, so we have to replace it with rubbish first.
		// This means we have to forget `self` to avoid having it being dropped with an invalid
		// state.
		let thread_handle = mem::replace(&mut self.thread_handle, unsafe { mem::uninitialized() });
		
		let err2 = match thread_handle.join() {
			Ok(_) => Ok(()),
			Err(_) => Err(LocalServerError::ServerPanicked),
		};
		
		// See if the server panicked right before it quit.
		let err = err.or({
			if self.has_panicked.load(Ordering::Relaxed) {
				Err(LocalServerError::ServerPanicked)
			} else {
				err2
			}
		});
		
		mem::forget(self);
		err
	}
	
	pub fn check_panic(&mut self) -> Result<(), LocalServerError> {
		if self.has_panicked.load(Ordering::Relaxed) {
			self.has_quit = true;
			return Err(LocalServerError::ServerPanicked);
		} else if self.has_quit {
			return Err(LocalServerError::AlreadyQuit);
		}
		
		Ok(())
	}
}

impl Drop for LocalServer {
	fn drop(&mut self) {
		// Last chance to send a quit message to the server.
		// If even that fails, then
		match self.quit() {
			Ok(_) => (),
			Err(_) => error!("Could not quit server thread, detaching!"),
		}
	}
}

#[derive(Debug)]
pub enum LocalServerError {
	AlreadyQuit,
	ServerPanicked,
}

impl fmt::Display for LocalServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			LocalServerError::AlreadyQuit => {
				"a quit message was already sent to the server".fmt(f)
			},
			LocalServerError::ServerPanicked => {
				"the server panicked".fmt(f)
			}
		}
    }
}

impl Error for LocalServerError { }
