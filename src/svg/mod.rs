use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub struct Svg<'a> {
    pub attrs: Vec<Attribute<'a>>,
    pub children: &'static [SvgChild],
}

impl<'a> Svg<'a> {
    pub fn segments(&'a self) -> SvgSegments<'a> {
        let mut segments = SvgSegments::new();
        segments.push("<svg");
        self.attrs.iter().for_each(|attr| attr.push_segments(&mut segments));
        segments.push(">");
        self.children.iter().for_each(|ch| ch.push_segments(&mut segments));
        segments.push("</svg>");
        segments
    }
}

pub struct SvgSegments<'a> {
    segments: Vec<&'a str>,
}

impl<'a> Default for SvgSegments<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> SvgSegments<'a> {
    pub fn new() -> SvgSegments<'a> {
        SvgSegments { segments: Vec::with_capacity(256) }
    }

    pub fn push(&mut self, segment: &'a str) {
        self.segments.push(segment);
    }

    pub fn push_all<S: IntoIterator<Item = &'a str>>(&mut self, segments: S) {
        segments.into_iter().for_each(|s| self.segments.push(s));
    }

    pub fn render(&self) -> String {
        let mut output = String::new();
        self.render_to(&mut output);
        output
    }

    pub fn render_to(&self, output: &mut String) {
        let len = self.segments.iter().map(|s| s.len()).sum();
        output.reserve(len);
        self.segments.iter().for_each(|s| output.push_str(s));
    }
}

pub trait ToSvg {
    fn to_svg(&self) -> Svg<'_>;
}

impl Display for Svg<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.segments().render())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SvgChild {
    pub tag_name: &'static str,
    pub attrs: &'static [Attribute<'static>],
}

impl SvgChild {
    pub fn push_segments<'a>(&'a self, segments: &mut SvgSegments<'a>) {
        segments.push_all(["<", self.tag_name]);
        self.attrs.iter().for_each(|attr| attr.push_segments(segments));
        segments.push("/>");
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Attribute<'a>(&'a str, &'a str);

impl<'a> Attribute<'a> {
    pub fn push_segments(&self, segments: &mut SvgSegments<'a>) {
        let Attribute(name, value) = self;
        segments.push_all([" ", name, "=\"", value, "\""]);
    }
}

include!(concat!(env!("OUT_DIR"), "/generated_from_icon_impl.rs"));
