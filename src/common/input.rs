use fnv::FnvHashMap;
use std::{fmt::Debug, hash::Hash};
use winit::event::{
	DeviceEvent, ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent,
};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Button {
	Key(VirtualKeyCode),
	Mouse(MouseButton),
}

impl From<VirtualKeyCode> for Button {
	fn from(keycode: VirtualKeyCode) -> Self {
		Button::Key(keycode)
	}
}

impl From<MouseButton> for Button {
	fn from(mouse_button: MouseButton) -> Self {
		Button::Mouse(mouse_button)
	}
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
}

impl InputState {
	pub fn new() -> InputState {
		InputState {
			mouse_delta: [0.0, 0.0],
			mouse_delta_enabled: false,
			pressed_keys: Vec::new(),
			pressed_mouse_buttons: Vec::new(),
		}
	}

	pub fn reset(&mut self) {
		self.mouse_delta = [0.0, 0.0];
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
pub struct Bindings<B: Clone + Debug + Hash + Eq, F: Clone + Debug + Hash + Eq> {
	button_bindings: FnvHashMap<Button, ButtonBinding<B, F>>,
	axis_bindings: FnvHashMap<Axis, (F, f64)>,
}

#[derive(Clone, Debug)]
pub enum ButtonBinding<B, F> {
	Bool(B),
	FloatPositive(F),
	FloatNegative(F),
}

impl<B: Clone + Debug + Hash + Eq, F: Clone + Debug + Hash + Eq> Bindings<B, F> {
	pub fn new() -> Bindings<B, F> {
		Bindings {
			button_bindings: FnvHashMap::default(),
			axis_bindings: FnvHashMap::default(),
		}
	}

	pub fn bind_button(&mut self, button: Button, binding: ButtonBinding<B, F>) {
		self.button_bindings.insert(button, binding);
	}

	pub fn bind_axis(&mut self, axis: Axis, axis_binding: F, scale: f64) {
		self.axis_bindings.insert(axis, (axis_binding, scale));
	}

	pub fn bool_value(&self, bool_input: &B, input_state: &InputState) -> bool {
		self.button_bindings
			.iter()
			.any(|(button, binding)| match binding {
				ButtonBinding::Bool(binding) => {
					binding == bool_input && input_state.button_is_down(*button)
				}
				_ => false,
			})
	}

	pub fn float_value(&self, float_input: &F, input_state: &InputState) -> f64 {
		let axis_value = self
			.axis_bindings
			.iter()
			.map(|(axis, (binding, scale))| {
				if binding == float_input {
					match axis {
						Axis::Mouse(axis) => input_state.mouse_delta(*axis) * scale,
					}
				} else {
					0.0
				}
			})
			.sum::<f64>();

		let buttons_positive = self
			.button_bindings
			.iter()
			.any(|(button, binding)| match binding {
				ButtonBinding::FloatPositive(binding) => {
					binding == float_input && input_state.button_is_down(*button)
				}
				_ => false,
			}) as usize as f64;

		let buttons_negative = self
			.button_bindings
			.iter()
			.any(|(button, binding)| match binding {
				ButtonBinding::FloatNegative(binding) => {
					binding == float_input && input_state.button_is_down(*button)
				}
				_ => false,
			}) as usize as f64;

		axis_value + (buttons_positive - buttons_negative)
	}
}
