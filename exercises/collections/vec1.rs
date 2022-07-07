// vec1.rs
// Your task is to create a `Vec` which holds the exact same elements
// as in the array `a`.
// Make me compile and pass the test!
// Execute the command `rustlings hint vec1` if you need hints.

// ✓ I AM NOT DONE

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array
    //let v = // TODO: declare your vector here with the macro for vectors
    let v = a.to_vec();
    //let v = a.iter().copied().collect::<Vec<i32>>();
    //let v = [].iter().chain(&a).copied().collect::<Vec<i32>>();
    /*let mut v = Vec::<i32>::from([0; 4]);
    v.copy_from_slice(&a);*/
    /*let mut v = Vec::<i32>::new();
    v.extend_from_slice(&a);*/

    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}

// $ rustlings hint vec1
/*
In Rust, there are two ways to define a Vector.
1. One way is to use the `Vec::new()` function to create a new vector
  and fill it with the `push()` method.
2. The second way, which is simpler is to use the `vec![]` macro and
  define your elements inside the square brackets.
Check this chapter: https://doc.rust-lang.org/stable/book/ch08-01-vectors.html
of the Rust book to learn more.
*/