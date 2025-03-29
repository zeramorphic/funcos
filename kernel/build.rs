fn main() {
    println!("cargo:rustc-link-arg=-Tkernel/link-script.ld");
}
