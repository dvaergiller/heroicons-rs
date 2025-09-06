use proc_macro2::TokenStream;
use quote::{format_ident, quote};
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

    let icon_files = svg_files().map(to_svg_file).collect();
    icon_names::generate(&icon_files);
    from_icon_impl::generate(&icon_files);

    println!("cargo::rerun-if-changed=heroicons");
    println!("cargo::rerun-if-changed=build.rs");
}

// SVG search and indexing:

fn svg_files() -> impl Iterator<Item = PathBuf> {
    get_dir_entries(PathBuf::from(ICONS_ROOT))
        .flat_map(get_dir_entries)
        .flat_map(get_dir_entries)
        .filter(|svg_path| svg_path.extension() == Some(OsStr::new("svg")))
}

fn get_dir_entries(path: PathBuf) -> impl Iterator<Item = PathBuf> {
    std::fs::read_dir(path)
        .unwrap()
        .map(|entry| entry.unwrap().path())
}

fn to_svg_file(svg_file: PathBuf) -> IconFile {
    let name = to_icon_name(&svg_file);

    let components = svg_file
        .components()
        .rev()
        .map(|component| component.as_os_str().to_string_lossy().into_owned())
        .take(3)
        .collect::<Vec<String>>();
    let type_string = &components[1];
    let size_string = &components[2];

    let variant = match (type_string.as_str(), size_string.as_str()) {
        ("outline", "24") => "Outline",
        ("solid", "24") => "Solid",
        ("solid", "20") => "Mini",
        ("solid", "16") => "Micro",
        _ => panic!("Unexpected folder structure: {svg_file:?}"),
    }.to_string();

    IconFile{ name, variant, svg_file }
}

fn to_icon_name(path: &Path) -> String {
    let stem = path.file_stem()
        .unwrap()
        .to_string_lossy()
        .into_owned();

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

    use crate::{write_src_file, IconFile};

    const NAMES_FILENAME: &str = "src/generated_icon_names.rs";

    pub fn generate(icons: &Vec<IconFile>) {
        let mut names = icons
            .iter()
            .map(|icon| &icon.name)
            .collect::<Vec<&String>>();
        names.sort();
        names.dedup();
        write_src_file(icon_names_code(names), NAMES_FILENAME);
    }

    pub fn icon_names_code(enum_names: Vec<&String>) -> TokenStream {
        let names = enum_names.iter().map(|name| format_ident!("{}", name));
        quote! {
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub enum IconName {
                #(#names),*
            }
        }
    }
}

mod from_icon_impl {
    use std::{borrow::Cow, fs::read_to_string};

    use tl::{Node, ParserOptions};

    use super::*;

    const FROM_ICON_IMPL_FILENAME: &str = "src/svg/generated_from_icon_impl.rs";

    pub fn generate(icons: &Vec<IconFile>) {
        write_src_file(tokens(icons), FROM_ICON_IMPL_FILENAME);
    }

    fn tokens(icons: &Vec<IconFile>) -> TokenStream {
        let case_tokens = icons.iter().map(svg_code);
        quote! {
            /// Generated code. Do not edit.
            use crate::{Icon, IconName, Variant};
            use crate::svg::{Svg, SvgChild, Attribute};

            impl From<Icon> for Svg {
                fn from(icon: Icon) -> Svg {
                    (&icon).into()
                }
            }

            impl From<&Icon> for Svg {
                fn from(icon: &Icon) -> Svg {
                    match (icon.name, icon.variant.unwrap_or_default()) {
                        #(#case_tokens),*,
                        _ => Svg::from(&Icon {
                            name: IconName::QuestionMarkCircle,
                            variant: None,
                        })
                    }
                }
            }
        }
    }

    fn svg_code(icon: &IconFile) -> TokenStream {
        let content = read_to_string(&icon.svg_file).unwrap();
        let svg = tl::parse(&content.trim(), ParserOptions::default())
            .expect("Failed to parse icon file");
        let &[svg_handle] = svg.children() else {
            panic!("Multiple top-level elements in SVG file")
        };
        let svg_node = svg_handle.get(svg.parser()).unwrap().as_tag().unwrap();
        assert_eq!(svg_node.name(), "svg");

        let name_ident = format_ident!("{}", icon.name);
        let variant_ident = format_ident!("{}", icon.variant);
        let attributes = svg_node.attributes().iter().map(attr_code);
        let children = svg_node
            .children()
            .all(svg.parser())
            .iter()
            .filter_map(child_code);

        quote! {
            (IconName::#name_ident, Variant::#variant_ident) =>
                Svg {
                    attrs: &[#(#attributes),*],
                    children: &[#(#children),*],
                }
        }
    }

    fn child_code(child: &Node) -> Option<TokenStream> {
        let child_node = child.as_tag()?;
        let tag_name = child_node.name().as_utf8_str();
        let attrs = child_node.attributes().iter().map(attr_code);
        Some(quote! {
            SvgChild {
                tag_name: #tag_name,
                attrs: &[#(#attrs),*],

            }
        })
    }

    fn attr_code((attribute, opt_value): (Cow<'_, str>, Option<Cow<'_, str>>)) -> TokenStream {
        let value = opt_value.unwrap_or("true".into());
        quote! {
            Attribute(#attribute, #value)
        }
    }
}
