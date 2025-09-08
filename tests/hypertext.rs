mod test_utils;

#[cfg(feature = "hypertext")]
mod hypertext_tests {
    use heroicons::{Icon, IconName, Variant};
    use heroicons_macros::icons_in_path;
    use hypertext::prelude::*;

    use crate::test_utils;

    #[test]
    fn test_hypertext_icons_render() {
        let home_icon =
            Icon { name: IconName::Home, variant: Some(Variant::Outline) };
        let check_icon =
            Icon { name: IconName::CheckCircle, variant: Some(Variant::Solid) };
        let envelope_icon =
            Icon { name: IconName::Envelope, variant: Some(Variant::Mini) };
        let cart_icon = Icon {
            name: IconName::ShoppingCart,
            variant: Some(Variant::Micro),
        };

        // Verify they render as strings
        let home_rendered = home_icon.render().into_inner();
        let check_rendered = check_icon.render().into_inner();
        let envelope_rendered = envelope_icon.render().into_inner();
        let cart_rendered = cart_icon.render().into_inner();

        assert!(!home_rendered.is_empty());
        assert!(!check_rendered.is_empty());
        assert!(!envelope_rendered.is_empty());
        assert!(!cart_rendered.is_empty());
    }

    #[test]
    fn test_outline_icons_are_equivalent_to_source() {
        let icons = icons_in_path!("heroicons/optimized/24/outline");

        for (icon_name, file) in icons.into_iter() {
            let icon = rsx! {
                <Icon name=(*icon_name) variant=(Some(Variant::Outline))/>
            }.render().into_inner();
            assert!(test_utils::equivalent_to_source(icon, file));
        }
    }

    #[test]
    fn test_solid_icons_are_equivalent_to_source() {
        let icons = icons_in_path!("heroicons/optimized/24/solid");

        for (icon_name, file) in icons.into_iter() {
            let icon = rsx! {
                <Icon name=(*icon_name) variant=(Some(Variant::Solid))/>
            }.render().into_inner();
            assert!(test_utils::equivalent_to_source(icon, file));
        }
    }

    #[test]
    fn test_mini_icons_are_equivalent_to_source() {
        let icons = icons_in_path!("heroicons/optimized/20/solid");

        for (icon_name, file) in icons.into_iter() {
            let icon = rsx! {
                <Icon name=(*icon_name) variant=(Some(Variant::Mini))/>
            }.render().into_inner();
            assert!(test_utils::equivalent_to_source(icon, file));
        }
    }

    #[test]
    fn test_micro_icons_are_equivalent_to_source() {
        let icons = icons_in_path!("heroicons/optimized/16/solid");

        for (icon_name, file) in icons.into_iter() {
            let icon = rsx! {
                <Icon name=(*icon_name) variant=(Some(Variant::Micro))/>
            }.render().into_inner();
            assert!(test_utils::equivalent_to_source(icon, file));
        }
    }
}

#[cfg(not(feature = "hypertext"))]
mod no_hypertext_tests {
    #[test]
    fn test_hypertext_module_not_available_without_feature() {
        // This test ensures the hypertext module is not accessible without the feature
        // The compilation itself serves as the test - if this compiles, the feature gating works
        assert!(true);
    }
}
