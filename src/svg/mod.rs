use std::fmt::{self, Display, Formatter};

mod generated_from_icon_impl;

#[derive(Clone, Debug, PartialEq)]
pub struct Svg {
    pub attrs: &'static [Attribute],
    pub children: &'static [SvgChild],
}

impl Display for Svg {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut output = "<svg".to_string();
        self.attrs.iter().for_each(|a| output.push_str(&a.to_string()));
        output.push('>');
        self.children.iter().for_each(|c| output.push_str(&c.to_string()));
        output.push_str("</svg>");
        f.write_str(&output)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SvgChild {
    pub tag_name: &'static str,
    pub attrs: &'static [Attribute],
}

impl Display for SvgChild {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut output = format!("<{}", self.tag_name);
        self.attrs.iter().for_each(|a| output.push_str(&a.to_string()));
        output.push_str("/>");
        f.write_str(&output)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Attribute(pub &'static str, pub &'static str);

impl Display for Attribute {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let Attribute(name, value) = self;
        f.write_fmt(format_args!(" {name}=\"{value}\""))
    }
}
