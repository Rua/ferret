use crate::{
	common::{assets::AssetStorage, dirs::config_dir},
	doom::{
		draw::FramebufferResizeEvent,
		ui::{UiHexFontText, UiParams, UiTransform},
	},
};
use anyhow::{bail, Context};
use clap::{App, AppSettings, ArgMatches};
use crossbeam_channel::{Receiver, Sender};
use legion::{
	systems::{ResourceSet, Runnable},
	IntoQuery, Read, Resources, SystemBuilder, World,
};
use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use std::{
	collections::HashMap,
	fs::File,
	io::{BufRead, BufReader, Read as _},
	thread::Builder,
};

pub fn init() -> anyhow::Result<(Sender<String>, Receiver<String>)> {
	let (sender, receiver) = crossbeam_channel::unbounded();
	let sender2 = sender.clone();

	// Spawns a thread to read commands from stdin asynchronously
	Builder::new()
		.name("stdin".to_owned())
		.spawn(move || {
			let stdin = std::io::stdin();
			let stdin_buf = stdin.lock();

			for line_result in stdin_buf.lines() {
				match line_result {
					Ok(line) => {
						sender2.send(line).ok();
					}
					Err(e) => {
						log::error!("Error: {}", e);
						break;
					}
				};
			}
		})
		.context("Could not start stdin thread")?;

	Ok((sender, receiver))
}

const MAIN_TEMPLATE: &'static str = "{subcommands}";
const SUBCOMMAND_TEMPLATE: &'static str = "{usage}\n{about}\n\n{all-args}";

pub fn execute_file(name: &str, resources: &Resources) {
	let mut path = config_dir();
	path.push(name);

	let result = File::open(&path)
		.and_then(|file| {
			let mut buf = String::new();
			BufReader::new(file).read_to_string(&mut buf)?;
			Ok(buf)
		})
		.map(|text| {
			let command_sender = <Read<Sender<String>>>::fetch(resources);
			command_sender.send(text).unwrap();
		});

	if let Err(e) = result {
		log::error!("Couldn't execute \"{}\": {}", path.display(), e);
	}
}

pub fn execute_commands<'a>(
	receiver: Receiver<String>,
	commands: Vec<(
		App<'static, 'static>,
		fn(&ArgMatches, &mut World, &mut Resources),
	)>,
) -> impl FnMut(&mut World, &mut Resources) + 'a {
	let mut app = Some(
		App::new("")
			.template(MAIN_TEMPLATE)
			.global_setting(AppSettings::DisableHelpFlags)
			.global_setting(AppSettings::DisableVersion)
			.global_setting(AppSettings::DontCollapseArgsInUsage)
			.setting(AppSettings::NoBinaryName),
	);

	let mut functions: HashMap<String, _> = HashMap::with_capacity(commands.len());

	for (subcommand, func) in commands.into_iter() {
		functions.insert(subcommand.get_name().into(), func);

		app = Some(
			app.take()
				.unwrap()
				.subcommand(subcommand.template(SUBCOMMAND_TEMPLATE)),
		);
	}

	let mut app = app.unwrap();

	move |world, resources| {
		while let Some(command) = receiver.try_iter().next() {
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
				let matches = match app.get_matches_from_safe_borrow(args) {
					Ok(m) => m,
					Err(e) => {
						if !e.use_stderr() {
							log::info!("{}", e.to_string().trim_end());
						} else if !functions.contains_key(&args[0]) {
							log::error!("Unknown command: \"{}\". Type \"help\" for a list of valid commands.", args[0]);
						} else {
							log::error!(
								"Invalid syntax for command. Type \"help {}\" for valid usage.",
								args[0]
							);
						}
						continue;
					}
				};

				if let (command, Some(matches)) = matches.subcommand() {
					functions[command](matches, world, resources);
				}
			}
		}
	}
}

