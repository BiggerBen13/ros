use cc::Build;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    unsafe { env::set_var("CC", "/opt/homebrew/opt/llvm/bin/clang") };

    let mut asm_files = Vec::<String>::new();
    let asm_dir = fs::read_dir("asm").unwrap();
    for entry in asm_dir {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        let file_path = Path::new(&file_name);
        let Some(extension) = file_path.extension() else {
            continue;
        };
        if extension == "s" {
            let mut file = "asm/".to_string();
            file.push_str(file_path.to_str().unwrap());
            asm_files.push(file);
        }
    }

    Build::new().files(asm_files).compile("asm");
}
