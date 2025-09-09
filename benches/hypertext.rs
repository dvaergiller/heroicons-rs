#![feature(test)]
extern crate test;
use test::Bencher;

use hypertext::{prelude::*, Buffer};

use heroicons::{Icon, icon_name::*, icon_variant::*};
use heroicons_macros::for_each_icon;

#[bench]
fn generate_all_icons_independently(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut buffer = test::black_box(String::new());
        for_each_icon!("heroicons/optimized/24/outline", |name, _| {
            buffer = rsx! {
                <Icon name=(name) variant=(Outline)/>
            }.render().into_inner();
        });
        for_each_icon!("heroicons/optimized/24/solid", |name, _| {
            buffer = rsx! {
                <Icon name=(name) variant=(Solid)/>
            }.render().into_inner();
        });
        for_each_icon!("heroicons/optimized/20/solid", |name, _| {
            buffer = rsx! {
                <Icon name=(name) variant=(Mini)/>
            }.render().into_inner();
        });
        for_each_icon!("heroicons/optimized/16/solid", |name, _| {
            buffer = rsx! {
                <Icon name=(name) variant=(Micro)/>
            }.render().into_inner();
        });
    });
}

#[bench]
fn generate_all_icons_into_one_buffer(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut buffer = test::black_box(Buffer::new());
        for_each_icon!("heroicons/optimized/24/outline", |name, _| {
            rsx! {
                <Icon name=(name) variant=(Outline)/>
            }.render_to(&mut buffer);
        });
        for_each_icon!("heroicons/optimized/24/solid", |name, _| {
            rsx! {
                <Icon name=(name) variant=(Solid)/>
            }.render_to(&mut buffer);
        });
        for_each_icon!("heroicons/optimized/20/solid", |name, _| {
            rsx! {
                <Icon name=(name) variant=(Mini)/>
            }.render_to(&mut buffer);
        });
        for_each_icon!("heroicons/optimized/16/solid", |name, _| {
            rsx! {
                <Icon name=(name) variant=(Micro)/>
            }.render_to(&mut buffer);
        });
    });
}
