// errors5.rs

// This program uses a completed version of the code from errors4.
// It won't compile right now! Why?
// Execute `rustlings hint errors5` for hints!

// ✓ I AM NOT DONE

use std::error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum ParseIntErrorOrCreationError {
    PParseIntError(ParseIntError),
    PCreationError(CreationError),
}

impl ParseIntErrorOrCreationError {
    fn from_creation(err: CreationError) -> ParseIntErrorOrCreationError {
        ParseIntErrorOrCreationError::PCreationError(err)
    }

    fn from_parse_int(err: ParseIntError) -> ParseIntErrorOrCreationError {
        ParseIntErrorOrCreationError::PParseIntError(err)
    }
}

/*impl From<ParseIntError> for ParseIntErrorOrCreationError {
    fn from(err: ParseIntError) -> ParseIntErrorOrCreationError {
        ParseIntErrorOrCreationError::PParseIntError(err)
    }
}

impl From<CreationError> for ParseIntErrorOrCreationError {
    fn from(err: CreationError) -> ParseIntErrorOrCreationError {
        ParseIntErrorOrCreationError::PCreationError(err)
    }
}*/

// TODO: update the return type of `main()` to make this compile.
//fn main() -> Result<(), ParseIntError> {
//fn main() -> Result<(), Box<dyn error::Error>> {
fn main() -> Result<(), ParseIntErrorOrCreationError> {
    let pretend_user_input = "42";
    //let x: i64 = pretend_user_input.parse()?;
    let x: i64 = pretend_user_input.parse().map_err(ParseIntErrorOrCreationError::from_parse_int)?;
    //println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    println!("output={:?}", PositiveNonzeroInteger::new(x).map_err(ParseIntErrorOrCreationError::from_creation));
    Ok(())
}

// Don't change anything below this line.

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
            x => Ok(PositiveNonzeroInteger(x as u64))
        }
    }
}

// This is required so that `CreationError` can implement `error::Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl error::Error for CreationError {}

// $ rustlings hint errors5
/*
Hint: There are two different possible `Result` types produced within
`main()`, which are propagated using `?` operators. How do we declare a
return type from `main()` that allows both?

Another hint: under the hood, the `?` operator calls `From::from`
on the error value to convert it to a boxed trait object, a
`Box<dyn error::Error>`, which is polymorphic-- that means that lots of
different kinds of errors can be returned from the same function because
all errors act the same since they all implement the `error::Error` trait.
Check out this section of the book:
https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator

This exercise uses some concepts that we won't get to until later in the
course, like `Box` and the `From` trait. It's not important to understand
them in detail right now, but you can read ahead if you like.

Read more about boxing errors:
https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/boxing_errors.html

Read more about using the `?` operator with boxed errors:
https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/reenter_question_mark.html
*/