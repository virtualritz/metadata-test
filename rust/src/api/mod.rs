pub mod file_info;
pub mod init;
pub mod metadata;

#[cfg(test)]
mod tests {
    use crate::api::file_info::FileInfo;
    use anyhow::Result;
    use std::path::PathBuf;

    #[test]
    fn metadata() -> Result<()> {
        let jpg = FileInfo::try_from(PathBuf::from(
            "assets/gamma_dalai_lama_gray_good.jpg",
        ))?;
        println!("JPG\n{:#?}", jpg.metadata());

        let exr = FileInfo::try_from(PathBuf::from("assets/j0.3toD.exr"))?;
        println!("EXR\n{:#?}", exr.metadata());

        Ok(())
    }

    #[test]
    fn preview() -> Result<()> {
        /*let jpg = FileInfo::try_from(PathBuf::from(
            "assets/gamma_dalai_lama_gray_good.jpg",
        ))?;
        println!("Preview written to {:?}", jpg.generate_preview(256)?);*/

        let png = FileInfo::try_from(PathBuf::from("assets/j0.3toD.png"))?;
        println!("Preview written to {:?}", png.generate_preview(256)?);

        //let exr = FileInfo::try_from(PathBuf::from("assets/j0.3toD.exr"))?;
        //println!("Preview written to {:?}", exr.generate_preview(256)?);

        Ok(())
    }
}
