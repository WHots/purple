use std::fs;
use std::path::{Path, PathBuf};
use std::io;

use crate::genericutils::generate_random_string;


pub struct FileUtils {
    file_path: String,
}

impl FileUtils {
    /// Creates a new `FileUtils` instance with the specified file path.
    ///
    /// # Arguments
    ///
    /// * `file_path` - A string representing the file path.
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
        }
    }

    /// Returns the internal name of the file (file stem) if available.
    ///
    /// # Returns
    ///
    /// An `Option` containing the internal name as a `&str`, or `None` if not available.
    #[inline(always)]
    fn get_internal_name(&self) -> Option<&str> {
        Path::new(&self.file_path).file_stem().and_then(|stem| stem.to_str())
    }

    /// Returns the file name if available.
    ///
    /// # Returns
    ///
    /// An `Option` containing the file name as a `&str`, or `None` if not available.
    #[inline(always)]
    fn get_file_name(&self) -> Option<&str> {
        Path::new(&self.file_path).file_name()?.to_str()
    }

    /// Returns the file extension if available.
    ///
    /// # Returns
    ///
    /// An `Option` containing the file extension as a `&str`, or `None` if not available.
    #[inline(always)]
    fn get_file_extension(&self) -> Option<&str> {
        Path::new(&self.file_path).extension().and_then(|ext| ext.to_str())
    }

    /// Returns the size of the file.
    ///
    /// # Returns
    ///
    /// A `Result` containing the file size as `u64` on success, or a `std::io::Error` on failure.
    #[inline(always)]
    fn get_size(&self) -> Result<u64, std::io::Error> {
        fs::metadata(&self.file_path).map(|metadata| metadata.len())
    }

    /// Concatenates internal name, file size, and file extension into a single string.
    ///
    /// # Returns
    ///
    /// A concatenated string representing internal name, file size, and file extension.
    #[inline(always)]
    fn get_concatenate_file_tags(&self) -> String {
        let internal_name = self.get_internal_name().unwrap_or("");
        let file_size = self.get_size().map_or_else(|_| String::from(""), |size| size.to_string());
        let file_extension = self.get_file_extension().unwrap_or("");

        format!("{}{}{}", internal_name, file_size, file_extension)
    }

    /// Sets a new file name for the file by renaming it with a random string appended to the target directory.
    ///
    /// # Arguments
    ///
    /// * `target_dir` - The target directory to move the file to.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure of the file renaming operation.
    pub fn set_file_name(&self, target_dir: PathBuf) -> std::io::Result<()> {
        let rand_file_name = target_dir.join(generate_random_string(&self.get_concatenate_file_tags()));
        let new_path = format!("{}{}", self.file_path, &rand_file_name.display());

        fs::rename(&self.file_path, &new_path)
    }

    /// Deletes the file.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure of the file deletion operation.
    pub fn delete_file(&self) -> Result<(), std::io::Error> {
        fs::remove_file(&self.file_path).map(|_| ())
    }
}
