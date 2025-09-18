#![feature(test)]
extern crate test;
use test::Bencher;

use heroicons::{Icon, icon_name::*, icon_variant::*};
use heroicons_macros::for_each_icon;

#[bench]
fn generate_all_icons_independently(bencher: &mut Bencher) {
    bencher.iter(|| {
        for_each_icon!("heroicons/optimized/24/outline", |name, _| {
            let icon = Icon { name, variant: Outline, ..Default::default() };
            test::black_box(icon.to_string());
        });
        for_each_icon!("heroicons/optimized/24/solid", |name, _| {
            let icon = Icon { name, variant: Solid, ..Default::default() };
            test::black_box(icon.to_string());
        });
        for_each_icon!("heroicons/optimized/20/solid", |name, _| {
            let icon = Icon { name, variant: Mini, ..Default::default() };
            test::black_box(icon.to_string());
        });
        for_each_icon!("heroicons/optimized/16/solid", |name, _| {
            let icon = Icon { name, variant: Micro, ..Default::default() };
            test::black_box(icon.to_string());
        });
    });
}

#[bench]
fn generate_all_icons_into_one_buffer(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut buffer = test::black_box(String::new());
        for_each_icon!("heroicons/optimized/24/outline", |name, _| {
            let icon = Icon { name, variant: Outline, ..Default::default() };
            buffer.push_str(&icon.to_string());
        });
        for_each_icon!("heroicons/optimized/24/solid", |name, _| {
            let icon = Icon { name, variant: Solid, ..Default::default() };
            buffer.push_str(&icon.to_string());
        });
        for_each_icon!("heroicons/optimized/20/solid", |name, _| {
            let icon = Icon { name, variant: Mini, ..Default::default() };
            buffer.push_str(&icon.to_string());
        });
        for_each_icon!("heroicons/optimized/16/solid", |name, _| {
            let icon = Icon { name, variant: Micro, ..Default::default() };
            buffer.push_str(&icon.to_string());
        });
    });
}
