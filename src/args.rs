use clap::Parser;
use clio::Input;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
	/// The bibtex (.bib) input file
	#[clap(value_parser, default_value="-")]
	#[arg(short = 'i', long)]
	pub file: Input,
	/// The path to the config file
	#[clap(value_parser, default_value="Reflist.toml")]
	#[arg(short = 'f', long)]
	pub config_file: Input,
	/// The field_name of the field to sort references by
	#[arg(short, long, default_value = "author")]
	pub sort_by: String,
	/// Whether to automatically copy the html to clipboard, in html format (ready for pasting into a WYSIWYG editor)
	#[arg(short, long)]
	pub copy: bool,
	/// Whether to report a summary of the reference counts
	#[arg(short = 'n', long)]
	pub counts: bool
}