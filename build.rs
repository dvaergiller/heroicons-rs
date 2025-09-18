use proc_macro2::TokenStream;
use std::{
    ffi::OsStr,
    io::Write,
    path::{Path, PathBuf},
};

static ICONS_ROOT: &str = "heroicons/optimized";

struct IconFile {
    pub name: String,
    pub variant: String,
    pub svg_file: PathBuf,
}

fn main() {
    std::process::Command::new("git")
        .args(["submodule", "update"])
        .output()
        .unwrap();

    let icon_files: Vec<IconFile> = svg_files().map(to_svg_file).collect();
    icon_names::generate(&icon_files);
    from_icon_impl::generate(&icon_files);

    println!("cargo::rerun-if-changed=heroicons");
    println!("cargo::rerun-if-changed=build.rs");
}

mod parser {
    use std::{fs::read_to_string, path::Path};

    use pest::{Parser, iterators::Pairs};
    use pest_derive::Parser;

    pub struct Tag {
        pub name: String,
        pub attributes: Vec<(String, String)>,
        pub children: Vec<Tag>,
    }

    pub fn parse(file: &Path) -> Tag {
        let content = read_to_string(file).unwrap();
        let mut result = SVGParser::parse(Rule::tag, &content).unwrap();
        let svg_pair = result.next().unwrap().into_inner();
        into_tag(svg_pair)
    }

    fn into_tag(mut pair: Pairs<Rule>) -> Tag {
        let svg_identifier = pair.next().unwrap();
        assert_eq!(svg_identifier.as_rule(), Rule::identifier);
        let svg_attrs = pair.next().unwrap();
        assert_eq!(svg_attrs.as_rule(), Rule::attributes);
        let svg_children = pair.next().unwrap();
        assert_eq!(svg_children.as_rule(), Rule::children);

        let name = svg_identifier.as_str().to_owned();

        let attributes = svg_attrs
            .into_inner()
            .map(|attr| {
                assert_eq!(attr.as_rule(), Rule::attribute);
                let mut pairs = attr.into_inner();
                let attr_name = pairs.next().unwrap().as_str().to_owned();
                let attr_value = pairs
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .to_owned();
                (attr_name, attr_value)
            })
            .collect::<Vec<(String, String)>>();

        let children = svg_children
            .into_inner()
            .map(|pair| into_tag(pair.into_inner()))
            .collect();

        Tag { name, attributes, children }
    }

    #[derive(Parser)]
    #[grammar_inline = r#"
      tag = { "<" ~ PUSH(identifier) ~ attributes ~ children }
      children = { ("/>" ~ DROP) | (">" ~ tag* ~ "</" ~ POP ~ ">") }
      attributes = { attribute* }
      attribute = { identifier ~ "=" ~ string_value }
      identifier = @{ ('a'..'z' | 'A'..'Z' | "-")+ }
      string_value = { "\"" ~ string_contents ~ "\"" }
      string_contents = { (!"\"" ~ ANY)* }
      WHITESPACE = _{ " " | "\n" }
      "#]
    pub struct SVGParser;
}

// SVG search and indexing:

fn svg_files() -> impl Iterator<Item = PathBuf> {
    get_dir_entries(PathBuf::from(ICONS_ROOT))
        .flat_map(get_dir_entries)
        .flat_map(get_dir_entries)
        .filter(|svg_path| svg_path.extension() == Some(OsStr::new("svg")))
}

fn get_dir_entries(path: PathBuf) -> impl Iterator<Item = PathBuf> {
    std::fs::read_dir(path).unwrap().map(|entry| entry.unwrap().path())
}

fn to_svg_file(svg_file: PathBuf) -> IconFile {
    let name = path_to_icon_name(&svg_file);

    let components = svg_file
        .components()
        .rev()
        .map(|component| component.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<String>>();
    let type_string = &components[1];
    let size_string = &components[2];

    let variant = match (type_string.as_str(), size_string.as_str()) {
        ("outline", "24") => "Outline",
        ("solid", "24") => "Solid",
        ("solid", "20") => "Mini",
        ("solid", "16") => "Micro",
        _ => panic!("Unexpected folder structure: {svg_file:?}"),
    }
    .to_string();

    IconFile { name, variant, svg_file }
}

fn path_to_icon_name(path: &Path) -> String {
    let stem = path.file_stem().unwrap().to_string_lossy().into_owned();

    // Convert to PascalCase
    stem.split('-')
        .map(|word| {
            let (first, rest) = word.split_at(1);
            let mut fixed = first.to_ascii_uppercase();
            fixed.push_str(rest);
            fixed
        })
        .collect::<String>()
}

fn write_src_file(tokens: TokenStream, filename: &str) {
    let syntax_tree = syn::parse_file(&tokens.to_string()).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);
    let mut output_file = std::fs::File::create(filename).unwrap();
    output_file.write_all(formatted.as_bytes()).unwrap();
}

