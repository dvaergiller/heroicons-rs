use heroicons::{Icon, IconName, Variant};

#[test]
fn test_common_icons_are_available() {
    let home_outline = Icon { 
        name: IconName::Home, 
        variant: Some(Variant::Outline),
    };
    let home_solid = Icon { 
        name: IconName::Home, 
        variant: Some(Variant::Solid),
    };
    let home_mini = Icon { 
        name: IconName::Home, 
        variant: Some(Variant::Mini),
    };
    let home_micro = Icon { 
        name: IconName::Home, 
        variant: Some(Variant::Micro),
    };
    
    // Convert to SVG strings to verify they work
    let _svg1 = home_outline.to_string();
    let _svg2 = home_solid.to_string();
    let _svg3 = home_mini.to_string();
    let _svg4 = home_micro.to_string();
}

#[test]
fn test_svg_structure_is_valid() {
    let home_outline = Icon { 
        name: IconName::Home, 
        variant: Some(Variant::Outline),
    };
    let check_circle_solid = Icon { 
        name: IconName::CheckCircle, 
        variant: Some(Variant::Solid),
    };
    
    let home_svg = home_outline.to_string();
    let check_svg = check_circle_solid.to_string();
    
    // Test that SVG contains proper attributes
    assert!(home_svg.contains("viewBox"));
    assert!(check_svg.contains("fill=\"currentColor\""));
}

#[test]
fn test_icon_variants_have_correct_attributes() {
    let home_outline = Icon { 
        name: IconName::Home, 
        variant: Some(Variant::Outline),
    };
    let home_solid = Icon { 
        name: IconName::Home, 
        variant: Some(Variant::Solid),
    };
    
    let outline_svg = home_outline.to_string();
    let solid_svg = home_solid.to_string();
    
    // Outline icons should have stroke attributes
    assert!(outline_svg.contains("stroke-width"));
    assert!(outline_svg.contains("fill=\"none\""));

    // Solid icons should have fill attributes
    assert!(solid_svg.contains("fill=\"currentColor\""));
    assert!(!solid_svg.contains("stroke-width"));
}

#[test]
fn test_icon_sizes_have_correct_viewbox() {
    let home_outline = Icon { 
        name: IconName::Home, 
        variant: Some(Variant::Outline),
    };
    let home_solid = Icon { 
        name: IconName::Home, 
        variant: Some(Variant::Solid),
    };
    let home_mini = Icon { 
        name: IconName::Home, 
        variant: Some(Variant::Mini),
    };
    let home_micro = Icon { 
        name: IconName::Home, 
        variant: Some(Variant::Micro),
    };
    
    let outline_svg = home_outline.to_string();
    let solid_svg = home_solid.to_string();
    let mini_svg = home_mini.to_string();
    let micro_svg = home_micro.to_string();
    
    // 24px icons (outline/solid)
    assert!(outline_svg.contains("viewBox=\"0 0 24 24\""));
    assert!(solid_svg.contains("viewBox=\"0 0 24 24\""));

    // 20px icons (mini)
    assert!(mini_svg.contains("viewBox=\"0 0 20 20\""));

    // 16px icons (micro)
    assert!(micro_svg.contains("viewBox=\"0 0 16 16\""));
}

#[test]
fn test_multiple_icons_different_content() {
    let home_outline = Icon { 
        name: IconName::Home, 
        variant: Some(Variant::Outline),
    };
    let check_circle_outline = Icon { 
        name: IconName::CheckCircle, 
        variant: Some(Variant::Outline),
    };
    let home_solid = Icon { 
        name: IconName::Home, 
        variant: Some(Variant::Solid),
    };
    let home_mini = Icon { 
        name: IconName::Home, 
        variant: Some(Variant::Mini),
    };
    let home_micro = Icon { 
        name: IconName::Home, 
        variant: Some(Variant::Micro),
    };
    
    // Ensure different icons have different SVG content
    let home_svg = home_outline.to_string();
    let check_svg = check_circle_outline.to_string();
    let home_solid_svg = home_solid.to_string();
    let home_mini_svg = home_mini.to_string();
    let home_micro_svg = home_micro.to_string();
    
    // Different icons should have different SVG content
    assert_ne!(home_svg, check_svg);
    assert_ne!(home_svg, home_solid_svg);
    assert_ne!(home_mini_svg, home_micro_svg);
}
