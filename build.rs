use std::env;
use std::path::Path;

/// Links both libfreefare & libnfc
/// 
/// Allow custom paths at LIBFREEFARE_PATH and LIBNFC_PATH if preferred
/// Assume default Homebrew installation location on Mac OS; can be overriden as mentioned above
/// Verify paths exist before linking
///
fn main() {
    let libfreefare_path = env::var("LIBFREEFARE_PATH").unwrap_or_else(|_| {
        if cfg!(target_os = "macos") {
            "/opt/homebrew/opt/libfreefare/lib".to_string()
        } else if cfg!(target_os = "linux") {
            "/usr/lib".to_string()
        } else {
            panic!("Unsupported platform. Please set LIBFREEFARE_PATH.");
        }
    });

    if !Path::new(&libfreefare_path).exists() {
        panic!(
            "Could not find libfreefare at '{}'. Please install libfreefare or set LIBFREEFARE_PATH.",
            libfreefare_path
        );
    }

    println!("cargo:rustc-link-search=native={}", libfreefare_path);
    println!("cargo:rustc-link-lib=freefare");

    let libnfc_path = env::var("LIBNFC_PATH").unwrap_or_else(|_| {
        if cfg!(target_os = "macos") {
            "/opt/homebrew/opt/libnfc/lib".to_string()
        } else if cfg!(target_os = "linux") {
            "/usr/lib".to_string()
        } else {
            panic!("Unsupported platform. Please set LIBNFC_PATH.");
        }
    });

    if !Path::new(&libnfc_path).exists() {
        panic!(
            "Could not find libnfc at '{}'. Please install libnfc or set LIBNFC_PATH.",
            libnfc_path
        );
    }

    println!("cargo:rustc-link-search=native={}", libnfc_path);
    println!("cargo:rustc-link-lib=nfc");
}
