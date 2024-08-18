fn main() {
    cc::Build::new()
        .include("include")
        .file("ffi_stub/ffi_stub.c")
        .opt_level(3)
        .debug(false)
        .warnings(true)
        .compile("ffi_stub");

    println!("cargo::rerun-if-changed=ffi_stub.c");
}
