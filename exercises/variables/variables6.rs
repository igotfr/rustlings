// variables6.rs
// Make me compile! Execute the command `rustlings hint variables6` if you want a hint :)

// ✓ I AM NOT DONE

//const NUMBER = 3;
const NUMBER: u8 = 3;
fn main() {
    println!("Number {}", NUMBER);
}

// $ rustlings hint variables6
/*
We know about variables and mutability, but there is another important type of
variable available; constants.
Constants are always immutable and they are declared with keyword 'const' rather
than keyword 'let'.
Constants types must also always be annotated.

Read more about constants under 'Differences Between Variables and Constants' in the book's section 'Variables and Mutability':
https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#differences-between-variables-and-constants
*/