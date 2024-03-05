use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
	#[serde(default)]
	pub text_style: String,
	pub formats: HashMap<String, String>
}