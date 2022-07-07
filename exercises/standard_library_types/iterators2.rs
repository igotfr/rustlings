// iterators2.rs
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
// As always, there are hints if you execute `rustlings hint iterators2`!

// ✓ I AM NOT DONE

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        //Some(first) => ???,
        //Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
        Some(first) => first.to_uppercase().to_string() + c.as_str(),
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    //vec![]
    //words.iter().map(|word| capitalize_first(word)).collect()
    words.iter().map(|&word| capitalize_first(word)).collect()
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    //String::new()
    //words.iter().map(|&word| capitalize_first(word)).collect::<Vec<String>>().join("")
    //words.iter().map(|word| capitalize_first(word)).collect::<String>()
    words.iter().map(|&word| capitalize_first(word)).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}

// $ rustlings hint iterators2
/*
Step 1
The variable `first` is a `char`. It needs to be capitalized and added to the
remaining characters in `c` in order to return the correct `String`.
The remaining characters in `c` can be viewed as a string slice using the
`as_str` method.
The documentation for `char` contains many useful methods.
https://doc.rust-lang.org/std/primitive.char.html

Step 2
Create an iterator from the slice. Transform the iterated values by applying
the `capitalize_first` function. Remember to collect the iterator.

Step 3.
This is surprising similar to the previous solution. Collect is very powerful
and very general. Rust just needs to know the desired type.
*/