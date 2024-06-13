pub mod file_info;
pub mod init;
pub mod metadata;

#[cfg(test)]
mod tests {
    use crate::api::FileInfo;
    use anyhow::Result;
    use std::path::Path;

    #[test]
    fn it_works() -> Result<()> {
        let jpg =
            File::new(Path::new("assets/gamma_dalai_lama_gray_good.jpg"))?;
        println!("JPG\n{:?}", jpg.metadata());

        let exr = File::new(Path::new("assets/j0.3toD.exr"))?;
        println!("EXR\n{:?}", exr.metadata());

        Ok(())
    }
}
