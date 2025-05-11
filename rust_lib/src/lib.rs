use wasm_bindgen::prelude::*;

// This is our first Rust function that we want to call from JavaScript.
// The `#[wasm_bindgen]` attribute makes this possible.
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    // `format!` is a Rust macro for creating strings.
    // `&str` is a string slice (a reference to a string).
    // `String` is an owned string type.
    format!("Hello from Rust, {}! I am learning too!", name)
}

// It's good practice to include tests for your functions.
// These tests will run when you use `cargo test`.
#[cfg(test)]
mod tests {
    // `use super::*;` brings everything from the parent module (our lib) into scope for the tests.
    use super::*;

    #[test]
    fn greet_returns_expected_message() {
        let name_to_greet = "WASM Explorer";
        let expected_message = format!("Hello from Rust, {}! I am learning too!", name_to_greet);
        assert_eq!(greet(name_to_greet), expected_message);
    }
}
