// iterators4.rs

// ✓ I AM NOT DONE

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    //(2..=num).fold(1, |sum, v| sum * v)
    //(1..=num).reduce(|sum, v| sum * v).unwrap()
    (2..=num).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}

// $ rustlings hint iterators4
/*
In an imperative language, you might write a for loop that updates
a mutable variable. Or, you might write code utilizing recursion
and a match clause. In Rust you can take another functional
approach, computing the factorial elegantly with ranges and iterators.
*/