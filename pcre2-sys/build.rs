use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=include/pcre2.h");
    println!("cargo:rerun-if-changed=include/wrapper.h");
    println!("cargo:rerun-if-changed=src/bindings.rs");
    println!(
        "cargo:rustc-link-search=native={}/lib",
        env::current_dir()?.display()
    );
    println!("cargo:rustc-link-lib=static=pcre2-8");

    let bindings = bindgen::Builder::default()
        .header("include/wrapper.h")
        .derive_debug(true)
        .derive_eq(true)
        .ctypes_prefix("::libc")
        .allowlist_function("^pcre2_.*")
        .allowlist_type("^pcre2_.*")
        .allowlist_var("^PCRE2_.*")
        .blocklist_function("^.*_callout_.*")
        .blocklist_type("^.*_callout_.*")
        .clang_arg("-DPCRE2_CODE_UNIT_WIDTH=8")
        .generate()?;

    bindings.write_to_file("src/bindings.rs")?;

    Ok(())
}
