use heroicons::strings::*;

#[test]
fn test_common_icons_are_available() {
    assert!(!HOME_OUTLINE.is_empty());
    assert!(!HOME_SOLID.is_empty());
    assert!(!HOME_MINI.is_empty());
    assert!(!HOME_MICRO.is_empty());
}

#[test]
fn test_svg_structure_is_valid() {
    // Test that icons contain proper SVG structure
    assert!(HOME_OUTLINE.contains("<svg"));
    assert!(HOME_OUTLINE.contains("</svg>"));
    assert!(HOME_OUTLINE.contains("viewBox"));

    assert!(CHECK_CIRCLE_SOLID.contains("fill=\"currentColor\""));
    assert!(ENVELOPE_OUTLINE.contains("stroke=\"currentColor\""));
}

#[test]
fn test_icon_variants_have_correct_attributes() {
    // Outline icons should have stroke attributes
    assert!(HOME_OUTLINE.contains("stroke-width"));
    assert!(HOME_OUTLINE.contains("fill=\"none\""));

    // Solid icons should have fill attributes
    assert!(HOME_SOLID.contains("fill=\"currentColor\""));
    assert!(!HOME_SOLID.contains("stroke-width"));
}

#[test]
fn test_icon_sizes_have_correct_viewbox() {
    // 24px icons (outline/solid)
    assert!(HOME_OUTLINE.contains("viewBox=\"0 0 24 24\""));
    assert!(HOME_SOLID.contains("viewBox=\"0 0 24 24\""));

    // 20px icons (mini)
    assert!(HOME_MINI.contains("viewBox=\"0 0 20 20\""));

    // 16px icons (micro)
    assert!(HOME_MICRO.contains("viewBox=\"0 0 16 16\""));
}

#[test]
fn test_multiple_icons_different_content() {
    // Ensure different icons have different SVG content
    assert_ne!(HOME_OUTLINE, CHECK_CIRCLE_OUTLINE);
    assert_ne!(HOME_SOLID, HOME_OUTLINE);
    assert_ne!(HOME_MINI, HOME_MICRO);
    assert_ne!(ENVELOPE_OUTLINE, SHOPPING_CART_OUTLINE);
}
