#![feature(test)]
extern crate test;
use test::Bencher;

use heroicons::{Icon, icon_name::*, icon_variant::*};
use heroicons_macros::for_each_icon;

#[bench]
fn generate_all_icons_independently(bencher: &mut Bencher) {
    bencher.iter(|| {
        for_each_icon!("heroicons/optimized/24/outline", |name, _| {
            let icon = Icon { name, variant: Outline };
            test::black_box(icon.to_string());
        });
        for_each_icon!("heroicons/optimized/24/solid", |name, _| {
            let icon = Icon { name, variant: Solid };
            test::black_box(icon.to_string());
        });
        for_each_icon!("heroicons/optimized/20/solid", |name, _| {
            let icon = Icon { name, variant: Mini };
            test::black_box(icon.to_string());
        });
        for_each_icon!("heroicons/optimized/16/solid", |name, _| {
            let icon = Icon { name, variant: Micro };
            test::black_box(icon.to_string());
        });
    });
}

#[bench]
fn generate_all_icons_into_one_buffer(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut buffer = test::black_box(String::new());
        for_each_icon!("heroicons/optimized/24/outline", |name, _| {
            let icon = Icon { name, variant: Outline };
            buffer.push_str(&icon.to_string());
        });
        for_each_icon!("heroicons/optimized/24/solid", |name, _| {
            let icon = Icon { name, variant: Solid };
            buffer.push_str(&icon.to_string());
        });
        for_each_icon!("heroicons/optimized/20/solid", |name, _| {
            let icon = Icon { name, variant: Mini };
            buffer.push_str(&icon.to_string());
        });
        for_each_icon!("heroicons/optimized/16/solid", |name, _| {
            let icon = Icon { name, variant: Micro };
            buffer.push_str(&icon.to_string());
        });
    });
}
