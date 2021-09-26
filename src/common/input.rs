use crate::common::console::quote_escape;
use crossbeam_channel::Sender;
use fnv::FnvHashMap;
use legion::{systems::ResourceSet, Resources, Write};
use serde::{de::value::BorrowedStrDeserializer, Deserialize, Serialize};
use smallvec::SmallVec;
use std::{
	fmt::{self, Debug, Display, Formatter},
	hash::Hash,
	io::{self, Write as IOWrite},
};
use winit::event::{
	DeviceEvent, ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent,
};

#[derive(Clone, Debug, Default)]
pub struct RepeatTracker {
	pressed: Vec<Button>,
}

impl RepeatTracker {
	#[inline]
	pub fn new() -> RepeatTracker {
		RepeatTracker {
			pressed: Vec::new(),
		}
	}

	pub fn is_repeat(&mut self, event: &Event<()>) -> bool {
		let (button, state) = match event {
			Event::WindowEvent { event, .. } => match *event {
				WindowEvent::KeyboardInput {
					input:
						KeyboardInput {
							state,
							virtual_keycode: Some(key_code),
							..
						},
					..
				} => (Button::Key(key_code), state),
				WindowEvent::MouseInput { state, button, .. } => (Button::Mouse(button), state),
				_ => return false,
			},
			_ => return false,
		};

		match state {
			ElementState::Pressed => {
				if self.pressed.iter().all(|&b| b != button) {
					self.pressed.push(button);
					false
				} else {
					true
				}
			}
			ElementState::Released => {
				if let Some(i) = self.pressed.iter().position(|&b| b == button) {
					self.pressed.swap_remove(i);
					false
				} else {
					true
				}
			}
		}
	}
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Button {
	Key(VirtualKeyCode),
	Mouse(MouseButton),
}

impl Display for Button {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Key(key) => {
				key.serialize(f)?;
			}
			Self::Mouse(button) => {
				f.write_str("Mouse")?;
				button.serialize(f)?;
			}
		}
		Ok(())
	}
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(untagged)]
pub enum Axis {
	Mouse(MouseAxis),
}

impl Display for Axis {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Mouse(axis) => write!(f, "Mouse{}", axis),
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub enum MouseAxis {
	X = 0,
	Y = 1,
}

impl Display for MouseAxis {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::X => write!(f, "X"),
			Self::Y => write!(f, "Y"),
		}
	}
}

#[derive(Clone, Debug)]
pub struct InputState {
	pub bindings: Bindings,
	bools: FnvHashMap<&'static str, SmallVec<[Button; 3]>>,
	floats: FnvHashMap<&'static str, FloatState>,
	mouse_delta_enabled: bool,
	command_sender: Sender<String>,
}

impl InputState {
	pub fn new(
		bools: impl IntoIterator<Item = &'static str>,
		floats: impl IntoIterator<Item = &'static str>,
		command_sender: Sender<String>,
	) -> InputState {
		InputState {
			bindings: Bindings::new(),
			bools: bools.into_iter().map(|s| (s, SmallVec::new())).collect(),
			floats: floats
				.into_iter()
				.map(|s| (s, FloatState::default()))
				.collect(),
			mouse_delta_enabled: false,
			command_sender,
		}
	}

	pub fn reset(&mut self) {
		for float_state in self.floats.values_mut() {
			float_state.mouse_delta = 0.0;
		}
	}

	pub fn set_mouse_delta_enabled(&mut self, enabled: bool) {
		self.mouse_delta_enabled = enabled;

		if !enabled {
			self.reset();
		}
	}

	pub fn bool_value(&self, name: &str) -> bool {
		!self
			.bools
			.get(name)
			.unwrap_or_else(|| panic!("Invalid bool-valued input name: {}", name))
			.is_empty()
	}

	pub fn float_value(&self, name: &str) -> f64 {
		self.floats
			.get(name)
			.unwrap_or_else(|| panic!("Invalid float-valued input name: {}", name))
			.value()
	}

	pub fn process_event(&mut self, event: &Event<()>) {
		match event {
			Event::WindowEvent { event, .. } => match *event {
				WindowEvent::KeyboardInput {
					input:
						KeyboardInput {
							state,
							virtual_keycode: Some(key_code),
							..
						},
					..
				} => {
					self.button_event(Button::Key(key_code), state);
				}
				WindowEvent::MouseInput { state, button, .. } => {
					self.button_event(Button::Mouse(button), state);
				}
				WindowEvent::Focused(false) => {
					for bool_value in self.bools.values_mut() {
						bool_value.clear();
					}

					for float_value in self.floats.values_mut() {
						float_value.buttons_positive.clear();
						float_value.buttons_negative.clear();
					}
				}
				_ => {}
			},
			Event::DeviceEvent { event, .. } => match *event {
				DeviceEvent::MouseMotion { delta } => {
					if self.mouse_delta_enabled {
						for (axis, delta) in [(MouseAxis::X, delta.0), (MouseAxis::Y, delta.1)] {
							self.delta_event(Axis::Mouse(axis), delta);
						}
					}
				}
				_ => {}
			},
			_ => {}
		}
	}

