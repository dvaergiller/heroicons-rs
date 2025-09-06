/// Generated code. Do not edit.
use crate::{Icon, IconName, Variant, Svg, SvgChild};
impl From<Icon> for Svg {
    fn from(icon: Icon) -> Svg {
        (&icon).into()
    }
}
impl From<&Icon> for Svg {
    fn from(icon: &Icon) -> Svg {
        match (icon.name, icon.variant.unwrap_or_default()) {
            (IconName::Bars3CenterLeft, Variant::Micro) => {
                Svg {
                    attrs: &[
                        ("viewBox", "0 0 16 16"),
                        ("xmlns", "http://www.w3.org/2000/svg"),
                        ("fill", "currentColor"),
                        ("data-slot", "icon"),
                        ("aria-hidden", "true"),
                    ],
                    children: &[
                        SvgChild {
                            tag_name: "path",
                            attrs: &[
                                ("clip-rule", "evenodd"),
                                ("fill-rule", "evenodd"),
                                (
                                    "d",
                                    "M2 3.75A.75.75 0 0 1 2.75 3h10.5a.75.75 0 0 1 0 1.5H2.75A.75.75 0 0 1 2 3.75ZM2 8a.75.75 0 0 1 .75-.75h4.5a.75.75 0 0 1 0 1.5h-4.5A.75.75 0 0 1 2 8Zm0 4.25a.75.75 0 0 1 .75-.75h10.5a.75.75 0 0 1 0 1.5H2.75a.75.75 0 0 1-.75-.75Z",
                                ),
                            ],
                        },
                    ],
                }
            }
            (IconName::StopCircle, Variant::Solid) => {
                Svg {
                    attrs: &[
                        ("viewBox", "0 0 24 24"),
                        ("xmlns", "http://www.w3.org/2000/svg"),
                        ("aria-hidden", "true"),
                        ("data-slot", "icon"),
                        ("fill", "currentColor"),
                    ],
                    children: &[
                        SvgChild {
                            tag_name: "path",
                            attrs: &[
                                ("fill-rule", "evenodd"),
                                ("clip-rule", "evenodd"),
                                (
                                    "d",
                                    "M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12Zm6-2.438c0-.724.588-1.312 1.313-1.312h4.874c.725 0 1.313.588 1.313 1.313v4.874c0 .725-.588 1.313-1.313 1.313H9.564a1.312 1.312 0 0 1-1.313-1.313V9.564Z",
                                ),
                            ],
                        },
                    ],
                }
            }
            (IconName::Underline, Variant::Solid) => {
                Svg {
                    attrs: &[
                        ("aria-hidden", "true"),
                        ("xmlns", "http://www.w3.org/2000/svg"),
                        ("fill", "currentColor"),
                        ("data-slot", "icon"),
                        ("viewBox", "0 0 24 24"),
                    ],
                    children: &[
                        SvgChild {
                            tag_name: "path",
                            attrs: &[
                                (
                                    "d",
                                    "M5.995 2.994a.75.75 0 0 1 .75.75v7.5a5.25 5.25 0 1 0 10.5 0v-7.5a.75.75 0 0 1 1.5 0v7.5a6.75 6.75 0 1 1-13.5 0v-7.5a.75.75 0 0 1 .75-.75Zm-3 17.252a.75.75 0 0 1 .75-.75h16.5a.75.75 0 0 1 0 1.5h-16.5a.75.75 0 0 1-.75-.75Z",
                                ),
                                ("fill-rule", "evenodd"),
                                ("clip-rule", "evenodd"),
                            ],
                        },
                    ],
                }
            }
            _ => {
                Svg::from(
                    &Icon {
                        name: IconName::QuestionMarkCircle,
                        variant: None,
                        id: None,
                        class: None,
                        style: None,
                    },
                )
            }
        }
    }
}
