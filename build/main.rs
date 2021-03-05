mod archive;
mod bindgen;
mod build;

use crate::{
    archive::MbedTlsTarball,
    bindgen::{ IncludeDir, Bindgen },
    build::{ BuildDir, ConfigH, MbedTls }
};


/// The main function :P
fn main() {
    // Create dir and open archive
    let build_dir = BuildDir::from_env().unwrap_or_default();
    let archive = MbedTlsTarball::from_env().unwrap_or_default();

    // Extract archive and build library
    archive.extract(build_dir.path());
    let config_h = ConfigH::from_env().unwrap_or_default();
    let artifacts = MbedTls::new(config_h, build_dir).build();

    // Link library
    println!("cargo:rustc-link-search=native={}/lib", artifacts.display());
    println!("cargo:rustc-link-lib=static=mbedcrypto");

    // Generate bindings
    let include_dir = IncludeDir::new(artifacts);
    Bindgen::new(include_dir).generate();
}