use wasm_bindgen::prelude::*;

// --- Greet Function ---
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello from Rust, {}! I am learning too!", name)
}

// --- Add Numbers Function ---
#[wasm_bindgen]
pub fn add_numbers_and_explain(num1: i32, num2: i32) -> String {
    let sum = num1 + num2;
    format!("Rust says: {} + {} = {}. Perfect!", num1, num2, sum)
}

// --- Reverse String Function ---
#[wasm_bindgen]
pub fn reverse_string(text: &str) -> String {
    text.chars().rev().collect::<String>()
}

// --- Tests ---
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_returns_expected_message() {
        let name_to_greet = "WASM Explorer";
        let expected_message = format!("Hello from Rust, {}! I am learning too!", name_to_greet);
        assert_eq!(greet(name_to_greet), expected_message);
    }

    #[test]
    fn add_numbers_works_and_explains() {
        assert_eq!(add_numbers_and_explain(5, 7), "Rust says: 5 + 7 = 12. Perfect!");
        assert_eq!(add_numbers_and_explain(-1, 10), "Rust says: -1 + 10 = 9. Perfect!");
    }

    #[test]
    fn reverse_string_works() {
        assert_eq!(reverse_string("hello"), "olleh");
        assert_eq!(reverse_string("Rust"), "tsuR");
        assert_eq!(reverse_string("-"), "-");
        assert_eq!(reverse_string(""), "");
        // Note: This simple reversal might not handle all Unicode grapheme clusters correctly,
        // but it's good for a basic example of string manipulation.
        assert_eq!(reverse_string("你好"), "好你"); // Reverses Unicode characters
    }
}
