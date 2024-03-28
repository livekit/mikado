// Generate CoreAudio bindings
//
//
// Code based on https://github.com/RustAudio/coreaudio-sys
// Under the license of MIT:
// Copyright (c) 2015

// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:

// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use std::env;
use std::path::PathBuf;

pub fn build() {
    let target = std::env::var("TARGET").unwrap();
    if !(target.contains("apple-darwin") || target.contains("apple-ios")) {
        panic!("coreaudio-sys requires macos or ios target");
    }

    let sdk_path = sdk_path(&target).ok();
    let sdk_path = sdk_path.as_ref().map(String::as_ref);
    let mut headers: Vec<&'static str> = vec![];

    println!("cargo:rustc-link-lib=framework=CoreAudio");

    if target.contains("apple-ios") {
        headers.push("CoreAudio/CoreAudioTypes.h");
    } else {
        headers.push("CoreAudio/CoreAudio.h");
    }

    println!("cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS");
    // Get the cargo out directory.
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("env variable OUT_DIR not found"));

    // Begin building the bindgen params.
    let mut builder = bindgen::Builder::default();

    // See https://github.com/rust-lang/rust-bindgen/issues/1211
    // Technically according to the llvm mailing list, the argument to clang here should be
    // -arch arm64 but it looks cleaner to just change the target.
    let target = if target == "aarch64-apple-ios" {
        "arm64-apple-ios"
    } else if target == "aarch64-apple-darwin" {
        "arm64-apple-darwin"
    } else {
        &target
    };
    builder = builder.size_t_is_usize(true);
    builder = builder.clang_args(&[&format!("--target={}", target)]);

    if let Some(sdk_path) = sdk_path {
        builder = builder.clang_args(&["-isysroot", sdk_path]);
    }
    if target.contains("apple-ios") {
        // time.h as has a variable called timezone that conflicts with some of the objective-c
        // calls from NSCalendar.h in the Foundation framework. This removes that one variable.
        builder = builder.blocklist_item("timezone");
        builder = builder.blocklist_item("objc_object");
    }

    // bindgen produces alignment tests that cause undefined behavior in some cases.
    // This seems to happen across all apple target tripples :/.
    // https://github.com/rust-lang/rust-bindgen/issues/1651
    builder = builder.layout_tests(false);

    let meta_header: Vec<_> = headers
        .iter()
        .map(|h| format!("#include <{}>\n", h))
        .collect();

    builder = builder.header_contents("coreaudio.h", &meta_header.concat());

    // Generate the bindings.
    builder = builder.trust_clang_mangling(false).derive_default(true);

    let bindings = builder.generate().expect("unable to generate bindings");

    // Write them to the crate root.
    bindings
        .write_to_file(out_dir.join("coreaudio.rs"))
        .expect("could not write bindings");
}

fn sdk_path(target: &str) -> Result<String, std::io::Error> {
    // Use environment variable if set
    println!("cargo:rerun-if-env-changed=COREAUDIO_SDK_PATH");
    if let Ok(path) = std::env::var("COREAUDIO_SDK_PATH") {
        return Ok(path);
    }

    use std::process::Command;

    let sdk = if target.contains("apple-darwin") {
        "macosx"
    } else if target == "x86_64-apple-ios"
        || target == "i386-apple-ios"
        || target == "aarch64-apple-ios-sim"
    {
        "iphonesimulator"
    } else if target == "aarch64-apple-ios"
        || target == "armv7-apple-ios"
        || target == "armv7s-apple-ios"
    {
        "iphoneos"
    } else {
        unreachable!();
    };
    let output = Command::new("xcrun")
        .args(&["--sdk", sdk, "--show-sdk-path"])
        .output()?
        .stdout;

    let prefix_str = std::str::from_utf8(&output).expect("invalid output from `xcrun`");
    Ok(prefix_str.trim_end().to_string())
}
