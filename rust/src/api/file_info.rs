use crate::api::metadata::Metadata;
use anyhow::{anyhow, Result};
use fast_image_resize::{images::Image, IntoImageView, Resizer, PixelType};
use image::{
    codecs::png::{CompressionType, FilterType, PngEncoder},
    ExtendedColorType, GenericImageView, ImageEncoder,
};
use imgref::Img;
use rgb::Rgba;
use std::{
    io::BufWriter,
    path::{Path, PathBuf},
};
use flutter_rust_bridge::frb;

/// Information about a file.
pub struct FileInfo {
    path: PathBuf,
    metadata: Metadata,
}

/*
impl TryFrom<PathBuf> for FileInfo {
    type Error = anyhow::Error;

    #[frb(sync)]
    fn try_from(path_buf: PathBuf) -> Result<Self> {
        Ok(Self {
            metadata: Metadata::try_from(path_buf.clone())?,
            path: path_buf,
        })
    }
}
*/

impl TryFrom<&str> for FileInfo {
    type Error = anyhow::Error;

    #[frb(sync)]
    fn try_from(string: &str) -> Result<Self> {
        let path_buf = PathBuf::from(string);
        Ok(Self {
            metadata: Metadata::try_from(path_buf.clone())?,
            path: path_buf,
        })
    }
}


impl FileInfo {
    #[frb(sync)]
    pub fn new(file_name: &str) -> Result<Self> {
        Self::try_from(file_name)
    }

    #[frb(sync)]
    pub fn metadata(&self) -> Metadata {
        self.metadata.clone()
    }

    // Generate a sRGBa 8bit PNG preview and return the path to that file.
    #[frb(sync)]
    pub fn generate_preview(
        &self,
        preview_larger_axis: u32,
    ) -> Result<PathBuf> {
        let image = image::open(&self.path)?;

        let (width, height) = image.dimensions();

        // Caclulate preview dimensions.
        let (resized_width, resized_height) = if width > height {
            // Landscape.
            (preview_larger_axis, (height * preview_larger_axis) / width)
        } else {
            // Portrait or square image.
            ((width * preview_larger_axis) / height, preview_larger_axis)
        };

        // If the image is already small, just convert to PNG

        let preview_file_name = if width < resized_width {
            let mut result_buffer = BufWriter::new(Vec::new());
            PngEncoder::new_with_quality(
                &mut result_buffer,
                CompressionType::Best,
                FilterType::Adaptive,
            )
            .write_image(
                image.as_bytes(),
                width,
                height,
                ExtendedColorType::Rgb8,
            )?;

            let preview_file_name = generate_preview_file_name(&self.path)?;

            std::fs::write(&preview_file_name, result_buffer.get_ref())?;

            preview_file_name
        } else {
            /*match dynamic_image {
                DynamicImage::ImageRgb8(image) => {
                    let image = image
                        .into_raw()
                        .par_chunks_exact(3)
                        .map(|rgb| println!("{:?}", (rgb[0], rgb[1], rgb[2])));
                }
                _ => println!("Not implemented yet"),
            }*/

            // FIXME: This most likely resizes in non-linear color space as
            // most input data will be in gamma-corrected (non-linear) sRGB.
            // Results will be wrong but good enough for now.
            // Once we have users that work professionally with media, this
            // needs to be fixed so resizing happens in linear color space.
            // See http://www.ericbrasseur.org/gamma.html

            //let resized_image = if let Some(pixel_type) = image.pixel_type() {
                let mut resized_image =
                    Image::new(resized_width, resized_height, PixelType::U8x3);
                Resizer::new().resize(&image, &mut resized_image, None)?;

                //resized_image
            /* } else {
                // TODO: Support other pixel types, e.g. with `f16`/`f32`-based channels.
                return Err(anyhow!("Unsupported pixel type"));
            };*/

            // Write destination image as PNG-file
            let mut result_buffer = BufWriter::new(Vec::new());
            PngEncoder::new_with_quality(
                &mut result_buffer,
                CompressionType::Best,
                FilterType::Adaptive,
            )
            .write_image(
                resized_image.buffer(),
                resized_width,
                resized_height,
                ExtendedColorType::Rgb8,
            )?;

            let preview_file_name = generate_preview_file_name(&self.path)?;

            std::fs::write(&preview_file_name, result_buffer.get_ref())?;

            preview_file_name
        };

        Ok(preview_file_name)
    }
}

fn generate_preview_file_name(path: &Path) -> Result<PathBuf> {
    Ok(PathBuf::from(
        path.file_prefix()
            .ok_or(anyhow!("No file prefix"))?
            .to_str()
            .ok_or(anyhow!("File is not valid UTF8"))?
            .to_owned()
            + "-preview.png",
    ))
}

#[derive(Debug)]
struct Preview {
    srgb_image: Img<Vec<Rgba<u8>>>,
}
