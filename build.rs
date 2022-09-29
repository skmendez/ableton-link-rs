use cmake::Config;
use bindgen::builder;

#[cfg(target_os = "macos")]
fn link_cpp() {
    // IMPORTANT!!! otherwise linker errors, apparently only on macOS
    println!("cargo:rustc-link-lib=c++");  
}

#[cfg(target_os = "linux")]
fn link_cpp() {
    println!("cargo:rustc-link-lib=stdc++");  
}

#[cfg(target_os = "windows")]
fn link_cpp() {
    // unneeded for windows
}

fn main() {
    // cmake
    // Builds the project in the directory located in `libfoo`, installing it
    // into $OUT_DIR
    let dst = Config::new("c-wrapper")
                // .cxxflag("-fno-rtti")
                // .no_build_target(true)
                .build();
    let builddir = dst.join("lib");
    println!("cargo:rustc-link-search=native={}", builddir.display());
    println!("cargo:rustc-link-lib=static=linkrs");
    link_cpp();

    // bindgen
    let bindings = builder()
                    .header("c-wrapper/link_rs.h")
                    .whitelist_function("Link_.*")
                    .whitelist_function("SessionState_.*")
                    .whitelist_function("Clock_.*")
                    .generate()
                    .expect("generate bindings");
    let outfile = dst.join("link_rs.rs");
    bindings.write_to_file(outfile).expect("write bindings to file");

}