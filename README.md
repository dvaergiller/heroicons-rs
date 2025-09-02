# Heroicons for Rust

A Rust crate providing access to the beautiful hand-crafted SVG icons from
[Heroicons](https://heroicons.com), created by the makers of Tailwind CSS.

This crate offers Heroicons in two formats:
- **String constants** - Direct access to SVG markup as `&'static str`
- **Hypertext components** - Ready-to-use components for the [`hypertext`](https://crates.io/crates/hypertext) framework (optional)

## Icon Variants

All icons are available in multiple variants:
- **Outline** (24×24px) - Outlined icons with 2px stroke
- **Solid** (24×24px) - Filled icons
- **Mini** (20×20px) - Smaller solid icons
- **Micro** (16×16px) - Tiny solid icons

## Usage

### String Constants

```rust
use heroicons::strings;

// Access icons as string constants
let home_icon = strings::HOME_OUTLINE;
let user_icon = strings::USER_SOLID;
let bell_icon = strings::BELL_MINI;
let star_icon = strings::STAR_MICRO;

// Use in HTML templates, web frameworks, etc.
let html = format!("<div>{}</div>", home_icon);
```

### Hypertext Components

Enable the `hypertext` feature in your `Cargo.toml`:

```toml
[dependencies]
heroicons = { version = "0.1", features = ["hypertext"] }
```

Then use the component functions with the `rsx!` macro:

```rust
use heroicons::hypertext::*;
use hypertext::prelude::*;

let page = rsx! {
    <div>
        <HomeOutline />
        <UserSolid />
    </div>
}.render();
```

Or with the `maud!` macro:

```rust
use heroicons::hypertext::*;
use hypertext::prelude::*;

let page = maud! {
    div {
        HomeOutline;
        UserSolid;
    }
}.render();
```

## Features

- **Default**: Only string constants via the `strings` module
- **`hypertext`**: Adds component functions in the `hypertext` module
