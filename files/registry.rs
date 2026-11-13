//! Windows registry read helpers for Softube Tube Tech Cl 1B.

pub const APP_KEY: &str = "Software\\Softube\\Softube Tube Tech Cl 1B";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Softube Tube Tech Cl 1B";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Softube Tube Tech Cl 1B") },
        RegValue { name: "Version",     value: String::from("v1.3.24") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Softube Tube Tech Cl 1B") },
        RegValue { name: "Publisher",       value: String::from("Softube") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Softube Tube Tech Cl 1B\\uninstall.exe\"") },
    ]
}
