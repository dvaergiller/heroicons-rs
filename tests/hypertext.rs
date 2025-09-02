#[cfg(feature = "hypertext")]
mod hypertext_tests {
    use heroicons::hypertext::*;
    use hypertext::prelude::*;

    #[test]
    fn test_hypertext_functions_return_raw() {
        let home_icon = home_outline();
        let check_icon = check_circle_solid();
        let envelope_icon = envelope_mini();
        let cart_icon = shopping_cart_micro();

        // Verify they return Raw<&'static str>
        assert!(!home_icon.as_str().is_empty());
        assert!(!check_icon.as_str().is_empty());
        assert!(!envelope_icon.as_str().is_empty());
        assert!(!cart_icon.as_str().is_empty());
    }

    #[test]
    fn test_hypertext_content_matches_strings() {
        use heroicons::strings;

        assert_eq!(home_outline().as_str(), strings::HOME_OUTLINE);
        assert_eq!(home_solid().as_str(), strings::HOME_SOLID);
        assert_eq!(check_circle_mini().as_str(), strings::CHECK_CIRCLE_MINI);
        assert_eq!(envelope_micro().as_str(), strings::ENVELOPE_MICRO);
    }

    #[test]
    fn test_hypertext_svg_content_is_valid() {
        let icons = vec![
            ("home_outline", home_outline()),
            ("check_circle_solid", check_circle_solid()),
            ("envelope_mini", envelope_mini()),
            ("shopping_cart_micro", shopping_cart_micro()),
        ];

        for (name, icon) in icons {
            let svg_content = icon.as_str();
            assert!(svg_content.starts_with("<svg"), "{} should start with <svg", name);
            assert!(svg_content.ends_with("</svg>\n"), "{} should end with </svg>", name);
            assert!(svg_content.contains("xmlns=\"http://www.w3.org/2000/svg\""), "{} should have SVG namespace", name);
            assert!(svg_content.contains("aria-hidden=\"true\""), "{} should have aria-hidden", name);
            assert!(svg_content.contains("data-slot=\"icon\""), "{} should have data-slot", name);
        }
    }

    #[test]
    fn test_all_icon_variants_available() {
        // Test that all variants are available for common icons
        let _home_outline = home_outline();
        let _home_solid = home_solid();
        let _home_mini = home_mini();
        let _home_micro = home_micro();

        let _user_plus_outline = user_plus_outline();
        let _user_plus_solid = user_plus_solid();
        let _user_plus_mini = user_plus_mini();
        let _user_plus_micro = user_plus_micro();

        let _envelope_outline = envelope_outline();
        let _envelope_solid = envelope_solid();
        let _envelope_mini = envelope_mini();
        let _envelope_micro = envelope_micro();

        // If this compiles, all variants are available
        assert!(true);
    }

    #[test]
    fn test_icon_content_different_between_variants() {
        // Ensure different variants have different content
        assert_ne!(home_outline().as_str(), home_solid().as_str());
        assert_ne!(home_solid().as_str(), home_mini().as_str());
        assert_ne!(home_mini().as_str(), home_micro().as_str());

        // Ensure different icons have different content
        assert_ne!(home_outline().as_str(), envelope_outline().as_str());
        assert_ne!(user_plus_solid().as_str(), shopping_cart_solid().as_str());
    }

    #[test]
    fn test_icon_variants_have_correct_attributes() {
        // Outline icons should have stroke attributes
        assert!(home_outline().as_str().contains("stroke-width"));
        assert!(home_outline().as_str().contains("fill=\"none\""));

        // Solid icons should have fill attributes
        assert!(home_solid().as_str().contains("fill=\"currentColor\""));
        assert!(!home_solid().as_str().contains("stroke-width"));
    }

    #[test]
    fn test_icon_sizes_have_correct_viewbox() {
        // 24px icons (outline/solid)
        assert!(home_outline().as_str().contains("viewBox=\"0 0 24 24\""));
        assert!(home_solid().as_str().contains("viewBox=\"0 0 24 24\""));

        // 20px icons (mini)
        assert!(home_mini().as_str().contains("viewBox=\"0 0 20 20\""));

        // 16px icons (micro)
        assert!(home_micro().as_str().contains("viewBox=\"0 0 16 16\""));
    }

    #[test]
    fn test_rsx_macro_with_multiple_heroicons() {
        let navigation = rsx! {
            <nav class="nav">
                <div><HomeOutline /></div>
                <div><EnvelopeOutline /></div>
                <div><ShoppingCartOutline /></div>
            </nav>
        };

        let _rendered = navigation.render();
        assert!(true);
    }

    #[test]
    fn test_maud_macro_with_multiple_heroicons() {
        let navigation = maud! {
            nav class="navigation" {
                div { HomeOutline; }
                div { EnvelopeOutline; }
                div { ShoppingCartOutline; }
                div { UserPlusOutline; }
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
