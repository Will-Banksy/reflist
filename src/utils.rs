use unicode_segmentation::UnicodeSegmentation;

pub fn get_field_names(format_string: impl AsRef<str>) -> Vec<String> {
	let format_string = format_string.as_ref();

	let mut fields = Vec::new();
		let mut sb = String::new();

		let mut state = 0;
		const STATE_NONE: u8 = 0;
		const STATE_INIT: u8 = 1;
		const STATE_FIELDNAME: u8 = 2;

		for gc in format_string.graphemes(true) {
			match state {
				STATE_NONE => {
					if gc == "$" {
						state = STATE_INIT;
					} else {
						state = STATE_NONE;
					}
				}
				STATE_INIT => {
					if gc == "{" {
						state = STATE_FIELDNAME;
					} else {
						state = STATE_NONE;
					}
				}
				STATE_FIELDNAME => {
					if gc == "}" {
						fields.push(sb.clone().to_lowercase());
						sb.clear();
						state = STATE_NONE;

						continue;
					}
					sb.push_str(gc);
				}
				_ => {

				}
			}
		}

		fields
}

// NOTE: Is it worth getting this working to remove the markdown-rs dependency?
// /// Just does some basic markdown -> html replacements
// /// | -------- | --------------- |
// /// | Markdown | Replacement     |
// /// | -------- | --------------- |
// /// | *t*      | <i>t</i>        |
// /// | **t**    | <b>t</b>        |
// /// | ***t***  | <i><b>t</b></i> |
// /// | -------- | --------------- |
// fn md_to_html(md: impl AsRef<str>) -> String {
// 	let md = md.as_ref();

// 	let mut sb = String::new();

// 	let mut escaped = false;
// 	let mut num_stars = 0;
// 	let mut looking_for_stars_num = 0;
// 	let mut amongst_stars = false;

// 	for gc in md.graphemes(true) {
// 		if escaped {
// 			match gc {
// 				"*" => {
// 					sb.push('*');
// 				}
// 				_ => {
// 					sb.push_str(gc);
// 				}
// 			}
// 			escaped = false;
// 		} else {
// 			match gc {
// 				"*" => {
// 					if amongst_stars {
// 						num_stars += 1;
// 					} else if num_stars > 0 {
// 						looking_for_stars_num = num_stars;
// 					}

// 					amongst_stars = true;
// 				}
// 				_ => {
// 					amongst_stars = false;
// 				}
// 			}
// 		}
// 	}

// 	sb
// }