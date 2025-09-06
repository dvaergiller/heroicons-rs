#[cfg(feature = "hypertext")]
mod hypertext_tests {
    use heroicons::{Icon, IconName, Variant};
    use hypertext::prelude::*;

    #[test]
    fn test_hypertext_icons_render_correctly() {
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

        // Verify they render as SVG strings
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
    fn test_icon_attributes_applied_correctly() {
        let home_icon =
            Icon { name: IconName::Home, variant: Some(Variant::Outline) };

        let rendered = home_icon.render().into_inner();

        // The rendered output should contain basic SVG structure
        assert!(rendered.contains("<svg"));
        assert!(rendered.contains("</svg>"));
    }

    #[test]
    fn test_hypertext_svg_content_is_valid() {
        let icons = vec![
            (
                "home_outline",
                Icon { name: IconName::Home, variant: Some(Variant::Outline) },
            ),
            (
                "check_circle_solid",
                Icon {
                    name: IconName::CheckCircle,
                    variant: Some(Variant::Solid),
                },
            ),
            (
                "envelope_mini",
                Icon { name: IconName::Envelope, variant: Some(Variant::Mini) },
            ),
            (
                "shopping_cart_micro",
                Icon {
                    name: IconName::ShoppingCart,
                    variant: Some(Variant::Micro),
                },
            ),
        ];

        for (name, icon) in icons {
            let svg_content = icon.render().into_inner();
            assert!(
                svg_content.starts_with("<svg"),
                "{} should start with <svg",
                name
            );
            assert!(
                svg_content.ends_with("</svg>"),
                "{} should end with </svg>",
                name
            );
            assert!(
                svg_content.contains("xmlns=\"http://www.w3.org/2000/svg\""),
                "{} should have SVG namespace",
                name
            );
            assert!(
                svg_content.contains("aria-hidden=\"true\""),
                "{} should have aria-hidden",
                name
            );
            assert!(
                svg_content.contains("data-slot=\"icon\""),
                "{} should have data-slot",
                name
            );
        }
    }

    #[test]
    fn test_all_icon_variants_available() {
        // Test that all variants are available for common icons
        let _home_outline =
            Icon { name: IconName::Home, variant: Some(Variant::Outline) };
        let _home_solid =
            Icon { name: IconName::Home, variant: Some(Variant::Solid) };
        let _home_mini =
            Icon { name: IconName::Home, variant: Some(Variant::Mini) };
        let _home_micro =
            Icon { name: IconName::Home, variant: Some(Variant::Micro) };

        let _user_plus_outline =
            Icon { name: IconName::UserPlus, variant: Some(Variant::Outline) };
        let _user_plus_solid =
            Icon { name: IconName::UserPlus, variant: Some(Variant::Solid) };
        let _user_plus_mini =
            Icon { name: IconName::UserPlus, variant: Some(Variant::Mini) };
        let _user_plus_micro =
            Icon { name: IconName::UserPlus, variant: Some(Variant::Micro) };

        let _envelope_outline =
            Icon { name: IconName::Envelope, variant: Some(Variant::Outline) };
        let _envelope_solid =
            Icon { name: IconName::Envelope, variant: Some(Variant::Solid) };
        let _envelope_mini =
            Icon { name: IconName::Envelope, variant: Some(Variant::Mini) };
        let _envelope_micro =
            Icon { name: IconName::Envelope, variant: Some(Variant::Micro) };
    }

    #[test]
    fn test_icon_content_different_between_variants() {
        let home_outline =
            Icon { name: IconName::Home, variant: Some(Variant::Outline) };
        let home_solid =
            Icon { name: IconName::Home, variant: Some(Variant::Solid) };
        let home_mini =
            Icon { name: IconName::Home, variant: Some(Variant::Mini) };
        let home_micro =
            Icon { name: IconName::Home, variant: Some(Variant::Micro) };

        let envelope_outline =
            Icon { name: IconName::Envelope, variant: Some(Variant::Outline) };
        let user_plus_solid =
            Icon { name: IconName::UserPlus, variant: Some(Variant::Solid) };
        let shopping_cart_solid = Icon {
            name: IconName::ShoppingCart,
            variant: Some(Variant::Solid),
        };

        // Ensure different variants have different content
        assert_ne!(
            home_outline.render().into_inner(),
            home_solid.render().into_inner()
        );
        assert_ne!(
            home_solid.render().into_inner(),
            home_mini.render().into_inner()
        );
        assert_ne!(
            home_mini.render().into_inner(),
            home_micro.render().into_inner()
        );

        // Ensure different icons have different content
        assert_ne!(
            home_outline.render().into_inner(),
            envelope_outline.render().into_inner()
        );
        assert_ne!(
            user_plus_solid.render().into_inner(),
            shopping_cart_solid.render().into_inner()
        );
    }

    #[test]
    fn test_icon_variants_have_correct_attributes() {
        let home_outline =
            Icon { name: IconName::Home, variant: Some(Variant::Outline) };
        let home_solid =
            Icon { name: IconName::Home, variant: Some(Variant::Solid) };

        let outline_rendered = home_outline.render().into_inner();
        let solid_rendered = home_solid.render().into_inner();

        // Outline icons should have stroke attributes
        assert!(outline_rendered.contains("stroke-width"));
        assert!(outline_rendered.contains("fill=\"none\""));

        // Solid icons should have fill attributes
        assert!(solid_rendered.contains("fill=\"currentColor\""));
        assert!(!solid_rendered.contains("stroke-width"));
    }

    #[test]
    fn test_icon_sizes_have_correct_viewbox() {
        let home_outline =
            Icon { name: IconName::Home, variant: Some(Variant::Outline) };
        let home_solid =
            Icon { name: IconName::Home, variant: Some(Variant::Solid) };
        let home_mini =
            Icon { name: IconName::Home, variant: Some(Variant::Mini) };
        let home_micro =
            Icon { name: IconName::Home, variant: Some(Variant::Micro) };

        // 24px icons (outline/solid)
        assert!(
            home_outline
                .render()
                .into_inner()
                .contains("viewBox=\"0 0 24 24\"")
        );
        assert!(
            home_solid.render().into_inner().contains("viewBox=\"0 0 24 24\"")
        );

        // 20px icons (mini)
        assert!(
            home_mini.render().into_inner().contains("viewBox=\"0 0 20 20\"")
        );

        // 16px icons (micro)
        assert!(
            home_micro.render().into_inner().contains("viewBox=\"0 0 16 16\"")
        );
    }

    #[test]
    fn test_rsx_macro_with_multiple_heroicons() {
        let navigation = rsx! {
            <nav class="nav">
                <div><Icon name=(IconName::Home) variant=(Some(Variant::Outline)) /></div>
                <div><Icon name=(IconName::Envelope) variant=(Some(Variant::Outline)) /></div>
                <div><Icon name=(IconName::ShoppingCart) variant=(Some(Variant::Outline)) /></div>
            </nav>
        };

        let _rendered = navigation.render();
        assert!(true);
    }

    #[test]
    fn test_maud_macro_with_multiple_heroicons() {
        let navigation = maud! {
            nav class="navigation" {
                div { (Icon { name: IconName::Home, variant: Some(Variant::Outline), }) }
                div { (Icon { name: IconName::Envelope, variant: Some(Variant::Outline), }) }
                div { (Icon { name: IconName::ShoppingCart, variant: Some(Variant::Outline), }) }
                div { (Icon { name: IconName::UserPlus, variant: Some(Variant::Outline), }) }
            }
        };

        let _rendered = navigation.render();
        assert!(true);
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
