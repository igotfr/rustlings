// functions3.rs
// Make me compile! Execute `rustlings hint functions3` for hints :)

// ✓ I AM NOT DONE

fn main() {
    //call_me();
    call_me(4);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

// $ rustlings hint functions3
/*
This time, the function *declaration* is okay, but there's something wrong
with the place where we're calling the function.
*/