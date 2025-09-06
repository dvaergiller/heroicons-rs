mod generated_from_icon_impl;

#[derive(Clone, Debug, PartialEq)]
pub struct Svg {
    pub attrs: &'static [Attribute],
    pub children: &'static [SvgChild],
}

impl ToString for Svg {
    fn to_string(&self) -> String {
        let mut output = "<svg ".to_string();
        self.attrs.iter().for_each(|a| output.push_str(&a.to_string()));
        output.push_str(">");
        self.children.iter().for_each(|c| output.push_str(&c.to_string()));
        output.push_str("</svg>");
        output
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SvgChild {
    pub tag_name: &'static str,
    pub attrs: &'static [Attribute],
}

impl ToString for SvgChild {
    fn to_string(&self) -> String {
        let mut output = format!("<{}", self.tag_name);
        self.attrs
            .iter()
            .for_each(|a| output.push_str(&a.to_string()));
        output.push_str(" />");
        output
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Attribute(pub &'static str, pub &'static str);

impl ToString for Attribute {
    fn to_string(&self) -> String {
        let Attribute(name, value) = self;
        format!(" {name}=\"{value}\"")
    }
}
