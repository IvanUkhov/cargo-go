// The code is borrowed from and should be kept in sync with:
// https://github.com/rust-lang/cargo/blob/master/src/cargo/ops/cargo_doc.rs

use std::process::Command;

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub fn open(path: &str) -> Result<&'static str, Vec<&'static str>> {
    use std::env;

    let mut methods = vec![];
    if let Ok(name) = env::var("BROWSER") {
        match Command::new(name).arg(path).status() {
            Ok(_) => return Ok("$BROWSER"),
            Err(_) => methods.push("$BROWSER"),
        }
    }
    for method in ["xdg-open", "gnome-open", "kde-open"].iter() {
        match Command::new(method).arg(path).status() {
            Ok(_) => return Ok(method),
            Err(_) => methods.push(method),
        }
    }
    Err(methods)
}

#[cfg(target_os = "macos")]
pub fn open(path: &str) -> Result<&'static str, Vec<&'static str>> {
    match Command::new("open").arg(path).status() {
        Ok(_) => Ok("open"),
        Err(_) => Err(vec!["open"]),
    }
}

#[cfg(target_os = "windows")]
pub fn open(path: &str) -> Result<&'static str, Vec<&'static str>> {
    match Command::new("cmd").arg("/C").arg("start").arg("").arg(path).status() {
        Ok(_) => Ok("cmd /C start"),
        Err(_) => Err(vec!["cmd /C start"]),
    }
}
