use regex::{Captures, Regex};
use std::borrow::Cow;
use std::collections::hash_map::{Entry, HashMap};
use std::error::Error;
use std::string::String;
use std::sync::{Arc, mpsc};
use std::sync::mpsc::{Receiver, Sender};

pub struct CommandDispatcher {
	receiver: Receiver<Vec<String>>,
	sender: CommandSender,
}

impl CommandDispatcher {
	/*pub fn register_variable<T: FromStr + ToString + 'static>(&mut self, name: &str, initvalue: T) -> Rc<ConsoleVariable<T>> {
		let var = Rc::new(ConsoleVariable::<T> {
			name: name.to_owned(),
			value: RefCell::new(initvalue),
		});
		
		let weak = Rc::downgrade(&var) as Weak<ConsoleVariableT>;
		self.variables.insert(name.to_owned(), weak);
		
		var
	}*/
	
	pub fn push(&self, command: &str) {
		self.sender.push(command)
	}
	
	pub fn next(&mut self, block: bool) -> Option<Vec<String>> {
		if block {
			self.receiver.iter().next()
		} else {
			self.receiver.try_iter().next()
		}
	}
	
	pub fn print_commands(&self) {
		self.sender.print_commands()
	}
	
	/*pub fn print_variables(&self) {
		let names = self.variables.keys().collect::<Vec<&String>>();
		
		for name in names {
			info!("{}", name);
		}
	}*/
}

pub struct CommandSender {
	all_commands: Arc<HashMap<String, Vec<usize>>>,
	senders: Vec<Sender<Vec<String>>>,
}

impl CommandSender {
	pub fn push(&self, command: &str) {
		for args in tokenize(command).unwrap().split(|tok| tok == ";") {
			let command = match self.all_commands.get(&args[0]) {
				Some(val) => val,
				None => {
					error!("Command not found: {}", args[0]);
					return
				}
			};
			
			for sender_id in command {
				self.senders[*sender_id].send(args.to_vec()).ok();
			}
		}
	}
	
	pub fn print_commands(&self) {
		let mut names = self.all_commands.keys().collect::<Vec<&String>>();
		names.sort();
		
		for name in names {
			info!("{}", name);
		}
	}
}

pub struct CommandList<T> {
	commands: HashMap<String, Command<T>>,
}

impl<T> CommandList<T> {
	pub fn new() -> CommandList<T> {
		CommandList {
			commands: HashMap::new(),
		}
	}
	
	pub fn add<F: Fn(&mut T, Vec<String>) + Sync + 'static>(mut self, name: &str, func: F) -> CommandList<T> {
		self.commands.insert(name.to_owned(), Command {
			func: Box::new(func),
		});
		
		self
	}
	
	pub fn keys(&self) -> Vec<&String> {
		self.commands.keys().collect::<Vec<_>>()
	}
	
	pub fn execute(&self, args: Vec<String>, system: &mut T) {
		match self.commands.get(&args[0]) {
			Some(val) => val.call(system, args),
			None => debug!("Received invalid command: {}", args[0]),
		}
	}
}

pub struct CommandUnion {
	all_commands: Arc<HashMap<String, Vec<usize>>>,
	receivers: Vec<Option<Receiver<Vec<String>>>>,
	senders: Vec<Sender<Vec<String>>>,
}

impl CommandUnion {
	pub fn new() -> CommandUnion {
		CommandUnion {
			all_commands: Arc::new(HashMap::new()),
			receivers: Vec::new(),
			senders: Vec::new(),
		}
	}
	
	pub fn add_commands(&mut self, commands: Vec<&String>) {
		for name in commands {
			match Arc::get_mut(&mut self.all_commands).unwrap().entry(name.clone()) {
				Entry::Occupied(mut e) => {
					e.get_mut().push(self.senders.len());
				},
				Entry::Vacant(e) => {
					e.insert(vec![self.senders.len()]);
				},
			}
		}
		
		let (sender, receiver) = mpsc::channel();
		self.receivers.push(Some(receiver));
		self.senders.push(sender);
	}
	
	pub fn make_sender(&self) -> CommandSender {
		CommandSender {
			all_commands: self.all_commands.clone(),
			senders: self.senders.clone(),
		}
	}
	
	pub fn make_dispatcher(&mut self, index: usize) -> CommandDispatcher {
		CommandDispatcher {
			receiver: self.receivers[index].take().unwrap(),
			sender: CommandSender {
				all_commands: self.all_commands.clone(),
				senders: self.senders.clone(),
			},
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
}

/*struct ConsoleVariable<T> {
	name: String,
	value: RefCell<T>,
}

impl<T> ConsoleVariable<T> {
	pub fn value(&self) -> Ref<T> {
		self.value.borrow()
	}
	
	pub fn set_value(&self, newvalue: T) {
		self.value.replace(newvalue);
	}
}

trait ConsoleVariableT {
	fn print_value_str(&self);
	fn set_value_str(&self, newvalue: &str);
}

impl<T: FromStr + ToString> ConsoleVariableT for ConsoleVariable<T> {
	fn print_value_str(&self) {
		info!("\"{}\" = \"{}\"", self.name, self.value.borrow().to_string());
		//if let Some(var) = self.upgrade() {
	}
	
	fn set_value_str(&self, newvalue: &str) {
		if let Ok(value) = newvalue.parse::<T>() {
			self.set_value(value);
		}
		//if let Some(var) = self.upgrade() {
		// TODO: print message if parse fails
	}
}
*/

pub fn tokenize(mut text: &str) -> Result<Vec<String>, Box<dyn Error>> {
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
	
	while text.len() > 0 {
		if let Some(mat) = RE_SPACE.find(text) {
			text = &text[mat.end()..];
		} else if let Some(mat) = RE_UNQUOTED.find(text) {
			tokens.push(String::from(&text[..mat.end()]));
			text = &text[mat.end()..];
		} else if let Some(mat) = RE_QUOTED.find(text) {
			let unescaped = RE_ESCAPE.replace_all(&text[1..mat.end() - 1], |caps: &Captures<'_>| {
				String::from(match &caps[0] {
					r#"\\"# => r#"\"#,
					r#"\""# => r#"""#,
					_ => unreachable!(),
				})
			});
			tokens.push(String::from(unescaped));
			text = &text[mat.end()..];
		} else if text.starts_with("\"") {
			return Err(Box::from(format!("unclosed quoted string: \"{}", text)));
		} else if let Some(mat) = RE_SEPARATOR.find(text) {
			// Ignore separator at the end of the string
			if mat.end() < text.len() {
				tokens.push(String::from(";"));
			}
			
			text = &text[mat.end()..];
		} else if let Some(mat) = RE_CMT_LINE.find(text) {
			if mat.end() == text.len() {
				text = &text[mat.end()..];  // Closed by end of string
			} else {
				text = &text[mat.end() - 1..];  // Leave the newline
			}
		} else if let Some(mat) = RE_CMT_BLOCK.find(text) {
			text = &text[mat.end()..];
		} else if text.starts_with("/*") {
			return Err(Box::from(format!("unclosed multiline comment: {}", text)));
		} else {
			println!("{:?}", tokens);
			return Err(Box::from(format!(
				"unexpected character: {}",
				text.chars().next().unwrap()
			)));
		}
	}
	
	Ok(tokens)
}

pub fn quote_escape(text: &str) -> Cow<'_, str> {
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
