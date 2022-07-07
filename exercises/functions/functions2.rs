// functions2.rs
// Make me compile! Execute `rustlings hint functions2` for hints :)

// ✓ I AM NOT DONE

fn main() {
    call_me(3);
}

//fn call_me(num:) {
fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

// $ rustlings hint functions2
/*
Rust requires that all parts of a function's signature have type annotations,
but `call_me` is missing the type annotation of `num`.
*/