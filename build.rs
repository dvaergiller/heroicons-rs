use std::{io::Write, collections::HashMap, ffi::OsStr, path::{Path, PathBuf}};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

static ICONS_ROOT: &str = "heroicons/optimized";
static OUTPUT_FILENAME: &str = "src/generated.rs";

#[derive(Default)]
struct IconEntry {
    outline: Option<PathBuf>,
    solid: Option<PathBuf>,
    mini: Option<PathBuf>,
    micro: Option<PathBuf>,
}

fn main() {
    let mut icon_index: HashMap<String, IconEntry> = HashMap::default();
    svg_files().for_each(|path| insert_svg(&path, &mut icon_index));

    let mut tokens = module_header();
    let icon_tokens = icon_index
        .iter()
        .map(|(name, entry)| icon_code(name, entry));
    tokens.extend(icon_tokens);
    let syntax_tree = syn::parse_file(&tokens.to_string()).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);
    let mut output_file = std::fs::File::create(OUTPUT_FILENAME).unwrap();
    output_file.write_all(formatted.as_bytes()).unwrap();
}

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

fn insert_svg(path: &PathBuf, index: &mut HashMap<String, IconEntry>) {
    let icon_name = to_icon_name(path).unwrap();
    let icon_entry = index.entry(icon_name).or_default();

    let components = path
        .components()
        .rev()
        .map(|component| component.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<String>>();
    let type_string = &components[1];
    let size_string = &components[2];

    match (type_string.as_str(), size_string.as_str()) {
        ("outline", "24") => icon_entry.outline = Some(path.clone()),
        ("solid", "24") => icon_entry.solid = Some(path.clone()),
        ("solid", "20") => icon_entry.mini = Some(path.clone()),
        ("solid", "16") => icon_entry.micro = Some(path.clone()),
        _ => panic!("Unexpected folder structure: {path:?}")
    }
}

fn to_icon_name(path: &Path) -> Option<String> {
    if path.extension() == Some(OsStr::new("svg")) {
        path
            .file_stem()
            .map(|stem| stem.to_string_lossy().into_owned())
            .map(|stem_str| stem_str.replace("-", "_"))
    }
    else {
        None
    }
}

fn module_header() -> TokenStream {
    quote! {
        use hypertext::{Raw, component};
    }
}

fn icon_code(name: &String, entry: &IconEntry) -> TokenStream {
    let mut tokens = TokenStream::new();

    if let Some(outline_file) = &entry.outline {
        let svg = std::fs::read_to_string(outline_file).unwrap();
        tokens.extend(icon_variant_code(name, "outline", svg));
    }

    tokens
}

fn icon_variant_code(name: &String, variant: &str, svg: String) -> TokenStream {
    let icon_ident = format_ident!("{name}_{variant}");
    quote! {
        #[component]
        pub fn #icon_ident() -> Raw<&'static str> {
            Raw::dangerously_create(#svg)
        }
    }
}
