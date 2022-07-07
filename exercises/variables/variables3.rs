// variables3.rs
// Make me compile! Execute the command `rustlings hint variables3` if you want a hint :)

// ✓ I AM NOT DONE

fn main() {
    //let x = 3;
    let mut x = 3;
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}

// $ rustlings hint variables3
/*
In Rust, variable bindings are immutable by default. But here we're trying
to reassign a different value to x! There's a keyword we can use to make
a variable binding mutable instead.
*/