// macros1.rs
// Make me compile! Execute `rustlings hint macros1` for hints :)

// ✓ I AM NOT DONE

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    //my_macro();
    my_macro!();
}

// $ rustlings hint macros1
/*
When you call a macro, you need to add something special compared to a
regular function call. If you're stuck, take a look at what's inside
`my_macro`.
*/