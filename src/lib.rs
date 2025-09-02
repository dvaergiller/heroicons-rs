#![doc = include_str!("../README.md")]

#![feature(doc_cfg)]

pub mod strings;

#[cfg(feature = "hypertext")]
#[doc(cfg(feature = "hypertext"))]
pub mod hypertext;
