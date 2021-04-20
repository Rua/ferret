use crate::{
	common::input::{bind_axis, bind_button},
	doom::{change_map, cheats::give_all, load_game, new_game, save_game, take_screenshot},
	ShouldQuit,
};
use clap::{App, AppSettings, Arg, ArgMatches};
use legion::{Resources, World};

pub fn commands() -> Vec<(
	App<'static, 'static>,
	fn(&ArgMatches, &mut World, &mut Resources),
)> {
	vec![
		(
			App::new("bind_axis")
				.about("Bind an axis to an action")
				.setting(AppSettings::AllowLeadingHyphen)
				.arg(
					Arg::with_name("AXIS")
						.help("Axis to bind to")
						.empty_values(false)
						.required(true),
				)
				.arg(
					Arg::with_name("BINDING").help(
						"Action to bind to the axis\nLeave empty to display the current binding",
					),
				)
				.arg(
					Arg::with_name("SCALE")
						.help("Sensitivity factor to apply to the movement")
						.default_value("1.0"),
				),
			|matches, _world, resources| {
				bind_axis(
					matches.value_of("AXIS").unwrap(),
					matches.value_of("BINDING").unwrap_or(""),
					matches.value_of("SCALE").unwrap(),
					resources,
				);
			},
		),
		(
			App::new("bind_button")
				.about("Bind a button to an action")
				.setting(AppSettings::AllowLeadingHyphen)
				.arg(
					Arg::with_name("BUTTON")
						.help("Button to bind to")
						.empty_values(false)
						.required(true),
				)
				.arg(Arg::with_name("BINDING").help(
					"Action to bind to the button\nLeave empty to display the current binding",
				)),
			|matches, _world, resources| {
				bind_button(
					matches.value_of("BUTTON").unwrap(),
					matches.value_of("BINDING").unwrap_or(""),
					resources,
				);
			},
		),
		(
			App::new("change").about("Change to a new map").arg(
				Arg::with_name("MAP")
					.help("Map to change to")
					.empty_values(false)
					.required(true),
			),
			|matches, world, resources| {
				change_map(matches.value_of("MAP").unwrap(), world, resources);
			},
		),
		(
			App::new("load").about("Load a previously saved game").arg(
				Arg::with_name("NAME")
					.help("Name of the saved game to load")
					.empty_values(false)
					.required(true),
			),
			|matches, world, resources| {
				load_game(matches.value_of("NAME").unwrap(), world, resources);
			},
		),
		(
			App::new("idfa").about("[Cheat] Give all weapons, ammo and armor"),
			|_matches, world, resources| {
				give_all(world, resources, false);
			},
		),
		(
			App::new("idkfa").about("[Cheat] Give all weapons, ammo, armor and keys"),
			|_matches, world, resources| {
				give_all(world, resources, true);
			},
		),
		(
			App::new("new").about("Start a new game").arg(
				Arg::with_name("MAP")
					.help("Map to start the new game on")
					.empty_values(false)
					.required(true),
			),
			|matches, world, resources| {
				new_game(matches.value_of("MAP").unwrap(), world, resources);
			},
		),
		(
			App::new("quit").about("Quit Ferret"),
			|_matches, _world, resources| {
				resources.insert(ShouldQuit);
			},
		),
		(
			App::new("save").about("Save the current game").arg(
				Arg::with_name("NAME")
					.help("Name to save the game to")
					.empty_values(false)
					.required(true),
			),
			|matches, world, resources| {
				save_game(matches.value_of("NAME").unwrap(), world, resources);
			},
		),
		(
			App::new("screenshot").about("Take a screenshot"),
			|_matches, _world, resources| {
				take_screenshot(resources);
			},
		),
	]
}
