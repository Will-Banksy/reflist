mod args;
mod utils;

use std::{io::Read, process::ExitCode};

use args::Args;
use biblatex::{Bibliography, Entry};
use clap::Parser;
use markdown::to_html;
use utils::get_field_names;

fn main() -> ExitCode {
	let mut args = Args::parse();

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

	let format_string = &args.format_string;

	let fields = get_field_names(format_string);

	let mut references = Vec::new();

	let mut bib_entries: Vec<Entry> = bib.into_iter().collect();
	bib_entries.sort_by_key(|entry| entry.get_as::<String>(&args.sort_by).unwrap());

	for entry in bib_entries {
		let mut reference = format_string.clone();
		for field in &fields {
			if let Ok(value) = entry.get_as::<String>(field) {
				reference = reference.replace(&format!("${{{}}}", field), &value); // &value.to_biblatex_string(true));
			} else {
				eprintln!("Warning: Entry {} in bibliography does not contain a value for required field {} (or it could not be parsed)", entry.key, field);
			}
		}

		references.push(reference);
	}

	// TODO: Print markdown representation of references?
	// for r in &references {
	// 	println!("- {r}");
	// }

	let references_list_str = references.iter_mut().map(|r| format!("- {r}")).collect::<Vec<_>>().join("\n");

	let mut references_list_html = to_html(&references_list_str);

	references_list_html = references_list_html.replace("<em>", "<span style=\"font-style:italic;\">");
	references_list_html = references_list_html.replace("</em>", "</span>");
	references_list_html = references_list_html.replace("<strong>", "<span style=\"font-weight:700;\">");
	references_list_html = references_list_html.replace("</strong>", "</span>");

	println!("{references_list_html}");

	// BUG: This won't necessarily work. I don't know why. On my machine at least, the markdown will get copied/pasted instead of the html - the html isn't available.
	//      A potential fix might be to use the wayland backend for arboard, but that uses the data-control protocol and *naturally* gnome's mutter wm doesn't
	//      implement that, intentionally afaik (FUCKING STUPID PIECES OF SHIT)
	// NOTE: For now, I'm going to just let it output html which can be piped into xclip (which does work)
	// match Clipboard::new() {
	// 	Ok(mut clipboard) => {
	// 		// TODO: references_list_str is in markdown format, NOT in plain text format. This should be fixed
	// 		if let Err(e) = clipboard.set_html(references_list_html, Some(references_list_str)) {
	// 			eprintln!("Warning: Could not write to clipboard: {e}");
	// 		}
	// 	},
	// 	Err(e) => {
	// 		eprintln!("Warning: Could not initialise clipboard - Is a clipboard available? {e}");
	// 	}
	// };

	// NOTE: This is some code using x11rb but I don't know what I'm doing
	// x11rb::atom_manager! {
	// 	pub Atoms: AtomCookies {
	// 		CLIPBOARD,
	// 		HTML: b"text/html",
	// 	}
	// }
	// let clipboard = Clipboard::new().unwrap();
	// clipboard.store(, , "<span style=\"font-weight:700;\">hello</span>");

	ExitCode::SUCCESS
}
