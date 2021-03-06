// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


#![feature(simd)]
#![allow(non_camel_case_types)]

use std::mem;

/// `T` should satisfy `size_of T (mod min_align_of T) === 0` to be stored at `Vec<T>` properly
/// Please consult the issue #20460
fn check<T>() {
    assert_eq!(mem::size_of::<T>() % mem::min_align_of::<T>(), 0)
}

fn main() {
    check::<u8x2>();
    check::<u8x3>();
    check::<u8x4>();
    check::<u8x5>();
    check::<u8x6>();
    check::<u8x7>();
    check::<u8x8>();

    check::<i16x2>();
    check::<i16x3>();
    check::<i16x4>();
    check::<i16x5>();
    check::<i16x6>();
    check::<i16x7>();
    check::<i16x8>();

    check::<f32x2>();
    check::<f32x3>();
    check::<f32x4>();
    check::<f32x5>();
    check::<f32x6>();
    check::<f32x7>();
    check::<f32x8>();
}

#[simd] struct u8x2(u8, u8);
#[simd] struct u8x3(u8, u8, u8);
#[simd] struct u8x4(u8, u8, u8, u8);
#[simd] struct u8x5(u8, u8, u8, u8, u8);
#[simd] struct u8x6(u8, u8, u8, u8, u8, u8);
#[simd] struct u8x7(u8, u8, u8, u8, u8, u8, u8);
#[simd] struct u8x8(u8, u8, u8, u8, u8, u8, u8, u8);

#[simd] struct i16x2(i16, i16);
#[simd] struct i16x3(i16, i16, i16);
#[simd] struct i16x4(i16, i16, i16, i16);
#[simd] struct i16x5(i16, i16, i16, i16, i16);
#[simd] struct i16x6(i16, i16, i16, i16, i16, i16);
#[simd] struct i16x7(i16, i16, i16, i16, i16, i16, i16);
#[simd] struct i16x8(i16, i16, i16, i16, i16, i16, i16, i16);

#[simd] struct f32x2(f32, f32);
#[simd] struct f32x3(f32, f32, f32);
#[simd] struct f32x4(f32, f32, f32, f32);
#[simd] struct f32x5(f32, f32, f32, f32, f32);
#[simd] struct f32x6(f32, f32, f32, f32, f32, f32);
#[simd] struct f32x7(f32, f32, f32, f32, f32, f32, f32);
#[simd] struct f32x8(f32, f32, f32, f32, f32, f32, f32, f32);
