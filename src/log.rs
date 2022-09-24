use crate::log;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

use std::string::ToString;
use strum_macros::Display;

fn default_txt() -> &'static str {
    return "\x1b[39m";
}

fn blue_txt() -> &'static str {
    return "\x1b[34m";
}

fn yellow_txt() -> &'static str {
    return "\x1b[33m";
}

fn red_txt() -> &'static str {
    return "\x1b[31m";
}

fn general_log(log_type: &str, color: &str, text: &str) {
    let output = format!("{}{: <3} {}{}", color, log_type, text, default_txt());

    #[cfg(target_arch = "wasm32")]
    {
        log(output.as_str())
    }

    println!("{}", output);
}

pub(crate) fn debug(text: &str) {
    // TODO: add color
    general_log("[D]", blue_txt(), text);
}

pub(crate) fn warn(text: &str) {
    // TODO: add color
    general_log("[W]", yellow_txt(), text);
}

pub(crate) fn error(text: &str) {
    // TODO: add color
    general_log("[E]", red_txt(), text);
}
