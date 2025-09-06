use crate::{
    Icon,
    svg::{Attribute, Svg, SvgChild},
};
use hypertext::{Buffer, Renderable};

mod hypertext_elements;

impl Renderable for Icon {
    fn render_to(&self, buffer: &mut Buffer) {
        let icon_svg: Svg = self.into();
        let buf_str = buffer.dangerously_get_string();
        buf_str.push_str("<svg");
        icon_svg
            .attrs
            .iter()
            .for_each(|attr| format_attr(attr, buf_str));
        buf_str.push('>');
        icon_svg
            .children
            .iter()
            .for_each(|child| format_child(child, buf_str));
        buf_str.push_str("</svg>");
    }
}

fn format_attr(&Attribute(name, val): &Attribute, buf_str: &mut String) {
    buf_str.push_str(&format!(" {name}=\"{val}\""));
}

fn format_child(child: &SvgChild, buf_str: &mut String) {
    buf_str.push_str(&format!("<{}", child.tag_name));
    child
        .attrs
        .iter()
        .for_each(|attr| format_attr(attr, buf_str));
    buf_str.push_str(" />");
}
