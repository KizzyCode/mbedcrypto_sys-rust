mod archive;
mod bindgen;
mod build;

use crate::{
    archive::{ MbedTlsTarball, Untar },
    bindgen::{ IncludeDir, Headers, Bindgen },
    build::{ BuildDir, ConfigH, MbedTls }
};


/// The main function :P
fn main() {
    // Extract archive
    let build_dir = BuildDir::from_env().unwrap_or_default();
    let archive = MbedTlsTarball::from_env().unwrap_or_default();
    Untar::from_env().unwrap_or_default().extract(archive.path(), build_dir.path(), 1);

    // Build library
    let config_h = ConfigH::from_env().unwrap_or_default();
    let artifacts = MbedTls::new(config_h, build_dir).build();

    // Link library
    let lib_dir = artifacts.join("lib");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=mbedcrypto");

    // Generate bindings
    let include_dir = IncludeDir::new(artifacts);
    let headers = Headers::from_env().unwrap_or_default();
    Bindgen::new(include_dir, headers).generate();
}