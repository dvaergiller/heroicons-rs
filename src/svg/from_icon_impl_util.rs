use super::Attribute;

pub fn optional_attrs<'a>(
    attrs: &[(&'a str, Option<&'a str>)],
) -> Vec<Attribute<'a>> {
    attrs
        .iter()
        .filter_map(|(name, optional_value)| {
            optional_value.map(|value| Attribute(name, value))
        })
        .collect::<Vec<Attribute<'a>>>()
}
