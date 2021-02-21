use crate::{
	doom::{change_map, cheats::give_all, load_game, new_game, save_game, take_screenshot},
	ShouldQuit,
};
use clap::{App, Arg, ArgMatches};
use legion::{Resources, World};

pub fn commands() -> Vec<(
	App<'static, 'static>,
	fn(&ArgMatches, &mut World, &mut Resources),
)> {
	vec![
		(
			App::new("change").about("Change to a new map").arg(
				Arg::with_name("MAP")
					.help("Map to change to")
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