mod icon_names {
    use proc_macro2::TokenStream;
    use quote::{format_ident, quote};

    use crate::{IconFile, write_src_file};

    const NAMES_FILENAME: &str = "src/generated_icon_names.rs";

    pub fn generate(icons: &[IconFile]) {
        let mut names =
            icons.iter().map(|icon| &icon.name).collect::<Vec<&String>>();
        names.sort();
        names.dedup();
        write_src_file(icon_names_code(names), NAMES_FILENAME);
    }

    pub fn icon_names_code(enum_names: Vec<&String>) -> TokenStream {
        let names = enum_names.iter().map(|name| {
            let name_ident = format_ident!("{}", name);
            quote! {
                #[derive(Clone, Copy, Debug, Default, PartialEq)]
                pub struct #name_ident;
                impl IconName for #name_ident {}
            }
        });

        quote! {
            use crate::IconName;

            #(#names)*
        }
    }
}

mod from_icon_impl {
    use proc_macro2::{Span, TokenStream};
    use quote::{format_ident, quote};
    use syn::Ident;

    use crate::{
        IconFile,
        parser::{self, Tag},
        write_src_file,
    };

    const FROM_ICON_IMPL_FILENAME: &str = "src/svg/generated_from_icon_impl.rs";

    const COMMON_ATTRS: &[(&str, &str, &str)] = &[
        ("XMLNS", "xmlns", "http://www.w3.org/2000/svg"),
        ("FILL_CURRENT", "fill", "currentColor"),
        ("FILL_NONE", "fill", "none"),
        ("STROKE_CURRENT", "stroke", "currentColor"),
        ("ARIA_HIDDEN", "aria-hidden", "true"),
        ("DATA_SLOT", "data-slot", "icon"),
        ("VIEWBOX_24", "viewBox", "0 0 24 24"),
        ("VIEWBOX_20", "viewBox", "0 0 20 20"),
        ("VIEWBOX_16", "viewBox", "0 0 16 16"),
        ("FILL_EVENODD", "fill-rule", "evenodd"),
        ("CLIP_EVENODD", "clip-rule", "evenodd"),
    ];

    pub fn generate(icons: &[IconFile]) {
        write_src_file(tokens(icons), FROM_ICON_IMPL_FILENAME);
    }

    fn tokens(icons: &[IconFile]) -> TokenStream {
        let impl_tokens = icons.iter().map(svg_code);
        let common_attr_tokens =
            COMMON_ATTRS.iter().map(|(name, attr, value)| {
                let name_ident = Ident::new(name, Span::call_site());
                quote! {
                    const #name_ident: Attribute<'static> =
                        Attribute(#attr, #value);
                }
            });

        quote! {
            /// Generated code. Do not edit.
            use crate::Icon;
            use crate::svg::{Svg, SvgChild, Attribute, ToSvg};
            use crate::icon_name::*;
            use crate::icon_variant::*;
            #(#common_attr_tokens)*
            #(#impl_tokens)*
        }
    }

    fn svg_code(icon: &IconFile) -> TokenStream {
        let svg_tag = parser::parse(&icon.svg_file);
        assert_eq!(svg_tag.name, "svg");

        let name_ident = format_ident!("{}", icon.name);
        let variant_ident = format_ident!("{}", icon.variant);
        let attributes = svg_tag.attributes.into_iter().map(attr_code);
        let children = svg_tag.children.into_iter().map(child_code);

        quote! {
            impl ToSvg for Icon<#name_ident, #variant_ident> {
                fn to_svg<'a>(&'a self) -> Svg<'a> {
                    let mut attrs = vec![#(#attributes),*];
                    if !self.id.is_empty() {
                        attrs.push(Attribute("id", self.id))
                    }
                    if !self.class.is_empty() {
                        attrs.push(Attribute("class", self.class))
                    }
                    Svg {
                        attrs,
                        children: &[#(#children),*],
                    }
                }
            }
        }
    }

    fn child_code(child: Tag) -> Option<TokenStream> {
        let tag_name = child.name;
        let attrs = child.attributes.into_iter().map(attr_code);
        Some(quote! {
            SvgChild {
                tag_name: #tag_name,
                attrs: &[
                    #(#attrs),*
                ],

            }
        })
    }

    fn attr_code((attribute, value): (String, String)) -> TokenStream {
        let common_attr = COMMON_ATTRS
            .iter()
            .find(|(_, attr, val)| attr == &attribute && val == &value);
        match common_attr {
            Some((name, _, _)) => {
                let ident = Ident::new(name, Span::call_site());
                quote! {
                    #ident
                }
            }
            None => quote! {
                Attribute(#attribute, #value)
            },
        }
    }
}
