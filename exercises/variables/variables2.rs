// variables2.rs
// Make me compile! Execute the command `rustlings hint variables2` if you want a hint :)

// ✓ I AM NOT DONE

fn main() {
    //let x;
    let x: u8 = 5;
    if x == 10 {
        println!("Ten!");
    } else {
        println!("Not ten!");
    }
}

// $ rustlings hint variables2
/*
The compiler message is saying that Rust cannot infer the type that the
variable binding `x` has with what is given here.
What happens if you annotate line 7 with a type annotation?
What if you give x a value?
What if you do both?
What type should x be, anyway?
What if x is the same type as 10? What if it's a different type?
*/