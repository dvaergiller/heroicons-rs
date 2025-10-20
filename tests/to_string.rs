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

#[test]
fn test_id_field_script_injection() {
    let icon = Icon {
        id: "\"></svg><script>alert(\"hickedy hackedy\");</script><svg id=\"",
        name: Home,
        variant: Outline,
        ..Default::default()
    };
    assert_eq!(
        icon.to_string(),
        "<svg xmlns=\"http://www.w3.org/2000/svg\" fill=\"none\" viewBox=\"0 0 24 24\" stroke-width=\"1.5\" stroke=\"currentColor\" aria-hidden=\"true\" data-slot=\"icon\" id=\"&quot;&gt;&lt;/svg&gt;&lt;script&gt;alert(&quot;hickedy hackedy&quot;);&lt;/script&gt;&lt;svg id=&quot;\"><path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"m2.25 12 8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25\"/></svg>"
    );
}
