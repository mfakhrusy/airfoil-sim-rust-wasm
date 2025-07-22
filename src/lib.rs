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
#[derive(Debug, Clone)]
pub struct AirfoilPoint {
    x: f64,
    y: f64,
}

#[wasm_bindgen]
impl AirfoilPoint {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> AirfoilPoint {
        AirfoilPoint { x, y }
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f64 {
        self.x
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f64 {
        self.y
    }
}

#[wasm_bindgen]
pub struct Airfoil {
    points: Vec<AirfoilPoint>,
    name: String,
}

#[wasm_bindgen]
impl Airfoil {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String) -> Airfoil {
        Airfoil {
            points: Vec::new(),
            name,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn point_count(&self) -> usize {
        self.points.len()
    }

    #[wasm_bindgen]
    pub fn get_point(&self, index: usize) -> Option<AirfoilPoint> {
        self.points.get(index).cloned()
    }

    #[wasm_bindgen]
    pub fn get_points_as_arrays(&self) -> js_sys::Object {
        let result = js_sys::Object::new();
        
        let x_coords: Vec<f64> = self.points.iter().map(|p| p.x).collect();
        let y_coords: Vec<f64> = self.points.iter().map(|p| p.y).collect();
        
        let x_array = js_sys::Array::new();
        let y_array = js_sys::Array::new();
        
        for x in x_coords {
            x_array.push(&JsValue::from_f64(x));
        }
        
        for y in y_coords {
            y_array.push(&JsValue::from_f64(y));
        }
        
        js_sys::Reflect::set(&result, &JsValue::from_str("x"), &x_array).unwrap();
        js_sys::Reflect::set(&result, &JsValue::from_str("y"), &y_array).unwrap();
        
        result
    }

    #[wasm_bindgen]
    pub fn add_point(&mut self, x: f64, y: f64) {
        self.points.push(AirfoilPoint::new(x, y));
    }

    #[wasm_bindgen]
    pub fn get_bounds(&self) -> js_sys::Object {
        if self.points.is_empty() {
            return js_sys::Object::new();
        }

        let mut min_x = self.points[0].x;
        let mut max_x = self.points[0].x;
        let mut min_y = self.points[0].y;
        let mut max_y = self.points[0].y;

        for point in &self.points {
            if point.x < min_x { min_x = point.x; }
            if point.x > max_x { max_x = point.x; }
            if point.y < min_y { min_y = point.y; }
            if point.y > max_y { max_y = point.y; }
        }

        let result = js_sys::Object::new();
        js_sys::Reflect::set(&result, &JsValue::from_str("minX"), &JsValue::from_f64(min_x)).unwrap();
        js_sys::Reflect::set(&result, &JsValue::from_str("maxX"), &JsValue::from_f64(max_x)).unwrap();
        js_sys::Reflect::set(&result, &JsValue::from_str("minY"), &JsValue::from_f64(min_y)).unwrap();
        js_sys::Reflect::set(&result, &JsValue::from_str("maxY"), &JsValue::from_f64(max_y)).unwrap();

        result
    }
}

#[wasm_bindgen]
pub fn parse_airfoil_dat(data: &str, name: &str) -> Result<Airfoil, JsValue> {
    console_log!("Parsing airfoil data for: {}", name);
    
    let mut airfoil = Airfoil::new(name.to_string());
    let mut line_count = 0;
    let mut error_count = 0;

    for line in data.lines() {
        line_count += 1;
        let trimmed = line.trim();
        
        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with(';') {
            continue;
        }

        // Parse the line - expect two numbers separated by whitespace
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        
        if parts.len() >= 2 {
            match (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
                (Ok(x), Ok(y)) => {
                    airfoil.add_point(x, y);
                }
                _ => {
                    error_count += 1;
                    console_log!("Warning: Could not parse line {}: '{}'", line_count, trimmed);
                }
            }
        } else if !trimmed.is_empty() {
            error_count += 1;
            console_log!("Warning: Invalid format on line {}: '{}'", line_count, trimmed);
        }
    }

    console_log!("Parsed {} points from {} lines ({} errors)", airfoil.point_count(), line_count, error_count);
    
    if airfoil.point_count() == 0 {
        return Err(JsValue::from_str("No valid points found in airfoil data"));
    }

    Ok(airfoil)
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
