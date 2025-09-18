use crate::{
    Icon, IconName, IconVariant,
    svg::{Svg, ToSvg},
};
use hypertext::{Buffer, Renderable};

mod hypertext_elements;

impl<Name, Variant> Renderable for Icon<Name, Variant>
where
    Name: IconName + Copy,
    Variant: IconVariant + Copy,
    Self: ToSvg,
{
    fn render_to(&self, buffer: &mut Buffer) {
        let icon_svg: Svg = (*self).to_svg();
        let segments = icon_svg.segments();
        let buf_str = buffer.dangerously_get_string();
        segments.render_to(buf_str);
    }
}
