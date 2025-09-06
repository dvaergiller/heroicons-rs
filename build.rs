use std::{io::Write, collections::HashMap, ffi::OsStr, path::{Path, PathBuf}};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

static ICONS_ROOT: &str = "heroicons/optimized";

type Name = String;
type Variant = String;

#[derive(Default)]
struct IconIndex(HashMap<(Name, Variant), PathBuf>);

fn main() {
    std::process::Command::new("git")
        .args(["submodule", "update"])
        .output()
        .unwrap();

    let mut icon_index = IconIndex::default();
    let files = svg_files();
    files.for_each(|path| insert_svg(path, &mut icon_index));
    icon_names::generate(&icon_index);
    from_icon_impl::generate(&icon_index);

    println!("cargo::rerun-if-changed=heroicons");
    println!("cargo::rerun-if-changed=build.rs");
}

/// SVG search and indexing:

fn svg_files() -> impl Iterator<Item = PathBuf> {
    get_dir_entries(PathBuf::from(ICONS_ROOT))
        .flat_map(|size_dir| get_dir_entries(size_dir))
        .flat_map(|format_dir| get_dir_entries(format_dir))
        .filter(|svg_path| svg_path.extension() == Some(OsStr::new("svg")))
}

fn get_dir_entries(path: PathBuf) -> impl Iterator<Item = PathBuf> {
    std::fs::read_dir(path)
        .unwrap()
        .map(|entry| entry.unwrap().path())
}

fn insert_svg(path: PathBuf, index: &mut IconIndex) {
    let name = to_icon_name(&path).unwrap();

    let components = path
        .components()
        .rev()
        .map(|component| {
            component.as_os_str().to_string_lossy().into_owned()
        })
        .collect::<Vec<String>>();
    let type_string = &components[1];
    let size_string = &components[2];

    let variant =
        match (type_string.as_str(), size_string.as_str()) {
            ("outline", "24") => "Outline",
            ("solid", "24") => "Solid",
            ("solid", "20") => "Mini",
            ("solid", "16") => "Micro",
            _ => panic!("Unexpected folder structure: {path:?}")
        };
    index.0.insert((name, variant.to_string()), path);
}

fn to_icon_name(path: &Path) -> Option<String> {
    let to_pascal_case = |path: String| {
        path.split('-')
            .map(|word| {
                let (first, rest) = word.split_at(1);
                let mut fixed = first.to_ascii_uppercase();
                fixed.push_str(rest);
                fixed
            })
            .collect::<String>()
    };

    if path.extension() == Some(OsStr::new("svg")) {
        path
            .file_stem()
            .map(|stem| stem.to_string_lossy().into_owned())
            .map(to_pascal_case)
    }
    else {
        None
    }
}

pub mod parser {
    use tl::{ParserOptions, VDom};

    pub fn parse<'a>(input: &'a str) -> VDom<'a> {
        tl::parse(&input, ParserOptions::default())
            .expect("Failed to parse icon file")
    }
}

mod icon_names {
    use super::*;

    const NAMES_FILENAME: &str = "src/generated_icon_names.rs";

    pub fn generate(index: &IconIndex) {
        let mut names = index.0
            .keys()
            .map(|(name, _)| name).collect::<Vec<&String>>();
        names.sort();
        names.dedup();
        let tokens = icon_names_code(names);
        let syntax_tree = syn::parse_file(&tokens.to_string()).unwrap();
        let formatted = prettyplease::unparse(&syntax_tree);
        let mut output_file = std::fs::File::create(NAMES_FILENAME).unwrap();
        output_file.write_all(formatted.as_bytes()).unwrap();
    }

