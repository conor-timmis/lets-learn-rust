// main.js: Interacts with the Rust/WASM module.

// Import the initializer and the `greet` function from our WASM package.
import init, { greet } from './rust_lib/pkg/rust_lib.js';

async function runWasm() {
    try {
        // Initialize the WASM module (loads the .wasm file).
        await init();

        // Call the Rust `greet` function.
        const nameToSend = "Web Page User";
        const greeting = greet(nameToSend);

        // Display the greeting in the HTML.
        const rustAppContainer = document.getElementById('rust-app-container');
        if (rustAppContainer) {
            rustAppContainer.textContent = greeting;
        } else {
            console.error("Element 'rust-app-container' not found.");
        }

    } catch (error) {
        console.error("WASM Error:", error);
        const rustAppContainer = document.getElementById('rust-app-container');
        if (rustAppContainer) {
            rustAppContainer.textContent = "Failed to load Rust app. Check console.";
        }
    }
}

// Execute the main WASM interaction logic.
runWasm();
