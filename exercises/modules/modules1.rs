// modules1.rs
// Make me compile! Execute `rustlings hint modules1` for hints :)

// ✓ I AM NOT DONE

mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    //fn make_sausage() {
    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}

// $ rustlings hint modules1
/*
Everything is private in Rust by default-- but there's a keyword we can use
to make something public! The compiler error should point to the thing that
needs to be public.
*/