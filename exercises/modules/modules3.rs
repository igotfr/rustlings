// modules3.rs
// You can use the 'use' keyword to bring module paths from modules from anywhere
// and especially from the Rust standard library into your scope.
// Bring SystemTime and UNIX_EPOCH
// from the std::time module. Bonus style points if you can do it with one line!
// Make me compile! Execute `rustlings hint modules3` for hints :)

// ✓ I AM NOT DONE

// TODO: Complete this use statement
//use ???
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

// $ rustlings hint modules3
/*
UNIX_EPOCH and SystemTime are declared in the std::time module. Add a use statement
for these two to bring them into scope. You can use nested paths or the glob
operator to bring these two in using only one line.
*/