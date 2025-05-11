// main.js: Interacts with the Rust/WASM module.

import init, { greet } from './rust_lib/pkg/rust_lib.js';

async function runWasm() {
    try {
        await init(); // Initialize the WASM module.

        const nameInput = document.getElementById('name-input');
        const greetButton = document.getElementById('greet-button');
        const rustAppContainer = document.getElementById('rust-app-container');

        if (!nameInput || !greetButton || !rustAppContainer) {
            console.error("Required HTML elements not found (input, button, or container).");
            if (rustAppContainer) {
                rustAppContainer.textContent = "Error: Page elements missing. Check console.";
            }
            return;
        }

        // Function to update the greeting
        function updateGreeting(name) {
            const greeting = greet(name);
            rustAppContainer.textContent = greeting;
        }

        // Initial greeting on load with the default input value
        if (nameInput.value) {
            updateGreeting(nameInput.value);
        }

        // Event listener for the button
        greetButton.addEventListener('click', () => {
            const userName = nameInput.value;
            if (userName) {
                updateGreeting(userName);
            } else {
                rustAppContainer.textContent = "Please enter a name first!";
            }
        });

    } catch (error) {
        console.error("WASM Error:", error);
        const rustAppContainer = document.getElementById('rust-app-container'); // Re-fetch in case of early error
        if (rustAppContainer) {
            rustAppContainer.textContent = "Failed to load Rust app. Check console.";
        }
    }
}

runWasm();
