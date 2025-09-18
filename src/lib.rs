#![doc = include_str!("../README.md")]
#![feature(doc_cfg)]

mod generated_icon_names;
use std::fmt::{Display, Formatter};

#[cfg(feature = "hypertext")]
pub mod hypertext;

pub(crate) mod svg;

pub use crate::svg::ToSvg;

#[derive(Clone, Copy, Debug, Default)]
pub struct Icon<Name: IconName, Variant: IconVariant> {
    pub name: Name,
    pub variant: Variant,
    pub id: Option<&'static str>,
    pub class: Option<&'static str>,
}

impl<Name, Variant> Display for Icon<Name, Variant>
where
    Icon<Name, Variant>: svg::ToSvg,
    Name: IconName + Copy,
    Variant: IconVariant + Copy,
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        self.to_svg().fmt(f)
    }
}

pub trait IconName {}
pub trait IconVariant {}

pub mod icon_variant {
    #[derive(Clone, Copy, Debug, Default)]
    pub struct Outline;
    impl super::IconVariant for Outline {}

    #[derive(Clone, Copy, Debug, Default)]
    pub struct Solid;
    impl super::IconVariant for Solid {}

    #[derive(Clone, Copy, Debug, Default)]
    pub struct Mini;
    impl super::IconVariant for Mini {}

    #[derive(Clone, Copy, Debug, Default)]
    pub struct Micro;
    impl super::IconVariant for Micro {}
}

pub mod icon_name {
    pub use crate::generated_icon_names::*;
}
