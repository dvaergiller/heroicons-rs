use std::fs::read_to_string;

use regex::Regex;

/// Removes some unneeded whitespace in input string and in contents read from
/// file to check that they are identical except for said whitespace.
pub fn equivalent_to_source(mut icon: String, source_path: &str) -> bool {
    let mut contents = read_to_string(source_path).unwrap();

    let space_after_tag = Regex::new(r">\s*").unwrap();
    contents = space_after_tag.replace_all(&contents, ">").to_string();
    let selfclose_spacing = Regex::new(r"\s*/>").unwrap();
    icon = selfclose_spacing.replace_all(&icon, "/>").to_string();

    icon == contents
}
