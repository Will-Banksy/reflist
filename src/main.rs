mod args;
mod utils;
mod config;

use std::{collections::HashMap, env, io::{stdin, Read}, process::{self, ExitCode}};

use arboard::{Clipboard, SetExtLinux};
use args::Args;
use biblatex::{Bibliography, Entry};
use clap::Parser;
use markdown::to_html;
use utils::get_field_names;

use crate::config::Config;

const CLIPBOARD_DAEMONIZE_ARG: &'static str = "__internal_daemonize";

fn main() -> ExitCode {
	// So this is basically taken from the arboard daemonize example, since on linux the clipboard contents isn't kept unless the process that
	// is providing the clipboard contents is alive. There should be a way to offload this responsibility to a clipboard manager but this
	// seems less effort for now
	#[cfg(target_os = "linux")]
	if env::args().nth(1).as_deref() == Some(CLIPBOARD_DAEMONIZE_ARG) {
		if let Ok(mut clipboard) = Clipboard::new() {
			let mut content = String::new();
			if let Err(_) = stdin().read_to_string(&mut content) {
				eprintln!("[Daemon] Error: Stdin is not UTF-8 compatible or stdin could not be read");
				return ExitCode::FAILURE;
			}

			if env::args().nth(2).is_some() {
				let html = env::args().nth(2).unwrap();
				let alt_text = env::args().nth(3);
				if let Err(_) = clipboard.set().wait().html(html, alt_text) {
					eprintln!("[Daemon] Error: Failed to set clipboard contents");
					return ExitCode::FAILURE;
				}
			} else {
				return ExitCode::FAILURE;
			}
		} else {
			return ExitCode::FAILURE;
		}

		return ExitCode::SUCCESS;
	}

	let mut args = Args::parse();

	let config = {
		let mut config_str = String::new();
		if let Ok(_) = args.config_file.read_to_string(&mut config_str) {
			match toml::from_str::<Config>(&config_str) {
				Ok(config) => config,
				Err(e) => {
					eprintln!("Error: Config file {} is not correctly formatted: {e}", args.config_file.path());
					return ExitCode::FAILURE;
				}
			}
		} else {
			eprintln!("Error: Config file {} could not be read", args.config_file.path());
			return ExitCode::FAILURE;
		}
	};

	println!("Config: {:?}", config);

	let mut bibtext = String::new();
	if let Err(e) = args.file.read_to_string(&mut bibtext) {
		eprintln!("Error: Input file could not be read: {e}");
	}

	let bib = match Bibliography::parse(&bibtext) {
		Ok(bib) => bib,
		Err(e) => {
			eprintln!("Error: Input file could not be parsed as bibtex: {e}");
			return ExitCode::FAILURE;
		}
	};

	let format_fields: HashMap<String, Vec<String>> = config.formats.iter().map(|(entry_type, fstr)| (entry_type.clone(), get_field_names(fstr))).collect();

	let mut references = Vec::new();

	let mut bib_entries: Vec<Entry> = bib.into_iter().collect();
	bib_entries.sort_by_key(|entry| entry.get_as::<String>(&args.sort_by).unwrap());

	for entry in bib_entries {
		let entry_type = entry.entry_type.to_string();
		if let Some(mut reference) = config.formats.get(&entry_type).cloned() {
			for field in &format_fields[&entry_type] {
				if let Ok(value) = entry.get_as::<String>(field) {
					reference = reference.replace(&format!("${{{}}}", field), &value); // &value.to_biblatex_string(true));
				} else {
					eprintln!("Warning: Entry {} in bibliography does not contain a value for required field {} (or it could not be parsed)", entry.key, field);
				}
			}
			references.push(reference);
		} else {
			eprintln!("Warning: Entry type {} does not have a reference format - omitting", entry.entry_type.to_string())
		}
	}

	// TODO: Print markdown representation of references? Perhaps add arg for output format?
	// for r in &references {
	// 	println!("- {r}");
	// }

	let references_list_str = references.join("\n");

	let mut references_list_html = to_html(&references_list_str);

	references_list_html = references_list_html.replace("<em>", "<span style=\"font-style:italic;\">");
	references_list_html = references_list_html.replace("</em>", "</span>");
	references_list_html = references_list_html.replace("<strong>", "<span style=\"font-weight:700;\">");
	references_list_html = references_list_html.replace("</strong>", "</span>");
	if !config.text_style.is_empty() {
		references_list_html = format!("<span style=\"{}\">{}</span>", config.text_style, references_list_html);
	}

	println!("{references_list_html}");

	// TODO: For providing a plain-text alternative to the html, we need a plain text version of the references. Perhaps just remove html tags from the html?

	if args.copy {
		if cfg!(target_os = "linux") {
			if let Ok(this_exe) = env::current_exe() {
				match process::Command::new(this_exe)
					.arg(CLIPBOARD_DAEMONIZE_ARG)
					.arg(references_list_html)
					.arg(references_list_str)
					.stdin(process::Stdio::null())
					.stdout(process::Stdio::null())
					.stderr(process::Stdio::inherit())
					.current_dir("/")
					.spawn() {
					Ok(_phandle) => {
						// eprintln!("Info: Clipboard daemon started");
					},
					Err(e) => {
						eprintln!("Error: Failed to start clipboard daemon: {e}");
						return ExitCode::FAILURE;
					}
				}
			} else {
				eprintln!("Error: Failed to start clipboard daemon: Failed to get path to executable");
				return ExitCode::FAILURE;
			}
		} else {
			match Clipboard::new() {
				Ok(mut clipboard) => {
					if let Err(e) = clipboard.set().wait().html(references_list_html, Some(references_list_str)) {
						eprintln!("Error: Could not write to clipboard: {e}");
						return ExitCode::FAILURE;
					}
				},
				Err(e) => {
					eprintln!("Error: Could not initialise clipboard - Is a clipboard available? {e}");
					return ExitCode::FAILURE;
				}
			};
		}
	}

	ExitCode::SUCCESS
}
