use std::fs;
use std::path::{Path, PathBuf};
use std::io;





pub struct QuarantineManager {
    quarantine_file_name: String,
}


pub enum QuarantineResult {
    /// The file was successfully quarantined.
    Quarantined,
    /// The file failed to be quarantined and was subsequently deleted.
    QuarantinedFailedDeleted,
    /// Deleting the file from the quarantine directory failed.
    DeletedFailed,
    /// The quarantine operation encountered a total failure.
    TotalFailuare,
}

impl QuarantineManager {
    /// Creates a new `QuarantineManager` with the specified quarantine file name.
    ///
    /// # Arguments
    ///
    /// * `quarantine_file_name` - The name of the quarantine file.
    pub fn new(quarantine_file_name: String) -> QuarantineManager {
        QuarantineManager {
            quarantine_file_name,
        }
    }

    /// Generates the path to the quarantine directory.
    ///
    /// # Returns
    ///
    /// A `PathBuf` representing the path to the quarantine directory.
    #[inline(always)]
    fn quarantine_directory_path(&self) -> PathBuf {
        let program_data_dir = std::env::var("ProgramData").unwrap_or_else(|_| String::from("C:\\ProgramData"));
        Path::new(&program_data_dir).join("RosaryAV")
    }

    /// Creates the quarantine directory if it doesn't exist.
    ///
    /// # Returns
    ///
    /// An `io::Result` indicating success or failure.
    #[inline(always)]
    fn create_quarantine_directory(&self) -> io::Result<()> {
        fs::create_dir_all(&self.quarantine_directory_path())
    }

    /// Checks if the quarantine directory exists.
    ///
    /// # Returns
    ///
    /// `true` if the quarantine directory exists, otherwise `false`.
    #[inline(always)]
    fn quarantine_directory_exists(&self) -> bool {
        self.quarantine_directory_path().is_dir()
    }

    /// Moves a file to the quarantine directory.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path of the file to be quarantined.
    ///
    /// # Returns
    ///
    /// A `QuarantineResult` indicating the outcome of the quarantine operation.
    pub fn move_to_quarantine(&self, file_path: &str) -> QuarantineResult {

        if !self.quarantine_directory_exists() {
            match self.create_quarantine_directory() {
                Ok(_) => {}
                Err(_) => {
                    let file_utils = crate::fileutils::FileUtils::new(file_path);
                    if let Err(_) = file_utils.delete_file() {
                        return QuarantineResult::DeletedFailed;
                    }
                    return QuarantineResult::QuarantinedFailedDeleted;
                }
            }
        }

        let file_utils = crate::fileutils::FileUtils::new(file_path).set_file_name(self.quarantine_directory_path());
        match file_utils {
            Ok(()) => QuarantineResult::Quarantined,
            Err(_) => QuarantineResult::TotalFailuare,
        }
    }
}
