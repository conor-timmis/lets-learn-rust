use wasm_bindgen::prelude::*;

// This is the entry point for the WASM module.
// wasm_bindgen is a macro that makes this Rust function
// callable from JavaScript.
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello from Rust, {}!", name)
}

// You can keep your tests or add new ones for your WASM functions
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_works() {
        assert_eq!(greet("WebAssembly"), "Hello from Rust, WebAssembly!");
    }
}
