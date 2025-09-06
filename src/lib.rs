#![doc = include_str!("../README.md")]

#![feature(doc_cfg)]

mod generated_icon_names;
pub use generated_icon_names::IconName;

#[cfg(feature = "hypertext")]
#[doc(cfg(feature = "hypertext"))]
pub mod hypertext;

pub(crate) mod svg;

#[derive(Clone, Debug, PartialEq)]
pub struct Icon {
    pub name: IconName,
    pub variant: Option<Variant>,

    // Need better support for optional fields for this
    // pub id: Option<String>,
    // pub class: Option<String>,
    // pub style: Option<String>,
}

impl ToString for Icon {
    fn to_string(&self) -> String {
        svg::Svg::from(self).to_string()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
