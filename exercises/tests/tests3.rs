// tests3.rs
// This test isn't testing our function -- make it do that in such a way that
// the test passes. Then write a second test that tests whether we get the result
// we expect to get when we call `is_even(5)`.
// Execute `rustlings hint tests3` for hints :)

// ✓ I AM NOT DONE

pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        //assert!();
        assert!(is_even(2));
    }

    #[test]
    fn is_false_when_odd() {
        //assert!();
        assert!(!is_even(1));
    }
}

// $ rustlings hint tests3
/*
You can call a function right where you're passing arguments to `assert!` -- so you could do
something like `assert!(having_fun())`. If you want to check that you indeed get false, you
can negate the result of what you're doing using `!`, like `assert!(!having_fun())`.
*/