// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints

// ✓ I AM NOT DONE

fn main() {
    let optional_word = Some(String::from("rustlings"));
    // TODO: Make this an if let statement whose value is "Some" type
    //word = optional_word {
    if let Some(word) = optional_word {
        println!("The word is: {}", word);
    } else {
        println!("The optional word doesn't contain anything");
    }

    let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_integers_vec.push(Some(x));
    }

    // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let
    //integer = optional_integers_vec.pop() {
    while let Some(Some(integer)) = optional_integers_vec.pop() {
        println!("current value: {}", integer);
    }
}

// $ rustlings hint option2
/*
check out:
https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html
https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html

Remember that Options can be stacked in if let and while let.
For example: Some(Some(variable)) = variable2
Also see Option::flatten
*/