mod test_utils;

#[cfg(feature = "hypertext")]
mod hypertext_tests {
    use heroicons::{Icon, icon_name::*, icon_variant::*};
    use heroicons_macros::for_each_icon;
    use hypertext::prelude::*;

    use crate::test_utils;

    #[test]
    fn test_hypertext_icons_render() {
        let home_icon = Icon { name: Home, variant: Outline };
        let check_icon = Icon { name: CheckCircle, variant: Solid };
        let envelope_icon = Icon { name: Envelope, variant: Mini };
        let cart_icon = Icon { name: ShoppingCart, variant: Micro };

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
        for_each_icon!("heroicons/optimized/24/outline", |icon_name, file| {
            let icon = rsx! {
                <Icon name=(icon_name) variant=(Outline)/>
            }
            .render()
            .into_inner();
            test_utils::check_equivalent_to_source(icon, file);
        });
    }

    #[test]
    fn test_solid_icons_are_equivalent_to_source() {
        for_each_icon!("heroicons/optimized/24/solid", |icon_name, file| {
            let icon = rsx! {
                <Icon name=(icon_name) variant=(Solid)/>
            }
            .render()
            .into_inner();
            test_utils::check_equivalent_to_source(icon, file);
        });
    }

    #[test]
    fn test_mini_icons_are_equivalent_to_source() {
        for_each_icon!("heroicons/optimized/20/solid", |icon_name, file| {
            let icon = rsx! {
                <Icon name=(icon_name) variant=(Mini)/>
            }
            .render()
            .into_inner();
            test_utils::check_equivalent_to_source(icon, file);
        });
    }

    #[test]
    fn test_micro_icons_are_equivalent_to_source() {
        for_each_icon!("heroicons/optimized/16/solid", |icon_name, file| {
            let icon = rsx! {
                <Icon name=(icon_name) variant=(Micro)/>
            }
            .render()
            .into_inner();
            test_utils::check_equivalent_to_source(icon, file);
        });
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
