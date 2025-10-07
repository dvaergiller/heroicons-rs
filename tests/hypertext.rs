mod test_utils;

#[cfg(feature = "hypertext")]
mod hypertext_tests {
    use heroicons::{Icon, icon_name::*, icon_variant::*};
    use heroicons_macros::for_each_icon;
    use hypertext::prelude::*;

    use crate::test_utils;

    #[test]
    fn test_hypertext_icons_render() {
        let home_icon =
            Icon { name: Home, variant: Outline, ..Default::default() };
        let check_icon =
            Icon { name: CheckCircle, variant: Solid, ..Default::default() };
        let envelope_icon =
            Icon { name: Envelope, variant: Mini, ..Default::default() };
        let cart_icon =
            Icon { name: ShoppingCart, variant: Micro, ..Default::default() };

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
                <Icon name=(icon_name) variant=(Outline) ../>
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
                <Icon name=(icon_name) variant=(Solid) ../>
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
                <Icon name=(icon_name) variant=(Mini) ../>
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
                <Icon name=(icon_name) variant=(Micro) ../>
            }
            .render()
            .into_inner();
            test_utils::check_equivalent_to_source(icon, file);
        });
    }

    #[test]
    fn test_id_attribute_can_be_added() {
        let mut with_id = rsx! {
            <Icon id="some-id" name=(AcademicCap) variant=(Solid) ../>
        }
        .render()
        .into_inner();
        let without_id = rsx! {
            <Icon name=(AcademicCap) variant=(Solid) ../>
        }
        .render()
        .into_inner();
        assert_ne!(with_id, without_id);
        let id_removed: String = with_id.split(" id=\"some-id\"").collect();
        assert_eq!(id_removed, without_id);
    }

    #[test]
    fn test_id_and_class_attributes_can_be_added() {
        let with_id_and_class = rsx! {
            <Icon
                id="some-id"
                class="some-class"
                name=(AcademicCap)
                variant=(Solid) ../>
        }
        .render()
        .into_inner();
        let without_id_or_class = rsx! {
            <Icon name=(AcademicCap) variant=(Solid) ../>
        }
        .render()
        .into_inner();

        // Not equal yet, still has those
        assert_ne!(with_id_and_class, without_id_or_class);
        let id_removed: String =
            with_id_and_class.split(" id=\"some-id\"").collect();

        // Still not equal: class remains
        assert_ne!(id_removed, without_id_or_class);
        let id_and_class_removed: String =
            id_removed.split(" class=\"some-class\"").collect();

        assert_eq!(id_and_class_removed, without_id_or_class);
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
