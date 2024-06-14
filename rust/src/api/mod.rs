pub mod file_info;
pub mod init;
pub mod metadata;

#[cfg(test)]
mod tests {
    use crate::api::file_info::FileInfo;
    use std::path::PathBuf;
    use anyhow::Result;

    #[test]
    fn it_works() -> Result<()> {

        let jpg =
            FileInfo::try_from(PathBuf::from("assets/gamma_dalai_lama_gray_good.jpg"))?;
        println!("JPG\n{:#?}", jpg.metadata());

        let exr = FileInfo::try_from(PathBuf::from("assets/j0.3toD.exr"))?;
        println!("EXR\n{:#?}", exr.metadata());
        Ok(())
    }
}
