// main.js: Interacts with the Rust/WASM module.

// Import functions from our WASM package.
import init, { greet, add_numbers_and_explain } from './rust_lib/pkg/rust_lib.js';

// --- Rust Code Snippets (for display) ---
const greetCode = `
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello from Rust, {}! I am learning too!", name)
}`.trim();

const addNumbersCode = `
#[wasm_bindgen]
pub fn add_numbers_and_explain(num1: i32, num2: i32) -> String {
    let sum = num1 + num2;
    format!("Rust says: {} + {} = {}. Perfect!", num1, num2, sum)
}`.trim();

async function runWasm() {
    try {
        await init(); // Initialize the WASM module.

        // --- Greet Example Elements ---
        const nameInput = document.getElementById('name-input');
        const greetButton = document.getElementById('greet-button');
        const rustAppContainer = document.getElementById('rust-app-container');
        const greetCodeDisplay = document.getElementById('greet-code-display');

        // --- Adder Example Elements ---
        const num1Input = document.getElementById('num1-input');
        const num2Input = document.getElementById('num2-input');
        const addNumbersButton = document.getElementById('add-numbers-button');
        const adderResultContainer = document.getElementById('adder-result-container');
        const adderCodeDisplay = document.getElementById('adder-code-display');

        // --- Check if all elements exist ---
        if (!nameInput || !greetButton || !rustAppContainer || !greetCodeDisplay || 
            !num1Input || !num2Input || !addNumbersButton || !adderResultContainer || !adderCodeDisplay) {
            console.error("Required HTML elements not found. Check IDs.");
            // Display a general error if a primary container is missing
            const mainErrorContainer = rustAppContainer || adderResultContainer;
            if (mainErrorContainer) {
                mainErrorContainer.textContent = "Error: Page elements missing. Check console.";
            }
            return;
        }

        // --- Populate Code Displays ---
        greetCodeDisplay.textContent = greetCode;
        adderCodeDisplay.textContent = addNumbersCode;

        // --- Greet Example Logic ---
        function updateGreeting(name) {
            rustAppContainer.textContent = greet(name);
        }
        if (nameInput.value) { // Initial greeting
            updateGreeting(nameInput.value);
        }
        greetButton.addEventListener('click', () => {
            const userName = nameInput.value.trim();
            if (userName) {
                updateGreeting(userName);
            } else {
                rustAppContainer.textContent = "Please enter a name first!";
            }
        });

        // --- Adder Example Logic ---
        function updateAdderResult(val1, val2) {
            const num1 = parseInt(val1, 10);
            const num2 = parseInt(val2, 10);

            if (isNaN(num1) || isNaN(num2)) {
                adderResultContainer.textContent = "Please enter valid numbers.";
                return;
            }
            adderResultContainer.textContent = add_numbers_and_explain(num1, num2);
        }
        if (num1Input.value && num2Input.value) { // Initial sum
            updateAdderResult(num1Input.value, num2Input.value);
        }
        addNumbersButton.addEventListener('click', () => {
            updateAdderResult(num1Input.value, num2Input.value);
        });

    } catch (error) {
        console.error("WASM Error:", error);
        const appContainer = document.getElementById('rust-app-container') || document.getElementById('adder-result-container');
        if (appContainer) {
            appContainer.textContent = "Failed to load Rust app. Check console.";
        }
    }
}

runWasm();
