extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::collections::HashSet;

#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

// Solution for IPPORT_RESERVED error from rust-lang/rust-bindgen#687
impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=r_anal");
    println!("cargo:rustc-link-lib=r_asm");
    println!("cargo:rustc-link-lib=r_lang");
    println!("cargo:rustc-link-lib=r_socket");
    println!("cargo:rustc-link-lib=r_crypto");
    println!("cargo:rustc-link-lib=r_parse");
    println!("cargo:rustc-link-lib=r_reg");
    println!("cargo:rustc-link-lib=r_syscall");
    println!("cargo:rustc-link-lib=r_search");
    println!("cargo:rustc-link-lib=r_cons");
    println!("cargo:rustc-link-lib=r_flag");
    println!("cargo:rustc-link-lib=r_util");

    let ignored_macros = IgnoreMacros(
            vec!["IPPORT_RESERVED".into()]
            .into_iter()
            .collect(),
        );

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg("-I/usr/include/libr")
        .parse_callbacks(Box::new(ignored_macros))
        .rustfmt_bindings(true)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
