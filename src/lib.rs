#![doc = include_str!("../README.md")]

#![feature(doc_cfg)]

mod generated_icon_names;
use std::fmt::{self, Display, Formatter};

pub use generated_icon_names::IconName;

#[cfg(feature = "hypertext")]
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

impl Display for Icon {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        svg::Svg::from(self).fmt(f)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Variant {
    #[default]
    Outline,
    Solid,
    Mini,
    Micro,
}
