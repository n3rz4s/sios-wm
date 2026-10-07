use std::{
    fs,
    path::PathBuf,
};

use xcursor::parser::parse_xcursor;

use super::CursorImage;


pub struct CursorTheme {
    pub name: String,
    pub path: PathBuf,
}

impl CursorTheme {
    pub fn new(
        name: impl Into<String>
    ) -> Self {
        let name = name.into();

        let path = PathBuf::from(
            "/usr/local/share/sios-wm-cursors",
        )
        .join(&name)
        .join("cursors");

        Self {
            name,
            path,
        }
    }

    pub fn load(
        &self,
        icon: &str,
        size: u32,
    ) -> Result<CursorImage, Box<dyn std::error::Error>> {
        let path = self.path.join(icon);

        eprintln!(
            "SIOS: Loading cursor file: {}",
            path.display(),
        );

        let data = fs::read(&path)?;

        let images = parse_xcursor(&data)
            .ok_or_else(
                || format!(
                    "SIOS: Failed to parse xcursor: {}",
                    path.display(),
                )
            )?;

        if images.is_empty() {
            return Err(format!(
                "SIOS: XCursor contains no images: {}",
                path.display()
            ).into());
        }

        let image = images
            .iter()
            .filter(|image| image.size >= size)
            .min_by_key(|image| image.size)
            .or_else(|| images.iter().max_by_key(|image| image.size))
            .ok_or("SIOS: Failed to select XCursor image.")?;

        Ok(CursorImage {
            width: image.width,
            height: image.height,
            hotspot_x: image.xhot,
            hotspot_y: image.yhot,

            pixels: image.pixels_rgba.clone(),
        })
    }
}