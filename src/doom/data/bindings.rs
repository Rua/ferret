use crate::common::input::{Axis, Bindings, Button, ButtonBinding, MouseAxis};
use winit::event::{MouseButton, VirtualKeyCode};

pub fn get_bindings() -> Bindings {
	let mut bindings = Bindings::new();
	bindings.bind_button(
		Button::Mouse(MouseButton::Left),
		ButtonBinding::Bool("attack".into()),
	);
	bindings.bind_button(
		Button::Key(VirtualKeyCode::Key1),
		ButtonBinding::Bool("weapon1".into()),
	);
	bindings.bind_button(
		Button::Key(VirtualKeyCode::Key2),
		ButtonBinding::Bool("weapon2".into()),
	);
	bindings.bind_button(
		Button::Key(VirtualKeyCode::Key3),
		ButtonBinding::Bool("weapon3".into()),
	);
	bindings.bind_button(
		Button::Key(VirtualKeyCode::Key4),
		ButtonBinding::Bool("weapon4".into()),
	);
	bindings.bind_button(
		Button::Key(VirtualKeyCode::Key5),
		ButtonBinding::Bool("weapon5".into()),
	);
	bindings.bind_button(
		Button::Key(VirtualKeyCode::Key6),
		ButtonBinding::Bool("weapon6".into()),
	);
	bindings.bind_button(
		Button::Key(VirtualKeyCode::Key7),
		ButtonBinding::Bool("weapon7".into()),
	);
	bindings.bind_button(
		Button::Key(VirtualKeyCode::Space),
		ButtonBinding::Bool("use".into()),
	);
	bindings.bind_button(
		Button::Mouse(MouseButton::Middle),
		ButtonBinding::Bool("use".into()),
	);
	bindings.bind_button(
		Button::Key(VirtualKeyCode::LShift),
		ButtonBinding::Bool("walk".into()),
	);
	bindings.bind_button(
		Button::Key(VirtualKeyCode::RShift),
		ButtonBinding::Bool("walk".into()),
	);
	bindings.bind_button(
		Button::Key(VirtualKeyCode::W),
		ButtonBinding::FloatPositive("forward".into()),
	);
	bindings.bind_button(
		Button::Key(VirtualKeyCode::S),
		ButtonBinding::FloatNegative("forward".into()),
	);
	bindings.bind_button(
		Button::Key(VirtualKeyCode::A),
		ButtonBinding::FloatPositive("strafe".into()),
	);
	bindings.bind_button(
		Button::Key(VirtualKeyCode::D),
		ButtonBinding::FloatNegative("strafe".into()),
	);
	bindings.bind_axis(Axis::Mouse(MouseAxis::X), "yaw".into(), 3.0);
	bindings.bind_axis(Axis::Mouse(MouseAxis::Y), "pitch".into(), 3.0);

	bindings
}
