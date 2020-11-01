use crate::{
	common::input::{Axis, Bindings, Button, ButtonBinding, MouseAxis},
	doom::input::{BoolInput, FloatInput},
};
use winit::event::{MouseButton, VirtualKeyCode};

pub fn get_bindings() -> Bindings<BoolInput, FloatInput> {
	let mut bindings = Bindings::new();
	bindings.bind_button(
		Button::Mouse(MouseButton::Left),
		ButtonBinding::Bool(BoolInput::Attack),
	);
	bindings.bind_button(
		Button::Key(VirtualKeyCode::Key1),
		ButtonBinding::Bool(BoolInput::Weapon1),
	);
	bindings.bind_button(
		Button::Key(VirtualKeyCode::Key2),
		ButtonBinding::Bool(BoolInput::Weapon2),
	);
	bindings.bind_button(
		Button::Key(VirtualKeyCode::Key3),
		ButtonBinding::Bool(BoolInput::Weapon3),
	);
	bindings.bind_button(
		Button::Key(VirtualKeyCode::Key4),
		ButtonBinding::Bool(BoolInput::Weapon4),
	);
	bindings.bind_button(
		Button::Key(VirtualKeyCode::Key5),
		ButtonBinding::Bool(BoolInput::Weapon5),
	);
	bindings.bind_button(
		Button::Key(VirtualKeyCode::Key6),
		ButtonBinding::Bool(BoolInput::Weapon6),
	);
	bindings.bind_button(
		Button::Key(VirtualKeyCode::Key7),
		ButtonBinding::Bool(BoolInput::Weapon7),
	);
	bindings.bind_button(
		Button::Key(VirtualKeyCode::Space),
		ButtonBinding::Bool(BoolInput::Use),
	);
	bindings.bind_button(
		Button::Mouse(MouseButton::Middle),
		ButtonBinding::Bool(BoolInput::Use),
	);
	bindings.bind_button(
		Button::Key(VirtualKeyCode::LShift),
		ButtonBinding::Bool(BoolInput::Walk),
	);
	bindings.bind_button(
		Button::Key(VirtualKeyCode::RShift),
		ButtonBinding::Bool(BoolInput::Walk),
	);
	bindings.bind_button(
		Button::Key(VirtualKeyCode::W),
		ButtonBinding::FloatPositive(FloatInput::Forward),
	);
	bindings.bind_button(
		Button::Key(VirtualKeyCode::S),
		ButtonBinding::FloatNegative(FloatInput::Forward),
	);
	bindings.bind_button(
		Button::Key(VirtualKeyCode::A),
		ButtonBinding::FloatPositive(FloatInput::Strafe),
	);
	bindings.bind_button(
		Button::Key(VirtualKeyCode::D),
		ButtonBinding::FloatNegative(FloatInput::Strafe),
	);
	bindings.bind_axis(Axis::Mouse(MouseAxis::X), FloatInput::Yaw, 3.0);
	bindings.bind_axis(Axis::Mouse(MouseAxis::Y), FloatInput::Pitch, 3.0);

	bindings
}
