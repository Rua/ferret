/*lazy_static! {
	static ref COMMANDS: HashMap<String, ConsoleCommand<Input>> = ConsoleCommandBuilder::new()
		.add("+forward", |s: &mut Input, args: Vec<String>| s.button_forward.press())
		.add("-forward", |s: &mut Input, args: Vec<String>| s.button_forward.release())
		.add("+back"   , |s: &mut Input, args: Vec<String>| s.button_back.press())
		.add("-back"   , |s: &mut Input, args: Vec<String>| s.button_back.release())
		.add("+left"   , |s: &mut Input, args: Vec<String>| s.button_left.press())
		.add("-left"   , |s: &mut Input, args: Vec<String>| s.button_left.release())
		.add("+right"  , |s: &mut Input, args: Vec<String>| s.button_right.press())
		.add("-right"  , |s: &mut Input, args: Vec<String>| s.button_right.release())
		.add("+attack" , |s: &mut Input, args: Vec<String>| s.button_attack.press())
		.add("-attack" , |s: &mut Input, args: Vec<String>| s.button_attack.release())
		.add("+use"    , |s: &mut Input, args: Vec<String>| s.button_use.press())
		.add("-use"    , |s: &mut Input, args: Vec<String>| s.button_use.release())
		.finish();
}*/

pub struct Input {
	button_forward: Button,
	button_back: Button,
	button_left: Button,
	button_right: Button,
	button_attack: Button,
	button_use: Button,
}

impl Input {
	pub fn init() -> Input {
		Input {
			button_forward: Button::new(),
			button_back: Button::new(),
			button_left: Button::new(),
			button_right: Button::new(),
			button_attack: Button::new(),
			button_use: Button::new(),
		}
	}

	/*	pub fn process(&mut self) {
		while let Ok(args) = self.receiver.try_recv() {
			if let Some(command) = COMMANDS.get(&args[0]) {
				command.call(self, args);
			}
		}
	}*/
}

struct Button {
	state: bool,
}

impl Button {
	fn new() -> Button {
		Button { state: false }
	}

	fn press(&mut self) {
		self.state = true;
	}

	fn release(&mut self) {
		self.state = false;
	}
}
