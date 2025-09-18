use heroicons::{Icon, icon_name::*, icon_variant::*};
use heroicons_macros::for_each_icon;

mod test_utils;

#[test]
fn test_common_icons_are_available() {
    let home_outline =
        Icon { name: Home, variant: Outline, ..Default::default() };
    let home_solid = Icon { name: Home, variant: Solid, ..Default::default() };
    let home_mini = Icon { name: Home, variant: Mini, ..Default::default() };
    let home_micro = Icon { name: Home, variant: Micro, ..Default::default() };

    // Convert to SVG strings to verify they work
    let _svg1 = home_outline.to_string();
    let _svg2 = home_solid.to_string();
    let _svg3 = home_mini.to_string();
    let _svg4 = home_micro.to_string();
}

#[test]
fn test_outline_icons_are_equivalent_to_source() {
    for_each_icon!("heroicons/optimized/24/outline", |icon_name, file| {
        let icon =
            Icon { name: icon_name, variant: Outline, ..Default::default() };
        test_utils::check_equivalent_to_source(icon.to_string(), file);
    });
}

#[test]
fn test_solid_icons_are_equivalent_to_source() {
    for_each_icon!("heroicons/optimized/24/solid", |icon_name, file| {
        let icon =
            Icon { name: icon_name, variant: Solid, ..Default::default() };
        test_utils::check_equivalent_to_source(icon.to_string(), file);
    });
}

#[test]
fn test_mini_icons_are_equivalent_to_source() {
    for_each_icon!("heroicons/optimized/20/solid", |icon_name, file| {
        let icon =
            Icon { name: icon_name, variant: Mini, ..Default::default() };
        test_utils::check_equivalent_to_source(icon.to_string(), file);
    });
}

#[test]
fn test_micro_icons_are_equivalent_to_source() {
    for_each_icon!("heroicons/optimized/16/solid", |icon_name, file| {
        let icon =
            Icon { name: icon_name, variant: Micro, ..Default::default() };
        test_utils::check_equivalent_to_source(icon.to_string(), file);
    });
}
