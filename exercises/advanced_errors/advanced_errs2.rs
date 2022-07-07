// advanced_errs2.rs

// This exercise demonstrates a few traits that are useful for custom error
// types to implement, especially so that other code can consume the custom
// error type more usefully.

// Make this compile, and make the tests pass!
// Execute `rustlings hint advanced_errs2` for hints.

// Steps:
// 1. Implement a missing trait so that `main()` will compile.
// 2. Complete the partial implementation of `From` for
//    `ParseClimateError`.
// 3. Handle the missing error cases in the `FromStr` implementation for
//    `Climate`.
// 4. Complete the partial implementation of `Display` for
//    `ParseClimateError`.

// ✓ I AM NOT DONE

use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;

// This is the custom error type that we will be using for the parser for
// `Climate`.
#[derive(Debug, PartialEq)]
enum ParseClimateError {
    Empty,
    BadLen,
    NoCity,
    ParseInt(ParseIntError),
    ParseFloat(ParseFloatError),
}

// This `From` implementation allows the `?` operator to work on
// `ParseIntError` values.
impl From<ParseIntError> for ParseClimateError {
    fn from(e: ParseIntError) -> Self {
        Self::ParseInt(e)
    }
}

// This `From` implementation allows the `?` operator to work on
// `ParseFloatError` values.
impl From<ParseFloatError> for ParseClimateError {
    fn from(e: ParseFloatError) -> Self {
        // TODO: Complete this function
        Self::ParseFloat(e)
    }
}

// TODO: Implement a missing trait so that `main()` below will compile. It
// is not necessary to implement any methods inside the missing trait.
impl Error for ParseClimateError {}

// The `Display` trait allows for other code to obtain the error formatted
// as a user-visible string.
impl Display for ParseClimateError {
    // TODO: Complete this function so that it produces the correct strings
    // for each error variant.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // Imports the variants to make the following code more compact.
        use ParseClimateError::*;
        match self {
            Empty => write!(f, "empty input"),
            BadLen => write!(f, "incorrect number of fields"),
            NoCity => write!(f, "no city name"),
            ParseInt(_e) => write!(f, "error parsing year: invalid digit found in string"),
            ParseFloat(e) => write!(f, "error parsing temperature: {}", e),
            _ => write!(f, "unhandled error!"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Climate {
    city: String,
    year: u32,
    temp: f32,
}

// Parser for `Climate`.
// 1. Split the input string into 3 fields: city, year, temp.
// 2. Return an error if the string is empty or has the wrong number of
//    fields.
// 3. Return an error if the city name is empty.
// 4. Parse the year as a `u32` and return an error if that fails.
// 5. Parse the temp as a `f32` and return an error if that fails.
// 6. Return an `Ok` value containing the completed `Climate` value.
impl FromStr for Climate {
    type Err = ParseClimateError;
    // TODO: Complete this function by making it handle the missing error
    // cases.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseClimateError::Empty);
        }
        let v: Vec<_> = s.split(',').collect();
        let (city, year, temp) = match &v[..] {
            [city, year, temp] => (city.to_string(), year, temp),
            _ => return Err(ParseClimateError::BadLen),
        };
        if city.is_empty() {
            return Err(ParseClimateError::NoCity);
        }
        let year: u32 = year.parse()?;
        let temp: f32 = temp.parse()?;
        Ok(Climate { city, year, temp })
    }
}

// Don't change anything below this line (other than to enable ignored
// tests).

fn main() -> Result<(), Box<dyn Error>> {
    println!("{:?}", "Hong Kong,1999,25.7".parse::<Climate>()?);
    println!("{:?}", "".parse::<Climate>()?);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_empty() {
        let res = "".parse::<Climate>();
        assert_eq!(res, Err(ParseClimateError::Empty));
        assert_eq!(res.unwrap_err().to_string(), "empty input");
    }
    #[test]
    fn test_short() {
        let res = "Boston,1991".parse::<Climate>();
        assert_eq!(res, Err(ParseClimateError::BadLen));
        assert_eq!(res.unwrap_err().to_string(), "incorrect number of fields");
    }
    #[test]
    fn test_long() {
        let res = "Paris,1920,17.2,extra".parse::<Climate>();
        assert_eq!(res, Err(ParseClimateError::BadLen));
        assert_eq!(res.unwrap_err().to_string(), "incorrect number of fields");
    }
    #[test]
    fn test_no_city() {
        let res = ",1997,20.5".parse::<Climate>();
        assert_eq!(res, Err(ParseClimateError::NoCity));
        assert_eq!(res.unwrap_err().to_string(), "no city name");
    }
    #[test]
    fn test_parse_int_neg() {
        let res = "Barcelona,-25,22.3".parse::<Climate>();
        assert!(matches!(res, Err(ParseClimateError::ParseInt(_))));
        let err = res.unwrap_err();
        if let ParseClimateError::ParseInt(ref inner) = err {
            assert_eq!(
                err.to_string(),
                format!("error parsing year: {}", inner.to_string())
            );
        } else {
            unreachable!();
        };
    }
    #[test]
    fn test_parse_int_bad() {
        let res = "Beijing,foo,15.0".parse::<Climate>();
        assert!(matches!(res, Err(ParseClimateError::ParseInt(_))));
        let err = res.unwrap_err();
        if let ParseClimateError::ParseInt(ref inner) = err {
            assert_eq!(
                err.to_string(),
                format!("error parsing year: {}", inner.to_string())
            );
        } else {
            unreachable!();
        };
    }
    #[test]
    fn test_parse_float() {
        let res = "Manila,2001,bar".parse::<Climate>();
        assert!(matches!(res, Err(ParseClimateError::ParseFloat(_))));
        let err = res.unwrap_err();
        if let ParseClimateError::ParseFloat(ref inner) = err {
            assert_eq!(
                err.to_string(),
                format!("error parsing temperature: {}", inner.to_string())
            );
        } else {
            unreachable!();
        };
    }
    #[test]
    fn test_parse_good() {
        let res = "Munich,2015,23.1".parse::<Climate>();
        assert_eq!(
            res,
            Ok(Climate {
                city: "Munich".to_string(),
                year: 2015,
                temp: 23.1,
            })
        );
    }
    #[test]
    #[ignore]
    fn test_downcast() {
        let res = "São Paulo,-21,28.5".parse::<Climate>();
        assert!(matches!(res, Err(ParseClimateError::ParseInt(_))));
        let err = res.unwrap_err();
        let inner: Option<&(dyn Error + 'static)> = err.source();
        assert!(inner.is_some());
        assert!(inner.unwrap().is::<ParseIntError>());
    }
}

// $ rustlings hint advanced_errs2
/*
This exercise demonstrates a few traits that are useful for custom error
types to implement. These traits make it easier for other code to consume
the custom error type.

Follow the steps in the comment near the top of the file. You will have to
supply a missing trait implementation, and complete a few incomplete ones.

You may find these pages to be helpful references:
https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/define_error_type.html
https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/boxing_errors.html
https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/wrap_error.html

Hint: What trait must our error type have for `main()` to return the return
type that it returns?

Another hint: It's not necessary to implement any methods inside the missing
trait. (Some methods have default implementations that are supplied by the
trait.)

Another hint: Consult the tests to determine which error variants (and which
error message text) to produce for certain error conditions.

Challenge: There is one test that is marked `#[ignore]`. Can you supply the
missing code that will make it pass? You may want to consult the standard
library documentation for a certain trait for more hints.
*/