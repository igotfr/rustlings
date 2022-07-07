// structs3.rs
// Structs contain data, but can also have logic. In this exercise we have
// defined the Package struct and we want to test some logic attached to it.
// Make the code compile and the tests pass!
// If you have issues execute `rustlings hint structs3`

// ✓ I AM NOT DONE

#[derive(Debug)]
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: i32,
}

impl Package {
    fn new(sender_country: String, recipient_country: String, weight_in_grams: i32) -> Package {
        if weight_in_grams <= 0 {
            // panic statement goes here...
            panic!("Weight should be greater than zero!");
        } else {
            Package {
                sender_country,
                recipient_country,
                weight_in_grams,
            }
        }
    }

    //fn is_international(&self) -> ??? {
    fn is_international(&self) -> bool {
        // Something goes here...
        self.sender_country != self.recipient_country
    }

    //fn get_fees(&self, cents_per_gram: i32) -> ??? {
    fn get_fees(&self, cents_per_gram: i32) -> i32 {
        // Something goes here...
        self.weight_in_grams * cents_per_gram
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_creating_weightless_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Austria");

        Package::new(sender_country, recipient_country, -2210);
    }

    #[test]
    fn create_international_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Russia");

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(package.is_international());
    }

    #[test]
    fn create_local_package() {
        let sender_country = String::from("Canada");
        let recipient_country = sender_country.clone();

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(!package.is_international());
    }

    #[test]
    fn calculate_transport_fees() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Spain");

        let cents_per_gram = 3;

        let package = Package::new(sender_country, recipient_country, 1500);

        assert_eq!(package.get_fees(cents_per_gram), 4500);
    }
}

// $ rustlings hint structs3
/*
The new method needs to panic if the weight is physically impossible :), how do we do that in Rust?

For is_international: What makes a package international? Seems related to the places it goes through right?

For calculate_transport_fees: Bigger is more expensive usually, we don't have size, but something may fit the bill here :)

Have a look in The Book, to find out more about method implementations: https://doc.rust-lang.org/book/ch05-03-method-syntax.html
*/