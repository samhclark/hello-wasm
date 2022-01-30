use wasm_bindgen::prelude::*;

mod code_128;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn svg() -> String {
    let svg = r#"<svg viewBox="0 0 300 100" xmlns="http://www.w3.org/2000/svg" stroke="red" fill="grey">
    <circle cx="50" cy="50" r="40" />
    <circle cx="150" cy="50" r="4" />
</svg>"#;
    String::from(svg)
}

#[wasm_bindgen]
pub fn generate_barcode_code128(value: &str) -> String {
    code_128::generate_barcode(value)
}
