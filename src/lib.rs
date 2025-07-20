mod utils;

use wasm_bindgen::prelude::*;

// Import the `console.log` function from the `console` module
#[wasm_bindgen]
extern "C" {
    // Bind the `alert` function
    fn alert(s: &str);
    
    // Bind the `console.log` function
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Define a macro for easier console logging
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, airfoil-sim-rust-wasm!");
}

#[wasm_bindgen]
pub fn greet_console() {
    console_log!("Hello from Rust WASM! 🦀");
    console_log!("This is the airfoil-sim-rust-wasm project!");
}

#[wasm_bindgen]
pub fn get_greeting() -> String {
    "Hello World from Rust WASM! 🛩️✨".to_string()
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    console_log!("Adding {} + {} = {}", a, b, a + b);
    a + b
}
