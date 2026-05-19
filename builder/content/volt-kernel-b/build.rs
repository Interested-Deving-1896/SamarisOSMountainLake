fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-cfg=linux");

    if std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "linux" {
        println!("cargo:rustc-cfg=target_linux");
    }
}
