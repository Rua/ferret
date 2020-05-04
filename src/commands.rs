use anyhow::{bail, Context};
use crossbeam_channel::{Receiver, Sender};
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::{io::BufRead, thread::Builder};

pub fn init() -> anyhow::Result<(Sender<String>, Receiver<String>)> {
	let (sender, receiver) = crossbeam_channel::unbounded();
	let sender2 = sender.clone();

	// Spawns a thread to read commands from stdin asynchronously
	Builder::new()
		.name("stdin".to_owned())
		.spawn(move || {
			let stdin = std::io::stdin();
			let stdin_buf = stdin.lock();

			for line_result in stdin_buf.lines() {
				match line_result {
					Ok(line) => {
						sender2.send(line).ok();
					}
					Err(e) => {
						log::error!("Error: {}", e);
						break;
					}
				};
			}
		})
		.context("Could not start stdin thread")?;

	Ok((sender, receiver))
}

/*pub struct CommandList<T> {
	commands: HashMap<String, Command<T>>,
}

impl<T> CommandList<T> {
	pub fn new() -> CommandList<T> {
		CommandList {
			commands: HashMap::new(),
		}
	}

	pub fn add<F: Fn(&mut T, Vec<String>) + Sync + 'static>(
		mut self,
		name: &str,
		func: F,
	) -> CommandList<T> {
		self.commands.insert(
			name.to_owned(),
			Command {
				func: Box::new(func),
			},
		);

		self
	}

	/*pub fn keys(&self) -> Vec<&String> {
		self.commands.keys().collect::<Vec<_>>()
	}*/

	pub fn execute(&self, args: Vec<String>, system: &mut T) {
		match self.commands.get(&args[0]) {
			Some(val) => val.call(system, args),
			None => debug!("Received invalid command: {}", args[0]),
		}
	}

	pub fn print_commands(&self) {
		let mut names = self.commands.keys().collect::<Vec<&String>>();
		names.sort();

		for name in names {
			info!("{}", name);
		}
	}
}

struct Command<T> {
	func: Box<dyn Fn(&mut T, Vec<String>) + Sync + 'static>,
}

impl<T> Command<T> {
	pub fn call(&self, system: &mut T, args: Vec<String>) {
		(self.func)(system, args);
	}
}*/

pub fn tokenize(mut text: &str) -> anyhow::Result<Vec<String>> {
	lazy_static! {
		// Whitespace, except newlines
		static ref RE_SPACE    : Regex = Regex::new(r#"^[^\S\n]+"#).unwrap();

		// C identifier or number literal
		static ref RE_UNQUOTED : Regex = Regex::new(r#"^[+-]?[.0-9A-Za-z_]+"#).unwrap();

		// Quoted string, with escapes
		static ref RE_QUOTED   : Regex = Regex::new(r#"^"(?:[^"\\]*(?:\\.)?)*""#).unwrap();

		// Newline or semicolon, also eats any whitespace and separators that follow
		static ref RE_SEPARATOR: Regex = Regex::new(r#"^[\n;][\s;]*"#).unwrap();

		// Line comment, starts with // or #
		static ref RE_CMT_LINE : Regex = Regex::new(r#"^(?://|#)[^\n]*(?:\n|$)"#).unwrap();

		// Block comment, matches lazily with *? so that it stops at the first "*/"
		static ref RE_CMT_BLOCK: Regex = Regex::new(r#"^/\*.*?\*/"#).unwrap();

		// Escape sequence in quoted string
		static ref RE_ESCAPE   : Regex = Regex::new(r#"\\[\\"]"#).unwrap();
	}

	let mut tokens = Vec::new();

	while !text.is_empty() {
		if let Some(mat) = RE_SPACE.find(text) {
			text = &text[mat.end()..];
		} else if let Some(mat) = RE_UNQUOTED.find(text) {
			tokens.push(String::from(&text[..mat.end()]));
			text = &text[mat.end()..];
		} else if let Some(mat) = RE_QUOTED.find(text) {
			let unescaped =
				RE_ESCAPE.replace_all(&text[1..mat.end() - 1], |caps: &Captures<'_>| {
					String::from(match &caps[0] {
						r#"\\"# => r#"\"#,
						r#"\""# => r#"""#,
						_ => unreachable!(),
					})
				});
			tokens.push(String::from(unescaped));
			text = &text[mat.end()..];
		} else if text.starts_with('\"') {
			bail!("unclosed quoted string: \"{}", text);
		} else if let Some(mat) = RE_SEPARATOR.find(text) {
			// Ignore separator at the end of the string
			if mat.end() < text.len() {
				tokens.push(String::from(";"));
			}

			text = &text[mat.end()..];
		} else if let Some(mat) = RE_CMT_LINE.find(text) {
			if mat.end() == text.len() {
				text = &text[mat.end()..]; // Closed by end of string
			} else {
				text = &text[mat.end() - 1..]; // Leave the newline
			}
		} else if let Some(mat) = RE_CMT_BLOCK.find(text) {
			text = &text[mat.end()..];
		} else if text.starts_with("/*") {
			bail!("unclosed multiline comment: {}", text);
		} else {
			bail!("unexpected character: {}", text.chars().next().unwrap());
		}
	}

	Ok(tokens)
}

/*pub fn quote_escape(text: &str) -> Cow<'_, str> {
	lazy_static! {
		// As above, but anchored to end of string as well
		static ref RE_UNQUOTED : Regex = Regex::new(r#"^[+-]?[.0-9A-Za-z_]+$"#).unwrap();

		// Characters that need escaping
		static ref RE_ESCAPE   : Regex = Regex::new(r#"[\\"]"#).unwrap();
	}

	if RE_UNQUOTED.is_match(text) {
		Cow::from(text)
	} else {
		Cow::from(format!("\"{}\"", RE_ESCAPE.replace_all(text, "\\$0")))
	}
}
*/
