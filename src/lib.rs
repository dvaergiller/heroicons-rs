#![doc = include_str!("../README.md")]

#![feature(doc_cfg)]

mod generated_icon_names;
pub use generated_icon_names::IconName;

mod generated_from_icon_impl;

#[cfg(feature = "hypertext")]
#[doc(cfg(feature = "hypertext"))]
pub mod hypertext;

#[derive(Clone, Debug)]
pub struct Icon {
    pub name: IconName,
    pub variant: Option<Variant>,

    // Need better support for optional fields for this
    // pub id: Option<String>,
    // pub class: Option<String>,
    // pub style: Option<String>,
}

#[derive(Clone, Copy, Debug)]
pub enum Variant {
    Outline,
    Solid,
    Mini,
    Micro,
}

impl Default for Variant {
    fn default() -> Variant {
        Variant::Outline
    }
}

pub struct Svg {
    pub attrs: &'static [(&'static str, &'static str)],
    pub children: &'static [SvgChild],
}

pub struct SvgChild {
    pub tag_name: &'static str,
    pub attrs: &'static [(&'static str, &'static str)],
}
