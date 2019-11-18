use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::{
	collections::{hash_map::Entry, HashMap},
	fmt::Debug,
	hash::Hash,
};
use winit::event::{
	DeviceEvent, ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Axis {
	Emulated { pos: Button, neg: Button },
	Mouse { axis: MouseAxis, scale: f64 },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum MouseAxis {
	X,
	Y,
}

#[derive(Debug, Default)]
pub struct InputState {
	mouse_delta: [f64; 2],
	pressed_keys: Vec<VirtualKeyCode>,
	pressed_mouse_buttons: Vec<MouseButton>,
}

impl InputState {
	pub fn new() -> InputState {
		InputState {
			mouse_delta: [0.0, 0.0],
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
					self.mouse_delta[0] += delta.0;
					self.mouse_delta[1] += delta.1;
				}
				_ => {}
			},
			_ => {}
		}
	}
}

#[derive(Derivative, Deserialize, Serialize)]
#[derivative(Debug(bound = ""), Default(bound = ""), Clone(bound = ""))]
pub struct Bindings<A: Clone + Debug + Hash + Eq, X: Clone + Debug + Hash + Eq> {
	actions: HashMap<A, Vec<Button>>,
	axes: HashMap<X, Axis>,
}

impl<A: Clone + Debug + Hash + Eq, X: Clone + Debug + Hash + Eq> Bindings<A, X> {
	pub fn new() -> Bindings<A, X> {
		Bindings {
			actions: HashMap::new(),
			axes: HashMap::new(),
		}
	}

	pub fn bind_action(&mut self, id: A, button: Button) {
		match self.actions.entry(id) {
			Entry::Occupied(mut entry) => {
				entry.get_mut().push(button);
			}
			Entry::Vacant(entry) => {
				entry.insert(vec![button]);
			}
		}
	}

	pub fn bind_axis(&mut self, id: X, axis: Axis) {
		self.axes.insert(id, axis);
	}

	pub fn action_is_down(&self, id: &A, input_state: &InputState) -> bool {
		self.actions
			.get(id)
			.map(|buttons| {
				buttons
					.iter()
					.any(|button| input_state.button_is_down(*button))
			})
			.unwrap_or(false)
	}

	pub fn axis_value(&self, id: &X, input_state: &InputState) -> f64 {
		self.axes
			.get(id)
			.map(|a| match *a {
				Axis::Emulated { pos, neg } => {
					(input_state.button_is_down(pos) as i32
						- input_state.button_is_down(neg) as i32) as f64
				}
				Axis::Mouse { axis, scale } => input_state.mouse_delta(axis) * scale,
			})
			.unwrap_or(0.0)
	}
}
