// modules2.rs
// You can bring module paths into scopes and provide new names for them with the
// 'use' and 'as' keywords. Fix these 'use' statements to make the code compile.
// Make me compile! Execute `rustlings hint modules2` for hints :)

// ✓ I AM NOT DONE

mod delicious_snacks {

    // TODO: Fix these use statements
    //use self::fruits::PEAR as ???
    //use self::veggies::CUCUMBER as ???
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}

// $ rustlings hint modules2
/*
The delicious_snacks module is trying to present an external interface that is
different than its internal structure (the `fruits` and `veggies` modules and
associated constants). Complete the `use` statements to fit the uses in main and
find the one keyword missing for both constants.
*/