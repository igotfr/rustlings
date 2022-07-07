// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` for hints :)

// ✓ I AM NOT DONE

#[macro_use]
mod macros {
    //#[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}

// $ rustlings hint macros3
/*
In order to use a macro outside of its module, you need to do something
special to the module to lift the macro out into its parent.

The same trick also works on "extern crate" statements for crates that have
exported macros, if you've seen any of those around.
*/