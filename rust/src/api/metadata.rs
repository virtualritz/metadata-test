use anyhow::Result;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use file_format::{FileFormat, Kind};
use filetime::FileTime;
use image::GenericImageView;
use num_rational::Ratio;
use std::path::{Path, PathBuf};
use flutter_rust_bridge::frb;

#[derive(Default, Debug, Clone)]
pub struct Metadata {
    format: FileFormat,
    accessed: DateTime<Utc>,
    modified: DateTime<Utc>,
    title: Option<String>,
    author: Option<String>,
    specific_metadata: Option<SpecificMetadata>,
}

#[derive(Clone, Debug)]
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
        pixel_aspect: f32, //Ratio<u32>,
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
        pixel_aspect: f32, //Ratio<u32>,
        number_of_frames: usize,
        frames_per_second: f32, //BigDecimal,
    },
}

impl TryFrom<PathBuf> for Metadata {
    type Error = anyhow::Error;

    fn try_from(path: PathBuf) -> Result<Self> {
        Self::extract_metadata(&path)
    }
}

impl Metadata {
    #[frb(sync)]
    pub fn format(&self) -> FileFormat {
        self.format.clone()
    }

    #[frb(sync)]
    pub fn accessed(&self) -> DateTime<Utc> {
        self.accessed.clone()
    }

    #[frb(sync)]
    pub fn modified(&self) -> DateTime<Utc> {
        self.modified.clone()
    }

    #[frb(sync)]
    pub fn title(&self) -> Option<String> {
        self.title.clone()
    }

    #[frb(sync)]
    pub fn author(&self) -> Option<String> {
        self.author.clone()
    }

    #[frb(sync)]
    pub fn specific(&self) -> Option<SpecificMetadata> {
        self.specific_metadata.clone()
    }

    /// Extract metadata from a file
    #[frb(sync)]
    fn extract_metadata(file: &Path) -> Result<Metadata> {
        let format = FileFormat::from_file(file)?;

        let metadata = std::fs::metadata(file)?;

        let ft = FileTime::from_last_access_time(&metadata);
        let accessed = DateTime::from_timestamp(ft.seconds(), ft.nanoseconds())
            .unwrap_or(Utc::now());

        let ft = FileTime::from_last_modification_time(&metadata);
        let modified = DateTime::from_timestamp(ft.seconds(), ft.nanoseconds())
            .unwrap_or(Utc::now());

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
                    pixel_aspect: 1.0, //Ratio::new(1, 1),
                })
            }
            /*Kind::Document => {
                if OfficeOpenXmlDocument format {


                    Some(SpecificMetadata::Document) {

                    }
                }
                else {
                    None
                }
            }*/
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
