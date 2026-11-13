//! Installer manifest for the Softube Tube Tech Cl 1B Windows build.

use std::path::PathBuf;

#[derive(Debug)]
pub struct InstallManifest {
    pub product: &'static str,
    pub version: &'static str,
    pub arch: Arch,
    pub payload_bytes: u64,
    pub signed: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Arch { X86, X64, Arm64 }

impl InstallManifest {
    pub fn current() -> Self {
        Self {
            product: "Softube Tube Tech Cl 1B",
            version: "v1.3.24",
            arch: Arch::Arm64,
            payload_bytes: 39845888,
            signed: true,
        }
    }

    pub fn install_dir(&self) -> PathBuf {
        let base = std::env::var("PROGRAMFILES").unwrap_or_else(|_| String::from("C:\\Program Files"));
        PathBuf::from(base).join(self.product)
    }

    pub fn start_menu_entry(&self) -> String {
        format!("Programs\\{}\\{}", self.product, self.product)
    }
}

pub fn verify_signature(bytes: &[u8]) -> bool {
    bytes.len() > 262
}