fn tokenize(mut text: &str) -> anyhow::Result<Vec<String>> {
	// Whitespace, except newlines
	static RE_SPACE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^[^\S\n]+"#).unwrap());

	// C identifier or number literal
	static RE_UNQUOTED: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^[=+-]?[.0-9A-Za-z_]+"#).unwrap());

	// Quoted string, with escapes
	static RE_QUOTED: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^"(?:[^"\\]*(?:\\.)?)*""#).unwrap());

	// Newline or semicolon, also eats any whitespace and separators that follow
	static RE_SEPARATOR: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^[\n;][\s;]*"#).unwrap());

	// Line comment, starts with // or #
	static RE_CMT_LINE: Lazy<Regex> =
		Lazy::new(|| Regex::new(r#"^(?://|#)[^\n]*(?:\n|$)"#).unwrap());

	// Block comment, matches lazily with *? so that it stops at the first "*/"
	static RE_CMT_BLOCK: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^/\*.*?\*/"#).unwrap());

	// Escape sequence in quoted string
	static RE_ESCAPE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"\\[\\"]"#).unwrap());

	let mut tokens = Vec::new();

	while !text.is_empty() {
		if let Some(mat) = RE_SPACE.find(text) {
			text = &text[mat.end()..];
		} else if let Some(mat) = RE_UNQUOTED.find(text) {
			tokens.push(String::from(&text[..mat.end()]));
			text = &text[mat.end()..];
		} else if let Some(mat) = RE_QUOTED.find(text) {
			let unescaped =
				RE_ESCAPE.replace_all(&text[1..mat.end() - 1], |caps: &Captures<'_>| {
					String::from(match &caps[0] {
						r#"\\"# => r#"\"#,
						r#"\""# => r#"""#,
						_ => unreachable!(),
					})
				});
			tokens.push(String::from(unescaped));
			text = &text[mat.end()..];
		} else if text.starts_with('\"') {
			bail!("unclosed quoted string: \"{}", text);
		} else if let Some(mat) = RE_SEPARATOR.find(text) {
			// Ignore separator at the end of the string
			if mat.end() < text.len() {
				tokens.push(String::from(";"));
			}

			text = &text[mat.end()..];
		} else if let Some(mat) = RE_CMT_LINE.find(text) {
			if mat.end() == text.len() {
				text = &text[mat.end()..]; // Closed by end of string
			} else {
				text = &text[mat.end() - 1..]; // Leave the newline
			}
		} else if let Some(mat) = RE_CMT_BLOCK.find(text) {
			text = &text[mat.end()..];
		} else if text.starts_with("/*") {
			bail!("unclosed multiline comment: {}", text);
		} else {
			bail!("unexpected character: {}", text.chars().next().unwrap());
		}
	}

	Ok(tokens)
}

pub fn quote_escape(text: &str) -> String {
	// Characters that need escaping
	static RE_ESCAPE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"[\\"]"#).unwrap());
	format!("\"{}\"", RE_ESCAPE.replace_all(text, "\\$0"))
}

pub fn update_console(receiver: Receiver<String>) -> impl Runnable {
	SystemBuilder::new("update_console")
		.read_resource::<AssetStorage>()
		.read_resource::<UiParams>()
		.with_query(<(&UiTransform, &mut UiHexFontText)>::query())
		.build(move |_command_buffer, world, resources, query| {
			let (asset_storage, ui_params) = resources;
			let (ui_transform, console) = query.iter_mut(world).next().unwrap();
			let font = asset_storage.get(&console.font).unwrap();
			let size = ui_transform.size + ui_params.stretch(ui_transform.stretch);

			while let Some(mut text) = receiver.try_iter().next() {
				// If the last line doesn't end with a newline, add the new text onto it.
				if console
					.lines
					.last()
					.map_or(false, |last| !last.ends_with('\n'))
				{
					text = console.lines.pop().unwrap() + &text;
				}

				for line in font.wrap_lines(size[0], &text) {
					console.lines.push(line.into());
				}
			}
		})
}

pub fn check_resize_console() -> impl Runnable {
	SystemBuilder::new("check_resize_console")
		.read_resource::<AssetStorage>()
		.read_resource::<UiParams>()
		.with_query(<&FramebufferResizeEvent>::query())
		.with_query(<(&UiTransform, &mut UiHexFontText)>::query())
		.build(move |_command_buffer, world, resources, queries| {
			if queries.0.iter(world).next().is_none() {
				return;
			}

			let (asset_storage, ui_params) = resources;
			let (ui_transform, console) = queries.1.iter_mut(world).next().unwrap();
			let font = asset_storage.get(&console.font).unwrap();
			let size = ui_transform.size + ui_params.stretch(ui_transform.stretch);

			console.lines = font
				.wrap_lines(size[0], &console.lines.concat())
				.map(|s| s.to_owned())
				.collect();
		})
}
