#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn general_log(log_type: &str, text: &str) {
    let output = format!("{: <8}{} {}", log_type, ":", text);

    #[cfg(target_arch = "wasm32")]
    {
        log(output.as_str())
    }

    println!("{}", output);
}

pub(crate) fn debug(text: &str) {
    // TODO: add color
    general_log("Debug", text);
}

pub(crate) fn warn(text: &str) {
    // TODO: add color
    general_log("Warning", text);
}

pub(crate) fn error(text: &str) {
    // TODO: add color
    general_log("Error", text);
}
