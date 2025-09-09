use crate::{
    Icon, IconName, IconVariant,
    svg::{Attribute, IntoSvg, Svg, SvgChild},
};
use hypertext::{Buffer, Renderable};

mod hypertext_elements;

impl<Name, Variant> Renderable for Icon<Name, Variant>
where
    Name: IconName + Copy,
    Variant: IconVariant + Copy,
    Self: IntoSvg,
{
    fn render_to(&self, buffer: &mut Buffer) {
        let icon_svg: Svg = (*self).into_svg();
        let buf_str = buffer.dangerously_get_string();
        buf_str.push_str("<svg");
        icon_svg.attrs.iter().for_each(|attr| format_attr(attr, buf_str));
        buf_str.push('>');
        icon_svg.children.iter().for_each(|child| format_child(child, buf_str));
        buf_str.push_str("</svg>");
    }
}

fn format_attr(&Attribute(name, val): &Attribute, buf_str: &mut String) {
    [" ", name, "=\"", val, "\""].iter().for_each(|s| buf_str.push_str(s));
}

fn format_child(child: &SvgChild, buf_str: &mut String) {
    buf_str.push('<');
    buf_str.push_str(child.tag_name);
    child.attrs.iter().for_each(|attr| format_attr(attr, buf_str));
    buf_str.push_str(" />");
}
