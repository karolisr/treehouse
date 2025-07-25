#[cfg(not(target_os = "windows"))]
fn main() {
    // Do nothing on non-Windows platforms
}

#[cfg(target_os = "windows")]
fn main() {
    // Convert PNG files to high resolution ICO files
    convert_png_to_ico();

    // Set up Windows resources to embed the icons
    let mut res = winres::WindowsResource::new();

    let _ = res
        .set_icon("../resources/icons/treehouse.ico")
        .set_icon_with_id("../resources/icons/newick.ico", "2")
        .set_icon_with_id("../resources/icons/nexus.ico", "3");

    let _ = res.set_manifest(
        r#"
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
<trustInfo xmlns="urn:schemas-microsoft-com:trustinfo">
    <security>
        <requestedPrivileges>
            <requestedExecutionLevel level="asInvoker" uiAccess="false" />
        </requestedPrivileges>
    </security>
</trustInfo>
</assembly>
"#,
    );

    res.compile().unwrap();

    println!("cargo:rerun-if-changed=../resources/icons/treehouse.ico");
    println!("cargo:rerun-if-changed=../resources/icons/newick.ico");
    println!("cargo:rerun-if-changed=../resources/icons/nexus.ico");
}

#[cfg(target_os = "windows")]
fn convert_png_to_ico() {
    use ico::{IconDir, IconDirEntry, IconImage, ResourceType};
    use image::{DynamicImage, ImageFormat, imageops};
    use std::fs::File;
    use std::io::{BufWriter, Cursor};
    use std::path::Path;

    let icons_dir = Path::new("../resources/icons");

    // Define the PNG files to convert and their target ICO names
    let conversions = [
        ("treehouse.png", "treehouse.ico"),
        ("newick.png", "newick.ico"),
        ("nexus.png", "nexus.ico"),
    ];

    for (png_name, ico_name) in conversions.iter() {
        let png_path = icons_dir.join(png_name);
        let ico_path = icons_dir.join(ico_name);

        // Skip if ICO file already exists and is newer than PNG
        if ico_path.exists() && png_path.exists() {
            if let (Ok(ico_meta), Ok(png_meta)) =
                (ico_path.metadata(), png_path.metadata())
            {
                if let (Ok(ico_time), Ok(png_time)) =
                    (ico_meta.modified(), png_meta.modified())
                {
                    if ico_time >= png_time {
                        println!(
                            "cargo:rerun-if-changed={}",
                            png_path.display()
                        );
                        continue;
                    }
                }
            }
        }

        if png_path.exists() {
            println!("Converting {} to {}", png_name, ico_name);

            match convert_png_to_ico_file(&png_path, &ico_path) {
                Ok(_) => println!(
                    "Successfully converted {} to {}",
                    png_name, ico_name
                ),
                Err(e) => eprintln!(
                    "Failed to convert {} to {}: {}",
                    png_name, ico_name, e
                ),
            }
        }

        println!("cargo:rerun-if-changed={}", png_path.display());
    }

    fn convert_png_to_ico_file(
        png_path: &Path,
        ico_path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Load the PNG image
        let img = image::open(png_path)?;

        // Create an ICO directory
        let mut icon_dir = IconDir::new(ResourceType::Icon);

        // Generate multiple resolutions for high-quality ICO
        let sizes = [16, 24, 32, 48, 64, 128, 256, 512];

        for &size in &sizes {
            // Resize the image to the target size
            let resized =
                img.resize_exact(size, size, imageops::FilterType::Lanczos3);

            // Convert to RGBA8 format
            let rgba_img = resized.to_rgba8();
            let (width, height) = rgba_img.dimensions();

            // Create PNG data for this size
            let mut png_data = Vec::new();
            {
                let mut cursor = Cursor::new(&mut png_data);
                DynamicImage::ImageRgba8(rgba_img.clone())
                    .write_to(&mut cursor, ImageFormat::Png)?;
            }

            // Create an icon image entry
            let icon_image =
                IconImage::from_rgba_data(width, height, rgba_img.into_raw());

            // Add to the icon directory
            icon_dir.add_entry(IconDirEntry::encode(&icon_image)?);
        }

        // Write the ICO file
        let ico_file = File::create(ico_path)?;
        let mut writer = BufWriter::new(ico_file);
        icon_dir.write(&mut writer)?;

        Ok(())
    }
}
