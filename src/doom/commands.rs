use crate::{
	common::commands::tokenize,
	doom::{change_map, load_game, new_game, save_game},
};
use clap::{App, AppSettings, Arg};
use crossbeam_channel::Receiver;
use legion::{Resources, World};
use std::sync::atomic::{AtomicBool, Ordering};

pub fn execute_commands<'a>(
	command_receiver: Receiver<String>,
	should_quit: &'a AtomicBool,
) -> impl FnMut(&mut World, &mut Resources) + 'a {
	const MAIN_TEMPLATE: &'static str = "{subcommands}";
	const SUBCOMMAND_TEMPLATE: &'static str = "{usage}\n{about}\n\n{all-args}";
	const SUBCOMMAND_TEMPLATE_NOARGS: &'static str = "{usage}\n{about}";

	let mut app = App::new("")
		.help_template(MAIN_TEMPLATE)
		.global_setting(AppSettings::DisableHelpFlags)
		.global_setting(AppSettings::DisableVersion)
		.setting(AppSettings::NoBinaryName)
		.subcommand(
			App::new("change")
				.help_template(SUBCOMMAND_TEMPLATE)
				.about("Change to a new map")
				.arg(Arg::new("MAP").about("Map to change to").required(true)),
		)
		.subcommand(
			App::new("load")
				.help_template(SUBCOMMAND_TEMPLATE)
				.about("Load a previously saved game")
				.arg(
					Arg::new("NAME")
						.about("Name of the saved game to load")
						.required(true),
				),
		)
		.subcommand(
			App::new("new")
				.help_template(SUBCOMMAND_TEMPLATE)
				.about("Start a new game")
				.arg(
					Arg::new("MAP")
						.about("Map to start the new game on")
						.required(true),
				),
		)
		.subcommand(
			App::new("quit")
				.help_template(SUBCOMMAND_TEMPLATE_NOARGS)
				.about("Quit Ferret"),
		)
		.subcommand(
			App::new("save")
				.help_template(SUBCOMMAND_TEMPLATE)
				.about("Save the current game")
				.arg(
					Arg::new("NAME")
						.about("Name to save the game to")
						.required(true),
				),
		);

	move |world, resources| {
		while let Some(command) = command_receiver.try_iter().next() {
			// Split into tokens
			let tokens = match tokenize(&command) {
				Ok(tokens) => tokens,
				Err(e) => {
					log::error!("Invalid syntax: {}", e);
					continue;
				}
			};

			// Split further into subcommands
			for args in tokens.split(|tok| tok == ";") {
				let matches = match app.try_get_matches_from_mut(args) {
					Ok(m) => m,
					Err(e) => {
						log::info!("{}", e);
						continue;
					}
				};

				match matches.subcommand() {
					Some(("change", matches)) => {
						change_map(matches.value_of("MAP").unwrap(), world, resources);
					}
					Some(("load", matches)) => {
						load_game(matches.value_of("NAME").unwrap(), world, resources);
					}
					Some(("new", matches)) => {
						new_game(matches.value_of("MAP").unwrap(), world, resources);
					}
					Some(("quit", _matches)) => {
						should_quit.store(true, Ordering::Relaxed);
					}
					Some(("save", matches)) => {
						save_game(matches.value_of("NAME").unwrap(), world, resources);
					}
					_ => {
						log::error!("Unknown command: {}", args[0]);
					}
				}
			}
		}
	}
}
