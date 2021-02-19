use crate::{
	doom::{change_map, load_game, new_game, save_game},
	ShouldQuit,
};
use clap::{App, Arg, ArgMatches};
use legion::{Resources, World};

pub fn commands() -> Vec<(App<'static>, fn(&ArgMatches, &mut World, &mut Resources))> {
	vec![
		(
			App::new("change")
				.about("Change to a new map")
				.arg(Arg::new("MAP").about("Map to change to").required(true)),
			|matches, world, resources| {
				change_map(matches.value_of("MAP").unwrap(), world, resources);
			},
		),
		(
			App::new("load").about("Load a previously saved game").arg(
				Arg::new("NAME")
					.about("Name of the saved game to load")
					.required(true),
			),
			|matches, world, resources| {
				load_game(matches.value_of("NAME").unwrap(), world, resources);
			},
		),
		(
			App::new("new").about("Start a new game").arg(
				Arg::new("MAP")
					.about("Map to start the new game on")
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
				Arg::new("NAME")
					.about("Name to save the game to")
					.required(true),
			),
			|matches, world, resources| {
				save_game(matches.value_of("NAME").unwrap(), world, resources);
			},
		),
	]
}
