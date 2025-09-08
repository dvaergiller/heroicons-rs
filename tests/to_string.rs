use heroicons::{Icon, IconName, Variant};
use heroicons_macros::icons_in_path;

mod test_utils;

#[test]
fn test_common_icons_are_available() {
    let home_outline =
        Icon { name: IconName::Home, variant: Variant::Outline };
    let home_solid =
        Icon { name: IconName::Home, variant: Variant::Solid };
    let home_mini = Icon { name: IconName::Home, variant: Variant::Mini };
    let home_micro =
        Icon { name: IconName::Home, variant: Variant::Micro };

    // Convert to SVG strings to verify they work
    let _svg1 = home_outline.to_string();
    let _svg2 = home_solid.to_string();
    let _svg3 = home_mini.to_string();
    let _svg4 = home_micro.to_string();
}

#[test]
fn test_outline_icons_are_equivalent_to_source() {
    let icons = icons_in_path!("heroicons/optimized/24/outline");

    for (icon_name, file) in icons.into_iter() {
        let icon = Icon { name: *icon_name, variant: Variant::Outline };
        assert!(test_utils::equivalent_to_source(icon.to_string(), file));
    }
}

#[test]
fn test_solid_icons_are_equivalent_to_source() {
    let icons = icons_in_path!("heroicons/optimized/24/solid");

    for (icon_name, file) in icons.into_iter() {
        let icon = Icon { name: *icon_name, variant: Variant::Solid };
        assert!(test_utils::equivalent_to_source(icon.to_string(), file));
    }
}

#[test]
fn test_mini_icons_are_equivalent_to_source() {
    let icons = icons_in_path!("heroicons/optimized/20/solid");

    for (icon_name, file) in icons.into_iter() {
        let icon = Icon { name: *icon_name, variant: Variant::Mini };
        assert!(test_utils::equivalent_to_source(icon.to_string(), file));
     }
}

#[test]
fn test_micro_icons_are_equivalent_to_source() {
    let icons = icons_in_path!("heroicons/optimized/16/solid");

    for (icon_name, file) in icons.into_iter() {
        let icon = Icon { name: *icon_name, variant: Variant::Micro };
        assert!(test_utils::equivalent_to_source(icon.to_string(), file));
    }
}
