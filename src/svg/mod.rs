use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub struct Svg<'a> {
    pub attrs: Vec<Attribute<'a>>,
    pub children: &'static [SvgChild],
}

impl<'a> Svg<'a> {
    pub fn render(&self) -> String {
        let mut buffer = String::with_capacity(512);
        self.render_to(&mut buffer);
        buffer
    }

    pub fn render_to(&self, buffer: &mut String) {
        buffer.push_str("<svg");
        self.attrs.iter().for_each(|attr| attr.push_segments(buffer));
        buffer.push('>');
        self.children.iter().for_each(|ch| ch.push_segments(buffer));
        buffer.push_str("</svg>");
    }
}

pub trait ToSvg {
    fn to_svg(&self) -> Svg<'_>;
}

impl Display for Svg<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.render())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SvgChild {
    pub tag_name: &'static str,
    pub attrs: &'static [Attribute<'static>],
}

impl SvgChild {
    pub fn push_segments(&self, buffer: &mut String) {
        buffer.push('<');
        buffer.push_str(self.tag_name);
        self.attrs.iter().for_each(|attr| attr.push_segments(buffer));
        buffer.push_str("/>");
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Attribute<'a>(&'a str, &'a str);

impl<'a> Attribute<'a> {
    pub fn push_segments(&self, buffer: &mut String) {
        let &Attribute(name, value) = self;

        buffer.push(' ');
        buffer.push_str(name);
        buffer.push_str("=\"");

        if name == "id" || name == "class" {
            html_escape::encode_double_quoted_attribute_to_string(
                value, buffer,
            );
        } else {
            buffer.push_str(value);
        }

        buffer.push('\"');
    }
}

include!(concat!(env!("OUT_DIR"), "/generated_from_icon_impl.rs"));
