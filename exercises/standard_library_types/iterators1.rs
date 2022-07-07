// iterators1.rs
//
//  Make me compile by filling in the `???`s
//
// When performing operations on elements within a collection, iterators are essential.
// This module helps you get familiar with the structure of using an iterator and
// how to go through elements within an iterable collection.
//
// Execute `rustlings hint iterators1` for hints :D

// ✓ I AM NOT DONE

fn main () {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    //let mut my_iterable_fav_fruits = ???;   // TODO: Step 1
    let mut my_iterable_fav_fruits = my_fav_fruits.iter();   // TODO: Step 1

    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    //assert_eq!(my_iterable_fav_fruits.next(), ???);     // TODO: Step 2
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));     // TODO: Step 2
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    //assert_eq!(my_iterable_fav_fruits.next(), ???);     // TODO: Step 3
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));     // TODO: Step 3
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    //assert_eq!(my_iterable_fav_fruits.next(), ???);     // TODO: Step 4
    assert_eq!(my_iterable_fav_fruits.next(), None);     // TODO: Step 4
}

// $ rustlings hint iterators1
/*
Step 1:
We need to apply something to the collection `my_fav_fruits` before we start to go through
it. What could that be? Take a look at the struct definition for a vector for inspiration:
https://doc.rust-lang.org/std/vec/struct.Vec.html.
Step 2 & step 2.1:
Very similar to the lines above and below. You've got this!
Step 3:
An iterator goes through all elements in a collection, but what if we've run out of
elements? What should we expect here? If you're stuck, take a look at
https://doc.rust-lang.org/std/iter/trait.Iterator.html for some ideas.
*/