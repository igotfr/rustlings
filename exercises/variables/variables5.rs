// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)

// ✓ I AM NOT DONE

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    //number = 3;
    let number = 3;
    println!("Number plus two is : {}", number + 2);
}

// $ rustlings hint variables5
/*
In variables3 we already learned how to make an immutable variable mutable
using a special keyword. Unfortunately this doesn't help us much in this exercise
because we want to assign a different typed value to an existing variable. Sometimes
you may also like to reuse existing variable names because you are just converting
values to different types like in this exercise.
Fortunately Rust has a powerful solution to this problem: 'Shadowing'!
You can read more about 'Shadowing' in the book's section 'Variables and Mutability':
https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing
Try to solve this exercise afterwards using this technique.
*/