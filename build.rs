use std::env;

fn main() {
    env::set_var("CC_aarch64_unknown_none", "aarch64-linux-gnu-gcc");

    let mut build = cc::Build::new();

    build.flag("-fno-stack-protector");
    build.warnings(false);
    build.file("nanojpeg/nanojpeg.c");

    #[cfg(not(feature = "libc"))]
    {
        build.define("NJ_USE_LIBC", "0");
        build.define("NULL", "0");
    }

    #[cfg(feature = "pbp32")]
    {
        build.define("NJ_USE_32BPP", "1");
    }

    #[cfg(feature = "bgr")]
    {
        build.define("NJ_USE_BGR", "1");
    }

    build.compile("nanojpeg");

    println!("cargo:rerun-if-env-changed=CC");
}
