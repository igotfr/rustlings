// structs1.rs
// Address all the TODOs to make the tests pass!

// ✓ I AM NOT DONE

struct ColorClassicStruct {
    // TODO: Something goes here
    name: String,
    hex: String,
}

struct ColorTupleStruct(String, String/* TODO: Something goes here */);

#[derive(Debug)]
struct UnitStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        // let green =
        let green = ColorClassicStruct { name: String::from("green"), hex: String::from("#00FF00") };

        assert_eq!(green.name, "green");
        assert_eq!(green.hex, "#00FF00");
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        // let green =
        let green = ColorTupleStruct(String::from("green"), String::from("#00FF00"));

        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct!
        // let unit_struct =
        let unit_struct = UnitStruct;
        let message = format!("{:?}s are fun!", unit_struct);

        assert_eq!(message, "UnitStructs are fun!");
    }
}

// $ rustlings hint structs1.rs
/*
Rust has more than one type of struct. Three actually, all variants are used to package related data together.
There are normal (or classic) structs. These are named collections of related data stored in fields.
Tuple structs are basically just named tuples.
Finally, Unit structs. These don't have any fields and are useful for generics.

In this exercise you need to complete and implement one of each kind.
Read more about structs in The Book: https://doc.rust-lang.org/book/ch05-01-defining-structs.html
*/