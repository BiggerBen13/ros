use cc::Build;
use std::env;

fn main() {
    unsafe { env::set_var("CC", "/opt/homebrew/opt/llvm/bin/clang") };
    Build::new().file("asm/boot.s").compile("asm");
}
