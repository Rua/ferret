use fnv::FnvHashMap;
use legion::{systems::ResourceSet, Resources, Write};
use serde::{de::value::BorrowedStrDeserializer, Deserialize};
use smallvec::SmallVec;
use std::{fmt::Debug, hash::Hash};
use winit::event::{
	DeviceEvent, ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent,
};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Button {
	Key(VirtualKeyCode),
	Mouse(MouseButton),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize)]
#[serde(untagged)]
pub enum Axis {
	Mouse(MouseAxis),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize)]
pub enum MouseAxis {
	X = 0,
	Y = 1,
}

#[derive(Clone, Debug, Default)]
pub struct InputState {
	pub bindings: Bindings,
	bools: FnvHashMap<&'static str, SmallVec<[Button; 3]>>,
	floats: FnvHashMap<&'static str, FloatState>,
	mouse_delta_enabled: bool,
}

impl InputState {
	pub fn new(
		bools: impl IntoIterator<Item = &'static str>,
		floats: impl IntoIterator<Item = &'static str>,
	) -> InputState {
		InputState {
			bindings: Bindings::new(),
			bools: bools.into_iter().map(|s| (s, SmallVec::new())).collect(),
			floats: floats
				.into_iter()
				.map(|s| (s, FloatState::default()))
				.collect(),
			mouse_delta_enabled: false,
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
						for (axis, delta) in std::array::IntoIter::new([
							(MouseAxis::X, delta.0),
							(MouseAxis::Y, delta.1),
						]) {
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
			let buttons = match binding {
				ButtonBinding::Bool(name) => Some(self.bools.get_mut(name.as_str()).unwrap()),
				ButtonBinding::FloatPositive(name) => {
					Some(&mut self.floats.get_mut(name.as_str()).unwrap().buttons_positive)
				}
				ButtonBinding::FloatNegative(name) => {
					Some(&mut self.floats.get_mut(name.as_str()).unwrap().buttons_negative)
				}
				_ => None,
			};

			buttons.map(|buttons| match state {
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
			});
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

#[derive(Clone, Debug)]
pub enum ButtonBinding {
	Bool(String),
	FloatPositive(String),
	FloatNegative(String),
	Command(String),
}

type AxisBinding = (String, f64);

impl Bindings {
	#[inline]
	pub fn new() -> Bindings {
		Bindings {
			button_bindings: FnvHashMap::default(),
			axis_bindings: FnvHashMap::default(),
		}
	}

	#[inline]
	pub fn bind_button(&mut self, button: Button, binding: ButtonBinding) {
		self.button_bindings.insert(button, binding);
	}

	#[inline]
	pub fn get_button(&self, button: Button) -> Option<&ButtonBinding> {
		self.button_bindings.get(&button)
	}

	#[inline]
	pub fn bind_axis(&mut self, axis: Axis, binding: AxisBinding) {
		self.axis_bindings.insert(axis, binding);
	}

	#[inline]
	pub fn get_axis(&self, axis: Axis) -> Option<&AxisBinding> {
		self.axis_bindings.get(&axis)
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
			Some(ButtonBinding::Bool(binding)) => {
				log::info!("{} is bound to: ={}", button, binding)
			}
			Some(ButtonBinding::FloatPositive(binding)) => {
				log::info!("{} is bound to: +{}", button, binding)
			}
			Some(ButtonBinding::FloatNegative(binding)) => {
				log::info!("{} is bound to: -{}", button, binding)
			}
			Some(ButtonBinding::Command(binding)) => {
				log::info!("{} is bound to: {}", button, binding)
			}
			None => log::info!("{} is not bound", button),
		}
	} else {
		let binding = match binding.chars().next() {
			Some('=') => ButtonBinding::Bool(binding[1..].into()),
			Some('+') => ButtonBinding::FloatPositive(binding[1..].into()),
			Some('-') => ButtonBinding::FloatNegative(binding[1..].into()),
			Some(_) => ButtonBinding::Command(binding.into()),
			None => unreachable!(),
		};
		// TODO release button from previous binding
		input_state.bindings.bind_button(button_val, binding);
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
