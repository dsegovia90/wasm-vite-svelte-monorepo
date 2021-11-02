mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: String);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "number[]")]
    pub type ArrayOfNumbers;
}

#[wasm_bindgen(js_name="addArray")]
pub fn add_array(arr: ArrayOfNumbers) -> u32 {
    let rust_arr: Vec<u32> = arr.into_serde().unwrap();
    let mut sum: u32 = 0;
    for element in rust_arr {
        sum = element + sum
    }
    return sum
}

#[wasm_bindgen(js_name="helloWorld")]
pub fn hello_world(name: Option<String>) {
    match name {
        Some(name) => alert(format!("Hello {}", name)),
        None => alert(String::from("Hello world!"))
    }
}
