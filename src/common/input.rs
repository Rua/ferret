use fnv::FnvHashMap;
use legion::{systems::ResourceSet, Resources, Write};
use serde::{de::value::BorrowedStrDeserializer, Deserialize};
use std::{fmt::Debug, hash::Hash};
use winit::event::{
	DeviceEvent, ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent,
};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Button {
	Key(VirtualKeyCode),
	Mouse(MouseButton),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Axis {
	Mouse(MouseAxis),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MouseAxis {
	X,
	Y,
}

#[derive(Debug, Default)]
pub struct InputState {
	mouse_delta: [f64; 2],
	mouse_delta_enabled: bool,
	pressed_keys: Vec<VirtualKeyCode>,
	pressed_mouse_buttons: Vec<MouseButton>,

	bools: FnvHashMap<&'static str, bool>,
	floats: FnvHashMap<&'static str, f64>,
}

impl InputState {
	pub fn new(
		bools: impl IntoIterator<Item = &'static str>,
		floats: impl IntoIterator<Item = &'static str>,
	) -> InputState {
		InputState {
			mouse_delta: [0.0, 0.0],
			mouse_delta_enabled: false,
			pressed_keys: Vec::new(),
			pressed_mouse_buttons: Vec::new(),

			bools: bools.into_iter().map(|s| (s, false)).collect(),
			floats: floats.into_iter().map(|s| (s, 0.0)).collect(),
		}
	}

	pub fn reset(&mut self) {
		self.mouse_delta = [0.0, 0.0];
	}

	pub fn set_values(&mut self, bindings: &Bindings) {
		self.bools = self
			.bools
			.keys()
			.map(|&name| {
				let value =
					bindings
						.button_bindings
						.iter()
						.any(|(button, binding)| match binding {
							ButtonBinding::Bool(binding) => {
								binding == name && self.button_is_down(*button)
							}
							_ => false,
						});

				(name, value)
			})
			.collect();

		self.floats = self
			.floats
			.keys()
			.map(|&name| {
				let axis_value = bindings
					.axis_bindings
					.iter()
					.map(|(axis, (binding, scale))| {
						if *binding == name {
							match axis {
								Axis::Mouse(axis) => self.mouse_delta(*axis) * scale,
							}
						} else {
							0.0
						}
					})
					.sum::<f64>();

				let buttons_positive =
					bindings
						.button_bindings
						.iter()
						.any(|(button, binding)| match binding {
							ButtonBinding::FloatPositive(binding) => {
								binding == name && self.button_is_down(*button)
							}
							_ => false,
						}) as usize as f64;

				let buttons_negative =
					bindings
						.button_bindings
						.iter()
						.any(|(button, binding)| match binding {
							ButtonBinding::FloatNegative(binding) => {
								binding == name && self.button_is_down(*button)
							}
							_ => false,
						}) as usize as f64;

				(name, axis_value + (buttons_positive - buttons_negative))
			})
			.collect();
	}

	pub fn button_is_down(&self, button: Button) -> bool {
		match button {
			Button::Key(key) => self.pressed_keys.iter().any(|&k| k == key),
			Button::Mouse(mouse_button) => self
				.pressed_mouse_buttons
				.iter()
				.any(|&mb| mb == mouse_button),
		}
	}

	pub fn mouse_delta(&self, axis: MouseAxis) -> f64 {
		self.mouse_delta[axis as usize]
	}

	pub fn set_mouse_delta_enabled(&mut self, enabled: bool) {
		self.mouse_delta_enabled = enabled;

		if !enabled {
			self.mouse_delta = [0.0, 0.0];
		}
	}

	pub fn bool_value(&self, name: &str) -> bool {
		*self
			.bools
			.get(name)
			.unwrap_or_else(|| panic!("Invalid bool-valued input name: {}", name))
	}

	pub fn float_value(&self, name: &str) -> f64 {
		*self
			.floats
			.get(name)
			.unwrap_or_else(|| panic!("Invalid float-valued input name: {}", name))
	}

	pub fn process_event(&mut self, event: &Event<()>) {
		match event {
			Event::WindowEvent { event, .. } => match *event {
				WindowEvent::KeyboardInput {
					input:
						KeyboardInput {
							state: ElementState::Pressed,
							virtual_keycode: Some(key_code),
							..
						},
					..
				} => {
					if self.pressed_keys.iter().all(|&k| k != key_code) {
						self.pressed_keys.push(key_code);
					}
				}
				WindowEvent::KeyboardInput {
					input:
						KeyboardInput {
							state: ElementState::Released,
							virtual_keycode: Some(key_code),
							..
						},
					..
				} => {
					if let Some(i) = self.pressed_keys.iter().position(|&k| k == key_code) {
						self.pressed_keys.swap_remove(i);
					}
				}
				WindowEvent::MouseInput {
					state: ElementState::Pressed,
					button,
					..
				} => {
					if self.pressed_mouse_buttons.iter().all(|&b| b != button) {
						self.pressed_mouse_buttons.push(button);
					}
				}
				WindowEvent::MouseInput {
					state: ElementState::Released,
					button,
					..
				} => {
					if let Some(i) = self.pressed_mouse_buttons.iter().position(|&b| b == button) {
						self.pressed_mouse_buttons.swap_remove(i);
					}
				}
				WindowEvent::Focused(false) => {
					self.pressed_keys.clear();
					self.pressed_mouse_buttons.clear();
				}
				_ => {}
			},
			Event::DeviceEvent { event, .. } => match *event {
				DeviceEvent::MouseMotion { delta } => {
					if self.mouse_delta_enabled {
						self.mouse_delta[0] += delta.0;
						self.mouse_delta[1] += delta.1;
					}
				}
				_ => {}
			},
			_ => {}
		}
	}
}

#[derive(Debug, Default, Clone)]
pub struct Bindings {
	button_bindings: FnvHashMap<Button, ButtonBinding>,
	axis_bindings: FnvHashMap<Axis, (&'static str, f64)>,
}

#[derive(Clone, Debug)]
pub enum ButtonBinding {
	Bool(String),
	FloatPositive(String),
	FloatNegative(String),
	Command(String),
}

impl Bindings {
	pub fn new() -> Bindings {
		Bindings {
			button_bindings: FnvHashMap::default(),
			axis_bindings: FnvHashMap::default(),
		}
	}

	pub fn bind_button(&mut self, button: Button, binding: ButtonBinding) {
		self.button_bindings.insert(button, binding);
	}

	pub fn bind_axis(&mut self, axis: Axis, axis_binding: &'static str, scale: f64) {
		self.axis_bindings.insert(axis, (axis_binding, scale));
	}
}

pub fn bind(button: &str, binding: &str, resources: &mut Resources) {
	debug_assert!(!binding.is_empty());

	let result = if let Some(button) = button.strip_prefix("Mouse") {
		let deserializer = BorrowedStrDeserializer::<serde::de::value::Error>::new(button);
		MouseButton::deserialize(deserializer).map(Button::Mouse)
	} else {
		let deserializer = BorrowedStrDeserializer::<serde::de::value::Error>::new(button);
		VirtualKeyCode::deserialize(deserializer).map(Button::Key)
	};

	let button = match result {
		Ok(x) => x,
		Err(_) => {
			log::error!("Invalid button: {}", button);
			return;
		}
	};

	let binding = match binding.chars().next() {
		Some('=') => ButtonBinding::Bool(binding[1..].into()),
		Some('+') => ButtonBinding::FloatPositive(binding[1..].into()),
		Some('-') => ButtonBinding::FloatNegative(binding[1..].into()),
		Some(_) => ButtonBinding::Command(binding.into()),
		None => unreachable!(),
	};

	let mut bindings = <Write<Bindings>>::fetch_mut(resources);
	bindings.bind_button(button, binding);
}
