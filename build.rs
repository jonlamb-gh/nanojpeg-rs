use std::env;

fn main() {
    env::set_var("CC_aarch64_unknown_none", "aarch64-linux-gnu-gcc");

    cc::Build::new()
        .flag("-fno-stack-protector")
        .warnings(false)
        .file("nanojpeg/nanojpeg.c")
        .compile("nanojpeg");

    println!("cargo:rerun-if-env-changed=CC");
}
