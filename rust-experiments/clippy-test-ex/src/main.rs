// #![warn(clippy::pedantic)]
// insert groups from https://github.com/rust-lang/rust-clippy/blob/master/README.md
// https://rust-lang.github.io/rust-clippy/master/
// #![warn(clippy::nursery)]
// #![allow(clippy::all)]

use std::f32;

fn main() {
    // should throw an error
    // cargo clippy
    // error: approximate value of `f32::consts::PI` found
    let pi = 3.14f32;
    let radius = 5.00f32;

    // unused should still trigger a warn, from cargo check itself probabaly
    // trigger an error even with allow clippy all
    let a = 0;

    let area = pi * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
    // if pedantic mode working properly it should warn about missing `;`
}
