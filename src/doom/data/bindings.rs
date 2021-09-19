use crate::common::input::{Axis, Bindings, Button, MouseAxis};
use winit::event::{MouseButton, VirtualKeyCode};

pub fn default_bindings() -> Bindings {
	let mut bindings = Bindings::new();

	bindings.bind_axis(Axis::Mouse(MouseAxis::X), ("yaw".into(), 3.0));
	bindings.bind_axis(Axis::Mouse(MouseAxis::Y), ("pitch".into(), 3.0));

	bindings.bind_button(Button::Key(VirtualKeyCode::W), "+forward".into());
	bindings.bind_button(Button::Key(VirtualKeyCode::S), "-forward".into());
	bindings.bind_button(Button::Key(VirtualKeyCode::A), "+strafe".into());
	bindings.bind_button(Button::Key(VirtualKeyCode::D), "-strafe".into());
	bindings.bind_button(Button::Key(VirtualKeyCode::LShift), "=walk".into());
	bindings.bind_button(Button::Key(VirtualKeyCode::RShift), "=walk".into());

	bindings.bind_button(Button::Mouse(MouseButton::Left), "=attack".into());
	bindings.bind_button(Button::Key(VirtualKeyCode::Key1), "=weapon1".into());
	bindings.bind_button(Button::Key(VirtualKeyCode::Key2), "=weapon2".into());
	bindings.bind_button(Button::Key(VirtualKeyCode::Key3), "=weapon3".into());
	bindings.bind_button(Button::Key(VirtualKeyCode::Key4), "=weapon4".into());
	bindings.bind_button(Button::Key(VirtualKeyCode::Key5), "=weapon5".into());
	bindings.bind_button(Button::Key(VirtualKeyCode::Key6), "=weapon6".into());
	bindings.bind_button(Button::Key(VirtualKeyCode::Key7), "=weapon7".into());
	bindings.bind_button(Button::Key(VirtualKeyCode::Space), "=use".into());
	bindings.bind_button(Button::Mouse(MouseButton::Middle), "=use".into());

	bindings.bind_button(Button::Key(VirtualKeyCode::F6), "save quick".into());
	bindings.bind_button(Button::Key(VirtualKeyCode::F9), "load quick".into());
	bindings.bind_button(Button::Key(VirtualKeyCode::F10), "quit".into());
	bindings.bind_button(Button::Key(VirtualKeyCode::F11), "screenshot".into());

	bindings
}
