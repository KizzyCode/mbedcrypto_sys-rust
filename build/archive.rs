use std::{
    env, process::Command,
    path::{ Component, Path, PathBuf }
};


/// An mbedTLS release tarball
pub struct MbedTlsTarball {
    /// The release tarball path
    archive: PathBuf
}
impl MbedTlsTarball {
    /// Loads an release from the environment
    pub fn from_env() -> Option<Self> {
        let archive = env::var("MBEDCRYPTO_ARCHIVE").ok()?;
        Some(MbedTlsTarball { archive: PathBuf::from(archive) })
    }

    /// The path to the tarball
    pub fn path(&self) -> &PathBuf {
        &self.archive
    }
}
impl Default for MbedTlsTarball {
    fn default() -> Self {
        let archive = env::current_dir().expect("Failed to get working directory")
            .join("mbedtls").join("mbedtls-2.25.0.tar.gz");
        MbedTlsTarball { archive }
    }
}


/// Downloads a file using a pre-installed tar binary
pub struct Untar {
    /// The location of the binary
    binary: PathBuf
}
impl Untar {
    /// Loads a tar path from the environment
    pub fn from_env() -> Option<Self> {
        let binary = PathBuf::from(env::var("MBEDCRYPTO_TAR").ok()?);
        Some(Self { binary })
    }

    /// Extracts an `archive` into `dest` and strips n leading path components during extraction
    pub fn extract<A, D>(&self, archive: A, dest: D, strip_n: usize) where A: AsRef<Path>, D: AsRef<Path> {
        // Create string arguments
        let archive_str = archive.as_ref().to_str().expect("Invalid archive path");
        let dest_str = dest.as_ref().to_str().expect("Invalid target path");
        let strip_string = format!("--strip-components={}", strip_n);
        
        // Extract archive
        dbg!("tar", "--extract", &strip_string, "--directory", dest_str, "--file", archive_str);
        Command::new(&self.binary)
            .args(vec!["--extract", &strip_string, "--directory", dest_str, "--file", archive_str])
            .output().expect("Failed to extract file");
    }
}
impl Default for Untar {
    fn default() -> Self {
        // Common prefixes for installed binaries
        let prefixes = vec![
            vec!["bin"],
            vec!["usr", "bin"],
            vec!["usr", "local", "bin"]
        ];

        // Find a curl instance
        for prefix in prefixes {
            // Create the path
            let mut binary = (Component::RootDir.as_ref() as &Path).to_owned();
            binary.extend(prefix);
            binary.push("tar");

            // Test if the binary exists
            if binary.exists() {
                return Self { binary };
            }
        }

        // Raise a fatal error
        panic!("`tar` could not be found. Set MBEDCRYPTO_TAR to specify a path manually.")
    }
}