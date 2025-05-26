use super::errors::FileErrors;

#[derive(Debug)]
pub struct ROMFile<T> {
    rom: T,
}

pub trait ROMFs {
    fn new(rom_path: String) -> Result<Self, FileErrors>
    where
        Self: Sized;
    fn validate_file(rom_path: &str) -> Result<(), FileErrors>;
    fn read_file(rom_path: &str) -> Result<Vec<u8>, FileErrors>;
    fn read_exact_at(&self, offset: usize, size: usize) -> Result<&[u8], FileErrors>;
    fn path(&self) -> &str;
    fn size(&self) -> usize;
}

impl<T: ROMFs> ROMFile<T> {
    pub fn new(rom_path: String) -> Result<Self, FileErrors> {
        let rom = T::new(rom_path)?;

        Ok(ROMFile { rom })
    }

    pub fn path(&self) -> &str {
        self.rom.path()
    }

    pub fn size(&self) -> usize {
        self.rom.size()
    }

    pub fn read_rom_header(&self) -> Result<&[u8], FileErrors> {
        Ok(&self.rom.read_exact_at(0, 16)?)
    }

    pub fn read_rom_content(&self) -> Result<&[u8], FileErrors> {
        Ok(&self.rom.read_exact_at(0, self.size())?)
    }

    pub fn read_exact_at(&self, offset: usize, size: usize) -> Result<&[u8], FileErrors> {
        Ok(&self.rom.read_exact_at(offset, size)?)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    use super::*;
    use crate::sys::{errors::FileErrors, rom_file::ROM};

    fn setup_test_files(file_path: Option<&str>, content: Option<&[u8]>) -> TempDir {
        let dir = TempDir::new().unwrap();
        let dir_path = dir.path();

        let file_path = file_path.unwrap_or("valid.nes");
        let content = content.unwrap_or(&[
            b'N', b'E', b'S', 0x1A, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]);

        let valid_path = dir_path.join(file_path);
        let mut valid_file = File::create(valid_path).unwrap();

        valid_file.write_all(content).unwrap();

        dir
    }

    #[test]
    fn test_romfile_new() {
        let dir = setup_test_files(None, None);
        let valid_path = dir.path().join("valid.nes").to_str().unwrap().to_string();

        let rom_file: ROMFile<ROM> = ROMFile::new(valid_path).unwrap();

        assert_eq!(rom_file.rom.size(), 20);
    }

    #[test]
    fn test_read_invalid_rom_size() {
        let dir = setup_test_files(
            Some("invalid_size.nes"),
            Some(&[b'N', b'E', b'S', 0x1A, 2, 1, 1]),
        );
        let invalid_size_path = dir
            .path()
            .join("invalid_size.nes")
            .to_str()
            .unwrap()
            .to_string();
        let rom_file: Result<ROMFile<ROM>, FileErrors> = ROMFile::new(invalid_size_path);

        assert!(rom_file.is_err());
        assert_eq!(rom_file.unwrap_err(), FileErrors::ErrorInvalidFileSize);
    }

    #[test]
    fn test_read_invalid_rom_header() {
        let dir = setup_test_files(
            Some("invalid_header.nes"),
            Some(&[
                78, 78, 78, 78, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]),
        );
        let invalid_header_path = dir
            .path()
            .join("invalid_header.nes")
            .to_str()
            .unwrap()
            .to_string();
        let rom_file: Result<ROMFile<ROM>, FileErrors> = ROMFile::new(invalid_header_path);

        assert!(rom_file.is_err());
        assert_eq!(rom_file.unwrap_err(), FileErrors::ErrorInvalidROMFile);
    }

    #[test]
    fn test_read_invalid_rom_extension() {
        let dir = setup_test_files(
            Some("invalid_extension.xxx"),
            Some(&[
                b'N', b'E', b'S', 0x1A, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]),
        );
        let invalid_extension_path = dir
            .path()
            .join("invalid_extension.xxx")
            .to_str()
            .unwrap()
            .to_string();
        let rom_file: Result<ROMFile<ROM>, FileErrors> = ROMFile::new(invalid_extension_path);

        assert!(rom_file.is_err());
        assert_eq!(rom_file.unwrap_err(), FileErrors::ErrorInvalidExtension);
    }

    #[test]
    fn test_read_non_existent_rom_file() {
        let non_existent_path = String::from("unexisting_file.nes");
        let rom_file: Result<ROMFile<ROM>, FileErrors> = ROMFile::new(non_existent_path);

        assert!(rom_file.is_err());
        assert_eq!(rom_file.unwrap_err(), FileErrors::ErrorInvalidROMFile);
    }

    #[test]
    fn test_read_rom_header() {
        let dir = setup_test_files(None, None);
        let valid_path = dir.path().join("valid.nes").to_str().unwrap().to_string();

        let rom_file: ROMFile<ROM> = ROMFile::new(valid_path).unwrap();

        let valid_header: [u8; 16] = [b'N', b'E', b'S', 0x1A, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(rom_file.read_rom_header().unwrap(), valid_header);
    }

    #[test]
    fn test_read_rom_content() {
        let dir = setup_test_files(None, None);
        let valid_path = dir.path().join("valid.nes").to_str().unwrap().to_string();

        let rom_file: ROMFile<ROM> = ROMFile::new(valid_path).unwrap();

        let content = rom_file.read_rom_content().unwrap();

        assert_eq!(content.len(), 20);
        assert_eq!(&content[0..4], &[b'N', b'E', b'S', 0x1A]);
        assert_eq!(
            &content,
            &[
                b'N', b'E', b'S', 0x1A, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
    }

    #[test]
    fn test_read_exact_at() {
        let dir = setup_test_files(None, None);
        let valid_path = dir.path().join("valid.nes").to_str().unwrap().to_string();

        let rom_file: ROMFile<ROM> = ROMFile::new(valid_path).unwrap();

        assert_eq!(rom_file.read_exact_at(0, 0).unwrap(), &[]);

        assert_eq!(rom_file.read_exact_at(0, 20).unwrap().len(), 20);

        assert_eq!(
            rom_file.read_exact_at(0, 4).unwrap(),
            &[b'N', b'E', b'S', 0x1A]
        );

        assert_eq!(
            rom_file.read_exact_at(0, 16).unwrap(),
            &[b'N', b'E', b'S', 0x1A, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );

        assert_eq!(
            rom_file.read_exact_at(20, 1).unwrap_err(),
            FileErrors::ErrorInvalidRange
        );
    }

    #[test]
    fn test_access_path() {
        let dir = setup_test_files(None, None);
        let valid_path = dir.path().join("valid.nes").to_str().unwrap().to_string();

        let rom_file: ROMFile<ROM> = ROMFile::new(valid_path.clone()).unwrap();

        assert_eq!(rom_file.path(), &valid_path);
    }

    #[test]
    fn test_access_size() {
        let dir = setup_test_files(None, None);
        let valid_path = dir.path().join("valid.nes").to_str().unwrap().to_string();

        let rom_file: ROMFile<ROM> = ROMFile::new(valid_path).unwrap();

        assert_eq!(rom_file.size(), 20);
    }
}
