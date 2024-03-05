use clap::Parser;
use clio::Input;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
	/// The bibtex (.bib) input file
	#[clap(value_parser, default_value="-")]
	#[arg(short = 'i', long)]
	pub file: Input,
	/// The format string, where fields are inserted at ${field_name}, e.g. the author field would be inserted at ${author}
	#[arg(short, long)]
	pub format_string: String,
	/// The field_name of the field to sort references by
	#[arg(short, long, default_value = "author")]
	pub sort_by: String,
	/// Whether to automatically copy the html to clipboard, in html format (ready for pasting into a WYSIWYG editor)
	#[arg(short, long)]
	pub copy: bool
}