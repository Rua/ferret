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
