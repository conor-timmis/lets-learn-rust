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

// --- Calculate Stats Function ---
#[wasm_bindgen]
pub fn calculate_stats(numbers_str: &str) -> String {
    if numbers_str.trim().is_empty() {
        return String::from("Input is empty. Please provide some numbers.");
    }

    let mut numbers: Vec<i32> = Vec::new();
    let parts = numbers_str.split(',');

    for part in parts {
        match part.trim().parse::<i32>() {
            Ok(num) => numbers.push(num),
            Err(_) => return format!("Error: Could not parse '{}' as a number. Please use comma-separated integers.", part.trim()),
        }
    }

    if numbers.is_empty() {
        return String::from("No valid numbers found to calculate statistics.");
    }

    let count = numbers.len();
    let sum: i32 = numbers.iter().sum();
    let average = sum as f64 / count as f64;
    let min_val = *numbers.iter().min().unwrap(); 
    let max_val = *numbers.iter().max().unwrap();

    format!(
        "Statistics Results:\nCount: {}\nSum: {}\nAverage: {:.2}\nMinimum: {}\nMaximum: {}",
        count, sum, average, min_val, max_val
    )
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
        assert_eq!(reverse_string("你好"), "好你");
    }

    #[test]
    fn calculate_stats_works_for_valid_input() {
        let expected = "Statistics Results:\nCount: 5\nSum: 26\nAverage: 5.20\nMinimum: 2\nMaximum: 10";
        assert_eq!(calculate_stats("5, 2, 10, 3, 6"), expected);
    }

    #[test]
    fn calculate_stats_handles_empty_input() {
        assert_eq!(calculate_stats(""), "Input is empty. Please provide some numbers.");
        assert_eq!(calculate_stats("   "), "Input is empty. Please provide some numbers.");
    }

    #[test]
    fn calculate_stats_handles_invalid_number() {
        assert_eq!(calculate_stats("1, two, 3"), "Error: Could not parse 'two' as a number. Please use comma-separated integers.");
    }

    #[test]
    fn calculate_stats_handles_single_number() {
        let expected = "Statistics Results:\nCount: 1\nSum: 42\nAverage: 42.00\nMinimum: 42\nMaximum: 42";
        assert_eq!(calculate_stats("42"), expected);
    }

    #[test]
    fn calculate_stats_handles_numbers_with_spaces() {
        let expected = "Statistics Results:\nCount: 3\nSum: 6\nAverage: 2.00\nMinimum: 1\nMaximum: 3";
        assert_eq!(calculate_stats(" 1 , 2 , 3 "), expected);
    }

     #[test]
    fn calculate_stats_no_valid_numbers_found() {
        assert_eq!(calculate_stats(",,"), "No valid numbers found to calculate statistics.");
    }
}