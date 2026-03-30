use convert_case::Case;
use convert_case::Casing;
use std::env::var_os;
use std::error::Error;
use std::ffi::OsString;
use std::fs::read;
use std::fs::read_dir;
use std::fs::write;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo::rerun-if-changed=build.rs");
    let data_dir = Path::new("..").join("resources").join("data");
    let code_out_dir = var_os("OUT_DIR").unwrap();
    build_icons(&code_out_dir)?;
    build_fonts(&data_dir, &code_out_dir)?;
    Ok(())
}

fn build_fonts(
    data_dir: &Path,
    code_out_dir: &OsString,
) -> Result<(), Box<dyn Error>> {
    let font_files = read_dir(data_dir.join("fonts"));
    let fonts_rs_file = Path::new(code_out_dir).join("fonts.rs");

    let mut lines: Vec<String> = Vec::new();
    if let Ok(files) = font_files {
        for file in files.flatten() {
            if file.path().is_file()
                && let Ok(bytes) = read(file.path())
                && let Some(name) = file.path().file_stem()
                && let Some(name) = name.to_str()
                && let Some(ext) = file.path().extension()
            {
                if ext != "ttf" {
                    continue;
                }

                let length = bytes.len();
                let const_name = name.replace('-', "_").to_case(Case::Constant);
                let line = format!(
                    "pub static {const_name}: [u8; {length}] = {bytes:?};\n"
                );

                lines.push(line);
            }
        }

        lines.sort();

        write(&fonts_rs_file, lines.concat())?;
    }

    Ok(())
}

fn build_icons(code_out_dir: &OsString) -> Result<(), Box<dyn Error>> {
    let svg_icon_files = read_dir(Path::new("icons"));
    let output_path = Path::new(&code_out_dir).join("icons.rs");

    let prefix = "MaterialSymbolsLight";
    let suffix = "Rounded";

    let mut lines: Vec<String> = Vec::new();
    if let Ok(files) = svg_icon_files {
        for file in files.flatten() {
            if file.path().is_file()
                && let Ok(bytes) = read(file.path())
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
                    "pub(crate) static {name}: [u8; {length}] = {bytes:?};\n"
                );

                lines.push(line);
            }
        }

        lines.sort();

        write(&output_path, lines.concat())?;
    }

    Ok(())
}
