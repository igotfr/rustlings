// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` for hints :)

// ✓ I AM NOT DONE

fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    let z = &mut x;
    //*y += 100;
    *z += 1000;
    assert_eq!(x, 1200);
}

// $ rustlings hint move_semantics5
/*
Carefully reason about the range in which each mutable reference is in
vogue. Does it help to update the value of referent (x) immediately after
the mutable reference is taken? Read more about 'Mutable References'
in the book's section References and Borrowing':
https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references.
*/