	fn button_event(&mut self, button: Button, state: ElementState) {
		if let Some(binding) = self.bindings.button_bindings.get(&button) {
			let buttons = match binding.chars().next().unwrap() {
				'=' => match self.bools.get_mut(&binding[1..]) {
					Some(x) => x,
					None => {
						log::warn!("Unknown bool-valued input name: {}", &binding[1..]);
						return;
					}
				},
				'+' => match self.floats.get_mut(&binding[1..]) {
					Some(x) => &mut x.buttons_positive,
					None => {
						log::warn!("Unknown float-valued input name: {}", &binding[1..]);
						return;
					}
				},
				'-' => match self.floats.get_mut(&binding[1..]) {
					Some(x) => &mut x.buttons_negative,
					None => {
						log::warn!("Unknown float-valued input name: {}", &binding[1..]);
						return;
					}
				},
				_ => {
					if state == ElementState::Pressed {
						self.command_sender.send(binding.clone()).ok();
					}

					return;
				}
			};

			match state {
				ElementState::Pressed => {
					if buttons.iter().all(|&b| b != button) {
						buttons.push(button);
					}
				}
				ElementState::Released => {
					if let Some(i) = buttons.iter().position(|&b| b == button) {
						buttons.swap_remove(i);
					}
				}
			}
		}
	}

	fn delta_event(&mut self, axis: Axis, delta: f64) {
		if let Some((name, scale)) = self.bindings.axis_bindings.get(&axis) {
			let mouse_delta = &mut self.floats.get_mut(name.as_str()).unwrap().mouse_delta;
			*mouse_delta += delta * scale;
		}
	}
}

#[derive(Clone, Debug, Default)]
struct FloatState {
	mouse_delta: f64,
	buttons_positive: SmallVec<[Button; 3]>,
	buttons_negative: SmallVec<[Button; 3]>,
}

impl FloatState {
	fn value(&self) -> f64 {
		let buttons_positive = (!self.buttons_positive.is_empty()) as usize as f64;
		let buttons_negative = (!self.buttons_negative.is_empty()) as usize as f64;
		self.mouse_delta + (buttons_positive - buttons_negative)
	}
}

#[derive(Debug, Default, Clone)]
pub struct Bindings {
	button_bindings: FnvHashMap<Button, ButtonBinding>,
	axis_bindings: FnvHashMap<Axis, AxisBinding>,
}

pub type ButtonBinding = String;
pub type AxisBinding = (String, f64);

impl Bindings {
	#[inline]
	pub fn new() -> Bindings {
		Bindings {
			button_bindings: FnvHashMap::default(),
			axis_bindings: FnvHashMap::default(),
		}
	}

	#[inline]
	pub fn is_empty(&self) -> bool {
		self.button_bindings.is_empty() && self.axis_bindings.is_empty()
	}

	#[inline]
	pub fn get_button(&self, button: Button) -> Option<&ButtonBinding> {
		self.button_bindings.get(&button)
	}

	#[inline]
	pub fn get_axis(&self, axis: Axis) -> Option<&AxisBinding> {
		self.axis_bindings.get(&axis)
	}

	#[inline]
	pub fn bind_button(&mut self, button: Button, binding: ButtonBinding) {
		debug_assert!(!binding.is_empty());
		self.button_bindings.insert(button, binding);
	}

	#[inline]
	pub fn bind_axis(&mut self, axis: Axis, binding: AxisBinding) {
		debug_assert!(!binding.0.is_empty());
		self.axis_bindings.insert(axis, binding);
	}

	pub fn write(&self, writer: &mut impl IOWrite) -> io::Result<()> {
		let mut commands = self
			.axis_bindings
			.iter()
			.map(|(axis, (binding, scale))| {
				vec![
					"bind_axis".into(),
					axis.to_string(),
					quote_escape(&binding.to_string()),
					scale.to_string(),
				]
			})
			.chain(self.button_bindings.iter().map(|(button, binding)| {
				vec![
					"bind_button".into(),
					button.to_string(),
					quote_escape(&binding.to_string()),
				]
			}))
			.collect::<Vec<_>>();
		commands.sort_unstable();

		for command in commands {
			writeln!(writer, "{}", command.join(" "))?;
		}
		Ok(())
	}
}

pub fn bind_button(button: &str, binding: &str, resources: &mut Resources) {
	let result = if let Some(button) = button.strip_prefix("Mouse") {
		let deserializer = BorrowedStrDeserializer::<serde::de::value::Error>::new(button);
		MouseButton::deserialize(deserializer).map(Button::Mouse)
	} else {
		let deserializer = BorrowedStrDeserializer::<serde::de::value::Error>::new(button);
		VirtualKeyCode::deserialize(deserializer).map(Button::Key)
	};

	let button_val = match result {
		Ok(x) => x,
		Err(_) => {
			log::error!("Invalid button: {}", button);
			return;
		}
	};

	let mut input_state = <Write<InputState>>::fetch_mut(resources);
	if binding.is_empty() {
		match input_state.bindings.get_button(button_val) {
			Some(binding) => log::info!("{} is bound to: {}", button, binding),
			None => log::info!("{} is not bound", button),
		}
	} else {
		// TODO release button from previous binding
		input_state.bindings.bind_button(button_val, binding.into());
	}
}

pub fn bind_axis(axis: &str, binding: &str, scale: &str, resources: &mut Resources) {
	let result = axis.strip_prefix("Mouse").and_then(|axis| {
		let deserializer = BorrowedStrDeserializer::<serde::de::value::Error>::new(axis);
		MouseAxis::deserialize(deserializer).map(Axis::Mouse).ok()
	});

	let axis_val = match result {
		Some(x) => x,
		None => {
			log::error!("Invalid axis: {}", axis);
			return;
		}
	};

	let mut input_state = <Write<InputState>>::fetch_mut(resources);
	if binding.is_empty() {
		match input_state.bindings.get_axis(axis_val) {
			Some((binding, scale)) => {
				log::info!("{} is bound to: {} * {}", axis, binding, scale)
			}
			None => log::info!("{} is not bound", axis),
		}
	} else {
		let scale = match scale.parse() {
			Ok(x) => x,
			Err(e) => {
				log::error!("Parse error: {}: {}", e, scale);
				return;
			}
		};
		input_state
			.bindings
			.bind_axis(axis_val, (binding.into(), scale));
	}
}
