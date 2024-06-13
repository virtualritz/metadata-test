use anyhow::Result;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use file_format::{FileFormat, Kind};
use filetime::FileTime;
use image::GenericImageView;
use num_rational::Ratio;
use std::path::{Path, PathBuf};

#[derive(Default, Debug)]
pub struct Metadata {
    format: FileFormat,
    title: Option<String>,
    author: Option<String>,
    accessed: Option<DateTime<Utc>>,
    modified: Option<DateTime<Utc>>,
    specific_metadata: Option<SpecificMetadata>,
}

#[derive(Debug)]
pub enum SpecificMetadata {
    Archive,
    Audio,
    Compressed,
    Database,
    Diagram,
    Disk,
    Document,
    Ebook,
    Executable,
    Font,
    Formula,
    Geospatial,
    Image {
        width: u32,
        height: u32,
        pixel_aspect: Ratio<u32>,
    },
    Metadata,
    Model,
    Other,
    Package,
    Playlist,
    Presentation,
    Rom,
    Spreadsheet,
    Subtitle,
    Video {
        width: u32,
        height: u32,
        pixel_aspect: Ratio<u32>,
        number_of_frames: usize,
        frames_per_second: BigDecimal,
    },
}

impl TryFrom<PathBuf> for Metadata {
    type Error = anyhow::Error;

    fn try_from(path: PathBuf) -> Result<Self> {
        Self::extract_metadata(&path)
    }
}

impl Metadata {
    /// Extract metadata from a file
    fn extract_metadata(file: &Path) -> Result<Metadata> {
        let format = FileFormat::from_file(file)?;

        let metadata = std::fs::metadata(file)?;

        let ft = FileTime::from_last_access_time(&metadata);
        let accessed = DateTime::from_timestamp(ft.seconds(), ft.nanoseconds());

        let ft = FileTime::from_last_modification_time(&metadata);
        let modified = DateTime::from_timestamp(ft.seconds(), ft.nanoseconds());

        let author = None;
        let title = None;

        let specific_metadata = match format.kind() {
            Kind::Image => {
                // Fastest way to get image size via the `imagesize`
                // crate -- it only reads 12 bytes.
                let (width, height) = match format {
                    // The following formats are supported by `imagesize` but
                    // not by `file_format`:
                    // * TGA      -> this one is important -- we should fix
                    //               this/make a PR for `file_format`.
                    // * Aseprite -> unimportant.
                    // * VTF      -> unimportant.
                    FileFormat::AdobePhotoshopDocument
                    | FileFormat::Farbfeld
                    | FileFormat::GraphicsInterchangeFormat
                    | FileFormat::HighEfficiencyImageCoding
                    | FileFormat::HighEfficiencyImageCodingSequence
                    | FileFormat::HighEfficiencyImageFileFormat
                    | FileFormat::HighEfficiencyImageFileFormatSequence
                    | FileFormat::JointPhotographicExpertsGroup
                    | FileFormat::JpegXl
                    | FileFormat::KhronosTexture2
                    | FileFormat::MicrosoftDirectdrawSurface
                    | FileFormat::Openexr
                    | FileFormat::PortableBitmap
                    | FileFormat::PortableGraymap
                    | FileFormat::PortableNetworkGraphics
                    | FileFormat::PortablePixmap
                    | FileFormat::QuiteOkImage
                    | FileFormat::RadianceHdr
                    | FileFormat::TagImageFileFormat
                    | FileFormat::Webp
                    | FileFormat::WindowsBitmap
                    | FileFormat::WindowsIcon => {
                        let dimensions = imagesize::size(file)?;
                        (dimensions.width as _, dimensions.height as _)
                    }
                    // For everything else we use the `image` crate.
                    // Potentially much slower/expensive.
                    _ => {
                        let image = image::open(file)?;
                        image.dimensions()
                    }
                };

                Some(SpecificMetadata::Image {
                    width,
                    height,
                    // For now we leave this as square.
                    pixel_aspect: Ratio::new(1, 1),
                })
            }
            _ => None,
        };

        Ok(Metadata {
            format,
            title,
            author,
            accessed,
            modified,
            specific_metadata,
        })
    }
}
