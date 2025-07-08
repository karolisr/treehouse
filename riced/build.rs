use convert_case::{Case, Casing};
use std::env::var_os;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = var_os("OUT_DIR").unwrap();
    let output_path = Path::new(&out_dir).join("icons.rs");
    let svg_icon_files = fs::read_dir(Path::new("icons"));

    let prefix = "MaterialSymbolsLight";
    let suffix = "Rounded";

    let mut lines: Vec<String> = Vec::new();
    if let Ok(files) = svg_icon_files {
        for file in files.flatten() {
            if file.path().is_file()
                && let Ok(bytes) = fs::read(file.path())
                && let Some(name) = file.path().file_stem()
                && let Some(name) = name.to_str()
                && let Some(ext) = file.path().extension()
            {
                if ext != "svg" || !name.starts_with(prefix) {
                    continue;
                }

                let length = bytes.len();
                let name = name
                    .replace(prefix, "")
                    .replace(suffix, "")
                    .to_case(Case::Constant);
                let line = format!(
                    "pub(crate) const {name}: [u8; {length}] = {bytes:?};\n"
                );

                lines.push(line);
            }
        }

        lines.sort();

        let _ = fs::write(&output_path, lines.concat());
    }

    println!("cargo::rerun-if-changed=build.rs");
}
