# Let's Learn Rust!

This repository is a learning project for Rust, with the goal of creating a web-based manual for other Rust beginners.

## Live Demo

You can try out the interactive manual I've put together here: [https://conor-timmis.github.io/lets-learn-rust/](https://conor-timmis.github.io/lets-learn-rust/)

## Disclaimer

I am also learning Rust as I build this project and have very little prior experience with the language. This is a learn-as-I-go adventure! Any contributions, corrections, or suggestions are highly welcome.

## Goal

The aim is to create a simple, classic-themed (I've since chosen Windows 98) web page that serves as an interactive manual for learning Rust. The Rust application itself is compiled to WebAssembly (WASM) to run in the browser.

## Current Features

The manual currently includes the following interactive Rust examples, compiled to WebAssembly:

*   **Greeter:** A simple function that takes a name and returns a personalized greeting.
*   **Number Adder:** Adds two numbers and displays the result with a message.
*   **String Reverser:** Reverses the characters of a given string.
*   **Statistics Calculator:** Calculates basic statistics (count, sum, average, minimum, maximum) from a comma-separated list of numbers.

Each example also displays the corresponding Rust code snippet.

## Tech Stack

*   **Rust:** For the core logic of the examples.
*   **HTML & CSS:** For the structure and classic styling of the web page.
*   **JavaScript (Vanilla):** For loading the WASM module and handling DOM interactions.
*   **WebAssembly (WASM):** Compiled target for running Rust in the browser.
*   **`wasm-pack`:** Tool for building and packaging Rust-generated WebAssembly.

Let's learn together. 