    pub fn icon_names_code<'a>(enum_names: Vec<&'a String>) -> TokenStream {
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

    use tl::{Node, VDom};

    use super::*;

    const OUTPUT_FILENAME: &str = "src/generated_from_icon_impl.rs";

    pub fn generate<'a>(index: &IconIndex) {
        let tokens = tokens(index);
        let syntax_tree = syn::parse_file(&tokens.to_string()).unwrap();
        let formatted = prettyplease::unparse(&syntax_tree);
        let mut output_file = std::fs::File::create(OUTPUT_FILENAME).unwrap();
        output_file.write_all(formatted.as_bytes()).unwrap();
    }

    fn tokens(index: &IconIndex) -> TokenStream {
        let case_tokens = index.0
            .iter()
            .take(3)
            .map(|((name, variant), path)| impl_code(name, variant, path));
        quote! {
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

    fn impl_code(name: &Name, variant: &Variant, path: &PathBuf) -> TokenStream {
        let content = read_to_string(path).unwrap();
        let svg = parser::parse(content.trim());
        svg_code(name, variant, svg)
    }

    fn svg_code(name: &Name, variant: &Variant, svg: VDom) -> TokenStream {
        let &[svg_handle] = svg.children()
            else { panic!("Multiple top-level elements in SVG file") };
        let svg_node = svg_handle
            .get(svg.parser())
            .unwrap()
            .as_tag()
            .unwrap();
        assert_eq!(svg_node.name(), "svg");

        let name_ident = format_ident!("{name}");
        let variant_ident = format_ident!("{variant}");
        let attributes = svg_node.attributes().iter().map(attr_code).collect::<Vec<TokenStream>>();
        let children =
            svg_node
                .children()
                .all(svg.parser())
                .iter()
                .filter_map(child_elem_code);
        quote! {
            (IconName::#name_ident, Variant::#variant_ident) =>
                Svg {
                    attrs: &[#(#attributes),*],
                    children: &[#(#children),*],
                }
        }
    }

    fn child_elem_code(child: &Node) -> Option<TokenStream> {
        let child_node = child.as_tag()?;
        let tag_name = child_node.name().as_utf8_str();
        let attrs = child_node.attributes().iter().map(attr_code).collect::<Vec<TokenStream>>();
        Some(quote! {
            SvgChild {
                tag_name: #tag_name,
                attrs: &[#(#attrs),*],

            }
        })
    }

    fn attr_code((attribute, opt_value): (Cow<'_, str>, Option<Cow<'_, str>>)) -> TokenStream {
        let value = opt_value.unwrap_or(Cow::Borrowed("true"));
        quote! {
            (#attribute, #value)
        }
    }
}

// #[allow(dead_code)]
// mod hypertext {
//     use super::*;

//     const NAMES_FILENAME: &str = "src/hypertext/generated_icon_names.rs";
//     const RENDERABLE_FILENAME: &str = "src/hypertext/generated_renderable.rs";

//     pub fn generate(index: &IconIndex) {
//         let enum_names = generate_enum_names(index);

//         let files =
//             [(NAMES_FILENAME, icon_names_code(enum_names)),
//              (RENDERABLE_FILENAME, renderable_impl_code(index))];

//         for (file, tokens) in files
//         {
//             let syntax_tree = syn::parse_file(&tokens.to_string()).unwrap();
//             let formatted = prettyplease::unparse(&syntax_tree);
//             let mut output_file = std::fs::File::create(file).unwrap();
//             output_file.write_all(formatted.as_bytes()).unwrap();
//         }
//     }

//     fn generate_enum_names(
//         index: &IconIndex
//     ) -> impl Iterator<Item = String>
//     {
//         let convert_case = move |path: &PathBuf| -> String {
//             s.split('_')
//                 .map(|word| {
//                     let (first, rest) = word.split_at(1);
//                     let mut fixed = first.to_ascii_uppercase();
//                     fixed.push_str(rest);
//                     fixed
//                 })
//                 .collect::<String>()
//         };

//         index.0.keys().map(convert_case)
//     }

//     pub fn icon_names_code(
//         enum_names: impl Iterator<Item = String>
//     ) -> TokenStream
//     {
//         let names = enum_names.map(|name| format_ident!("{}", name));
//         quote! {
//             pub enum IconName {
//                 #(#names),*
//             }
//         }
//     }

//     pub fn renderable_impl_code(
//         enum_names: impl Iterator<Item = String>,
//         icon_index: &IconIndex
//     ) -> TokenStream
//     {
//         let with_names = icon_index.iter().zip(enum_names);
//         let match_cases = with_names.map(|(index_entry, name)| {
//             quote! {
//                 (IconName::#name, IconVariant::Solid) =>
//             }
//         });
//         quote! {
//             /// Generated code. Do not edit.
//             use hypertext::{Buffer, prelude::*};
//             use crate::hypertext::Icon;
//             use crate::icons::*;

//             impl Renderable for Icon {
//                 fn render_to(&self, buffer: &mut Buffer) {
//                     match (self.name, self.variant) {
//                     }
//                 }
//             }
//         }
//     }

//     fn icon_code(name: &String, entry: &super::IconEntry) -> TokenStream {
//         let variants = [
//             (&entry.outline, "outline"),
//             (&entry.solid, "solid"),
//             (&entry.mini, "mini"),
//             (&entry.micro, "micro"),
//         ];
//         let icon_tokens = variants
//             .iter()
//             .filter_map(|(maybe_file, variant)| {
//                 if maybe_file.is_some() {
//                     Some(icon_variant_code(name, variant))
//                 }
//                 else {
//                     None
//                 }
//             });

//         TokenStream::from_iter(icon_tokens)
//     }

//     fn icon_variant_code(name: &String, variant: &str) -> TokenStream {
//         let fn_name = format!("{name}_{variant}");
//         let fn_name_ident = format_ident!("{fn_name}");
//         let string_name = fn_name.to_uppercase();
//         let string_name_ident = format_ident!("{string_name}");
//         quote! {
//             #[component]
//             pub fn #fn_name_ident() -> impl Renderable {
//                 rsx! {
//                     <div>
//                     </div>
//                 }
//             }
//         }
//     }
// }
