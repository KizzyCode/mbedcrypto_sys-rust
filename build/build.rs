use std::{
    fs, env, thread, time::Duration,
    path::{ Path, PathBuf }
};


/// A build directory
pub struct BuildDir {
    /// The path to the build directory
    path: PathBuf
}
impl BuildDir {
    /// Loads a build directory path from the environment
    pub fn from_env() -> Option<Self> {
        let path = PathBuf::from(env::var("MBEDCRYPTO_BUILD_DIR").ok()?);
        Some(Self::new(path))
    }
    /// Creates a new build directory
    fn new(path: PathBuf) -> Self {
        // Delete an existing directory
        'delete_loop: while path.exists() {
            match fs::remove_dir_all(&path) {
                Ok(_) => break 'delete_loop,
                Err(_) => thread::sleep(Duration::from_secs(1))
            }
        }

        // Create the directory
        fs::create_dir_all(&path).expect("Failed to create build dir");
        Self { path }
    }

    /// The path to the temp dir
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}
impl Default for BuildDir {
    fn default() -> Self {
        let mut path = env::current_dir().expect("Failed to get working directory");
        path.push("target");
        path.push("mbedtls");

        Self::new(path)
    }
}


/// A config header
pub struct ConfigH {
    /// A path buffer
    path: PathBuf
}
impl ConfigH {
    /// Loads a build directory path from the environment
    pub fn from_env() -> Option<Self> {
        let path = PathBuf::from(env::var("MBEDCRYPTO_CONFIG_H").ok()?);
        Some(Self { path })
    }

    /// Copies the header to `dest`
    pub fn copy<P>(&self, dest: P) where P: AsRef<Path> {
        fs::copy(&self.path, dest).expect("Failed to copy config.h");
    }
}
impl Default for ConfigH {
    fn default() -> Self {
        let mut path = PathBuf::from(".");
        path.push("mbedtls");
        path.push("config.h");

        Self { path }
    }
}


/// A mbedTLS build instance
pub struct MbedTls {
    /// The config.h header
    config_h: ConfigH,
    /// The mbedtls directory
    build_dir: BuildDir
}
impl MbedTls {
    /// Creates a new build instance
    pub fn new(config_h: ConfigH, build_dir: BuildDir) -> Self {
        Self { config_h, build_dir }
    }

    /// Builds `mbedTLS` and returns the path to the artifacts
    pub fn build(&self) -> PathBuf {
        // Copy config.h
        let mut config_h_dest = self.build_dir.path().clone();
        config_h_dest.push("include");
        config_h_dest.push("mbedtls");
        config_h_dest.push("config.h");
        self.config_h.copy(config_h_dest);

        // Build the library
        cmake::build(self.build_dir.path())
    }
}