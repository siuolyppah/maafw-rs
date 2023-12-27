//! Finds `MaaFramework` and `MaaToolKit` shared libraries and links to them.
//!
//! # Environment Variables
//!
//! This build script can make use of several environment variables to help it find the required shared
//! libraries. The environment variables will be resolved by order until the libraries are found.
//!
//! * `LIBMAAFW_PATH` - provides a path to a directory containing `MaaFramework`and `MaaToolKit` shared libraries
//! * `LIBMAAFRAMEWORK_PATH` - provides a path to the `MaaFramework` shared library (TODO)
//! * `LIBMAATOOLKIT_PATH` - provides a path to the `MaaToolKit` shared library (TODO)

use std::env;
use std::path::Path;


const SEARCH_LINUX: &[&str] = &[
    "/usr/lib",
    "/usr/local/lib",
];

// TODO: Find the correct paths for OSX
const SEARCH_OSX: &[&str] = &[
    "/usr/lib",
    "/usr/local/lib",
];

// TODO: Find the correct paths for Windows
const SEARCH_WINDOWS: &[&str] = &[
    // "C:\\Windows\\SysWOW64",    // The directory where 32-bit system files are stored on a 64-bit Windows system.
    // "C:\\Windows\\System32",    // On a 32-bit Windows system, this directory is used for 32-bit system files. On a 64-bit Windows system, this directory is used for 64-bit system files.
];


/// Returns the names of the `MaaFramework` and `MaaToolKit` shared libraries based on the target operating system.
///
/// This function checks the target operating system using the `cfg!` macro. If the target OS is Windows, it returns
/// the names of the shared libraries with the appropriate DLL suffix. For other operating systems, it returns the names
/// with the appropriate prefix and suffix.
///
/// # Returns
///
/// A tuple of two `String`s, where the first element is the name of the `MaaFramework` shared library and the second
/// element is the name of the `MaaToolKit` shared library.
fn lib_name() -> (String, String) {
    if cfg!(target_os="windows") {
        (
            format!("MaaFramework{}", env::consts::DLL_SUFFIX),
            format!("MaaToolKit{}", env::consts::DLL_SUFFIX)
        )
    } else {
        (
            format!("{}MaaFramework{}", env::consts::DLL_PREFIX, env::consts::DLL_SUFFIX),
            format!("{}MaaToolKit{}", env::consts::DLL_PREFIX, env::consts::DLL_SUFFIX)
        )
    }
}

/// Searches for the `MaaFramework` and `MaaToolKit` shared libraries in the system.
///
/// This function first checks if the `LIBMAAFW_PATH` environment variable is set. If it is, it uses the directory
/// specified by this variable as the search path. If the `LIBMAAFW_PATH` environment variable is not set, it uses
/// default search paths based on the target operating system.
///
/// # Returns
///
/// A tuple of two `Option<String>`s, where the first element is the directory of the `MaaFramework` shared library
/// (if found), and the second element is the directory of the `MaaToolKit` shared library (if found). If a shared
/// library is not found, the corresponding element in the tuple is `None`.
fn find_maafw_libs() -> (Option<String>, Option<String>) {
    let search = if let Ok(dir) = env::var("LIBMAAFW_PATH") {
        vec![dir]
    } else {
        println!("cargo:warning=LIBMAAFW_PATH environment variable not set, using default search paths");

        if cfg!(any(target_os="freebsd", target_os="linux")) {
            SEARCH_LINUX
        } else if cfg!(target_os="osx") {
            SEARCH_OSX
        } else if cfg!(target_os="windows") {
            SEARCH_WINDOWS
        } else {
            panic!("unsupported operating system!");
        }.iter().map(|s| s.to_string()).collect()
    };

    let (maafw, maatk) = lib_name();

    let maafw_dir = search.iter().find(|d| Path::new(&d).join(&maafw).exists());
    let maatk_dir = search.iter().find(|d| Path::new(&d).join(&maatk).exists());

    (
        maafw_dir.cloned(),
        maatk_dir.cloned(),
    )
}

/// Finds and links to the required libraries dynamically or statically
#[cfg(not(feature = "runtime"))]
fn main() {
    if let (Some(maafw_dir), Some(maatk_dir)) = find_maafw_libs() {
        if cfg!(feature = "static") {
            println!("cargo:rustc-link-search=native={}", maafw_dir);
            println!("cargo:rustc-link-search=native={}", maatk_dir);
            println!("cargo:rustc-link-lib=static=MaaFramework");
            println!("cargo:rustc-link-lib=static=MaaToolKit");
        } else {
            println!("cargo:rustc-link-search=native={}", maafw_dir);
            println!("cargo:rustc-link-search=native={}", maatk_dir);
            println!("cargo:rustc-link-lib=dylib=MaaFramework");
            println!("cargo:rustc-link-lib=dylib=MaaToolKit");
        }
    } else {
        panic!("MaaFramework and MaaToolKit shared libraries not found!");
    }
}

#[cfg(feature = "runtime")]
fn main() {
    if cfg!(feature = "static") {
        panic!("`runtime` and `static` features can't be combined");
    }

    unimplemented!("`runtime` feature not implemented yet!")
}