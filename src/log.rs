/// Color reference: https://docs.rs/embedded-text/0.4.0/embedded_text/style/index.html
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
    let output = format!(
        "{}{} {}{}{}",
        color,
        log_type,
        default_txt(),
        text,
        default_txt()
    );

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
