use std::{
    env,
    path::{ Path, PathBuf }
};
use bindgen::RustTarget;


/// The mbedTLS include dir
pub struct IncludeDir {
    /// The artifacts include dir
    path: PathBuf
}
impl IncludeDir {
    /// Creates a new include directory handle from the artifacts directors
    pub fn new<P>(artifacts: P) -> Self where P: AsRef<Path> {
        let mut include = artifacts.as_ref().to_owned();
        include.push("include");
        Self { path: include }
    }

    /// The path to the include dir
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}


/// The headers
pub struct Headers {
    /// A list to the headers
    headers: Vec<String>
}
impl Headers {
    /// Loads a header list from the environment
    pub fn from_env() -> Option<Self> {
        let headers = env::var("MBEDCRYPTO_HEADERS").ok()?.split(",")
            .map(|h| h.to_string()).collect();
        Some(Self { headers })
    }

    /// Creates full paths to the include files
    pub fn paths(&self, include_dir: &IncludeDir) -> Vec<String> {
        self.headers.iter().map(|h| format!("{}/mbedtls/{}", include_dir.path().display(), h)).collect()
    }
}
impl Default for Headers {
    fn default() -> Self {
        let headers = [
            "aes.h", "aesni.h", "arc4.h", "aria.h", "asn1.h", "asn1write.h", "base64.h", "bignum.h",
            "blowfish.h", "bn_mul.h", "camellia.h", "ccm.h", /* "certs.h", */ "chacha20.h", "chachapoly.h",
            /* "check_config.h", */ "cipher.h", /* "cipher_internal.h", */ "cmac.h", /* "compat-1.3.h", */ "config.h",
            /* "config_psa.h", */ "ctr_drbg.h", /* "debug.h", */ "des.h", "dhm.h", "ecdh.h", "ecdsa.h", "ecjpake.h",
            "ecp.h", /* "ecp_internal.h", */ "entropy.h", "entropy_poll.h", "error.h", "gcm.h", /* "havege.h", */
            "hkdf.h", "hmac_drbg.h", "md.h", "md2.h", "md4.h", "md5.h", /* "md_internal.h", */
            /* "memory_buffer_alloc.h", */ /* "net.h", */ /* "net_sockets.h", */ "nist_kw.h", "oid.h", "padlock.h",
            "pem.h", "pk.h", /* "pk_internal.h", */ /* "pkcs11.h", */ "pkcs12.h", "pkcs5.h", "platform.h",
            "platform_time.h", "platform_util.h", "poly1305.h", /* "psa_util.h", */ "ripemd160.h", "rsa.h",
            /* "rsa_internal.h", */ "sha1.h", "sha256.h", "sha512.h", /* "ssl.h", */ /* "ssl_cache.h", */
            /* "ssl_ciphersuites.h", */ /* "ssl_cookie.h", */ /* "ssl_internal.h", */ /* "ssl_ticket.h", */
            /* "threading.h", */ /* "timing.h", */ "version.h", /* "x509.h", */ /* "x509_crl.h", */ /* "x509_crt.h", */
            /* "x509_csr.h", */ "xtea.h"
        ];
        Self { headers: headers.iter().map(|h| h.to_string()).collect() }
    }
}


/// Generates Rust-bindings from the header files
pub struct Bindgen {
    /// The include dir
    include_dir: IncludeDir,
    /// The headers
    headers: Headers
}
impl Bindgen {
    /// Generates a new bindgen generator
    pub fn new(include_dir: IncludeDir) -> Self {
        let headers = Headers::from_env().unwrap_or_default();
        Self { include_dir, headers }
    }

    /// Generates the bindings
    pub fn generate(&self) {
        // Create the target path
        let mut mbedtls_rs = env::current_dir().expect("Failed to get working directory");
        mbedtls_rs.push("src");
        mbedtls_rs.push("mbedtls.rs");

        // Configure the builder
        let mut builder = bindgen::builder()
            .clang_arg(format!("-I{}", self.include_dir.path().display()))
            .size_t_is_usize(true)
            .rust_target(RustTarget::Stable_1_40)
            .derive_copy(true)
            .whitelist_recursively(false)
            .generate_comments(false)
            // mbedTLS types
            .whitelist_type("mbedtls.*").whitelist_type("MBEDTLS.*")
            .whitelist_function("mbedtls.*").whitelist_function("MBEDTLS.*")
            .whitelist_var("mbedtls.*").whitelist_var("MBEDTLS.*")
            // C-stdlib types
            .whitelist_type("time_t").opaque_type("time_t");
        
        // Register headers and generate and write the bindings
        for header in self.headers.paths(&self.include_dir) {
            builder = builder.header(header);
        }
        builder.generate().expect("Failed to generate bindings")
            .write_to_file(mbedtls_rs).expect("Failed to write bindings");
    }
}