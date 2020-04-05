# nanojpeg-rs

Rust `no_std` port of [nanojpeg](https://keyj.emphy.de/nanojpeg/).

Mostly for use in my [rpi4 projects](https://github.com/jonlamb-gh/rpi4-rust-workspace).

`nanojpeg` modifications I made:

* `NJ_USE_32BPP` : Use 32 bits per pixel instead of 24
* `NJ_USE_BGR` : Use BGR pixel order instead of RGB
