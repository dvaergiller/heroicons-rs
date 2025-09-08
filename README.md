# Heroicons for Rust

A Rust crate providing access to the beautiful hand-crafted SVG icons from
[Heroicons](https://heroicons.com), created by the makers of Tailwind CSS.

This crate offers Heroicons through a structured `Icon` system:
- **Icon struct** - Represent icons with `IconName` and `Variant`
- **String conversion** - Convert icons to SVG strings using `to_string()`
- **Hypertext components** - Ready-to-use components for the [`hypertext`](https://crates.io/crates/hypertext) framework

## Icon Variants

All icons are available in multiple variants:
- **Outline** (24×24px) - Outlined icons with 2px stroke
- **Solid** (24×24px) - Filled icons
- **Mini** (20×20px) - Smaller solid icons
- **Micro** (16×16px) - Tiny solid icons

## Usage

### Icon Struct

```rust
use heroicons::{Icon, IconName, Variant};

// Create icons with different variants
let home_icon = Icon {
    name: IconName::Home,
    variant: Variant::Outline,
};

let user_icon = Icon {
    name: IconName::User,
    variant: Variant::Solid,
};

// Convert to SVG string
let svg_string = home_icon.to_string();
// Use the SVG string in HTML templates, web frameworks, etc.
```

### Hypertext Components

The `hypertext` feature is enabled by default. Use the `<Icon>` component with the `rsx!` macro:

```rust
use heroicons::{Icon, IconName, Variant};
use hypertext::prelude::*;

let page = rsx! {
    <div>
        <Icon name=(IconName::Home) variant=(Variant::Outline) />
        <Icon name=(IconName::User) variant=(Variant::Solid) />
    </div>
}.render();
```

Or with the `maud!` macro:

```rust
use heroicons::{Icon, IconName, Variant};
use hypertext::prelude::*;

let page = maud! {
    div {
        Icon name=(IconName::Home) variant=(Variant::Outline);
        Icon name=(IconName::User) variant=(Variant::Solid);
    }
}.render();
```

## Features

- **Default**: Includes `hypertext` feature for component support
- **`hypertext`**: Adds component functions in the `hypertext` module  
- **`simd`**: Enables SIMD optimizations for HTML parsing
