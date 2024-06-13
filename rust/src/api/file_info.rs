use crate::api::metadata::Metadata;
use anyhow::Result;
use fast_image_resize::{images::Image, IntoImageView, Resizer};
use image::{
    codecs::png::{CompressionType, FilterType, PngEncoder},
    ExtendedColorType, GenericImageView, ImageEncoder,
};
use imgref::Img;
use rgb::Rgba;
use std::{io::BufWriter, path::PathBuf};

/// Information about a file.
pub struct FileInfo {
    path: PathBuf,
    metadata: Metadata,
}

impl TryFrom<PathBuf> for FileInfo {
    type Error = anyhow::Error;

    fn try_from(path_buf: PathBuf) -> Result<Self> {
        Ok(Self {
            metadata: Metadata::try_from(path_buf.clone())?,
            path: path_buf,
        })
    }
}

impl FileInfo {
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    // Generate a sRGBa 8bit PNG preview and return the path to that file.
    pub fn generate_preview(
        &self,
        preview_larger_axis: u32,
    ) -> Result<PathBuf> {
        let image = image::open(&self.path)?;

        // Caclulate preview dimensions.
        let (width, height) = image.dimensions();
        let (resized_width, resized_height) = if width > height {
            (preview_larger_axis, (height * preview_larger_axis) / width)
        } else {
            ((width * preview_larger_axis) / height, preview_larger_axis)
        };

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

        let mut resized_image = Image::new(
            resized_width,
            resized_height,
            image.pixel_type().unwrap(),
        );
        Resizer::new().resize(&image, &mut resized_image, None)?;

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
        )
        .unwrap();

        std::fs::write("preview.png", result_buffer.get_ref())?;

        Ok(PathBuf::new())
    }
}

#[derive(Debug)]
struct Preview {
    srgb_image: Img<Vec<Rgba<u8>>>,
}
