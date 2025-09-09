use crate::{
    Icon, IconName, IconVariant,
    svg::{IntoSvg, Svg},
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
        let segments = icon_svg.segments();
        let mut buf_str = buffer.dangerously_get_string();
        segments.render_to(&mut buf_str);
    }
}
