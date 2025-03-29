fn main() {
    println!("cargo:rustc-env=env-emboss-var=4");
    println!("cargo:rustc-env=many-env-emboss-var-1=5");
    println!("cargo:rustc-env=many-env-emboss-var-3=6");
}
