#![doc = include_str!("../README.md")]

use std::fmt::{Display, Formatter};

#[cfg(feature = "hypertext")]
pub mod hypertext;

pub(crate) mod svg;

pub use crate::svg::ToSvg;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Icon<Name: IconName, Variant: IconVariant> {
    pub name: Name,
    pub variant: Variant,
    pub id: &'static str,
    pub class: &'static str,
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
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct Outline;
    impl super::IconVariant for Outline {}

    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct Solid;
    impl super::IconVariant for Solid {}

    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct Mini;
    impl super::IconVariant for Mini {}

    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct Micro;
    impl super::IconVariant for Micro {}
}

pub mod icon_name {
    include!(concat!(env!("OUT_DIR"), "/generated_icon_names.rs"));
}
