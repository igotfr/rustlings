// advanced_errs1.rs

// Remember back in errors6, we had multiple mapping functions so that we
// could translate lower-level errors into our custom error type using
// `map_err()`? What if we could use the `?` operator directly instead?

// Make this code compile! Execute `rustlings hint advanced_errs1` for
// hints :)

// ✓ I AM NOT DONE

use std::num::ParseIntError;
use std::str::FromStr;

// This is a custom error type that we will be using in the `FromStr`
// implementation.
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

impl From<CreationError> for ParsePosNonzeroError {
    fn from(e: CreationError) -> Self {
        // TODO: complete this implementation so that the `?` operator will
        // work for `CreationError`
        ParsePosNonzeroError::Creation(e)
    }
}

// TODO: implement another instance of the `From` trait here so that the
// `?` operator will work in the other place in the `FromStr`
// implementation below.
impl From<ParseIntError> for ParsePosNonzeroError {
    fn from(e: ParseIntError) -> Self {
        ParsePosNonzeroError::ParseInt(e)
    }
}

// Don't change anything below this line.

impl FromStr for PositiveNonzeroInteger {
    type Err = ParsePosNonzeroError;
    fn from_str(s: &str) -> Result<PositiveNonzeroInteger, Self::Err> {
        let x: i64 = s.parse()?;
        Ok(PositiveNonzeroInteger::new(x)?)
    }
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        // We can't construct a ParseIntError, so we have to pattern match.
        assert!(matches!(
            PositiveNonzeroInteger::from_str("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_))
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            PositiveNonzeroInteger::from_str("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative))
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            PositiveNonzeroInteger::from_str("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero))
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42);
        assert!(x.is_ok());
        assert_eq!(PositiveNonzeroInteger::from_str("42"), Ok(x.unwrap()));
    }
}

// $ rustlings hint advanced_errs1
/*
This exercise uses an updated version of the code in errors6. The parsing
code is now in an implementation of the `FromStr` trait. Note that the
parsing code uses `?` directly, without any calls to `map_err()`. There is
one partial implementation of the `From` trait example that you should
complete.

Details: The `?` operator calls `From::from()` on the error type to convert
it to the error type of the return type of the surrounding function.

Hint: You will need to write another implementation of `From` that has a
different input type.
*/