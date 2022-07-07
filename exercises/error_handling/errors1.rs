// errors1.rs
// This function refuses to generate text to be printed on a nametag if
// you pass it an empty string. It'd be nicer if it explained what the problem
// was, instead of just sometimes returning `None`. Thankfully, Rust has a similar
// construct to `Option` that can be used to express error conditions. Let's use it!
// Execute `rustlings hint errors1` for hints!

// ✓ I AM NOT DONE

//pub fn generate_nametag_text(name: String) -> Option<String> {
pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.len() > 0 {
        //Some(format!("Hi! My name is {}", name))
        Ok(format!("Hi! My name is {}", name))
    } else {
        // Empty names aren't allowed.
        //None
        Err("`name` was empty; it must be nonempty.".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}

// $ rustlings hint errors1
/*
`Ok` and `Err` are one of the variants of `Result`, so what the tests are saying
is that `generate_nametag_text` should return a `Result` instead of an
`Option`.

To make this change, you'll need to:
   - update the return type in the function signature to be a Result<String, String> that
     could be the variants `Ok(String)` and `Err(String)`
   - change the body of the function to return `Ok(stuff)` where it currently
     returns `Some(stuff)`
   - change the body of the function to return `Err(error message)` where it
     currently returns `None`
*/