// move_semantics1.rs
// Make me compile! Execute `rustlings hint move_semantics1` for hints :)

// ✓ I AM NOT DONE

fn main() {
    let vec0 = Vec::new();

    //let vec1 = fill_vec(vec0);
    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

// $ rustlings hint move_semantics1
/*
So you've got the "cannot borrow immutable local variable `vec1` as mutable" error on line 13, right?
The fix for this is going to be adding one keyword, and the addition is NOT on line 13 where the error is.
*/