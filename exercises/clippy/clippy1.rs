// clippy1.rs
// The Clippy tool is a collection of lints to analyze your code
// so you can catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy warnings
// check clippy's suggestions from the output to solve the exercise.
// Execute `rustlings hint clippy1` for hints :)

// ✓ I AM NOT DONE

use std::f32;

fn main() {
    //let pi = 3.14f32;
    let pi = f32::consts::PI;
    let radius = 5.00f32;

    let area = pi * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}

// $ rustlings hint clippy1
/*
Rust stores the highest precision version of any long or inifinite precision
mathematical constants in the rust standard library.
https://doc.rust-lang.org/stable/std/f32/consts/index.html

We may be tempted to use our own approximations for certain mathematical constants,
but clippy recognizes those imprecise mathematical constants as a source of
potential error.
See the suggestions of the clippy warning in compile output and use the
appropriate replacement constant from std::f32::consts...
*/