#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
include!("mbedtls.rs");


/// Performs the mbedTLS self tests
pub fn all_selftests() {
    /// Executes a test function
    macro_rules! test_fn {
        ($desc:expr, $name:ident) => ({
            assert_eq!(unsafe { crate::$name(1) }, 0, "{} failed", $desc);
        });
    }

    // The self tests
    /* test_fn!("calloc", calloc_self_test); */
    test_fn!("md2", mbedtls_md2_self_test);
    test_fn!("md4", mbedtls_md4_self_test);
    test_fn!("md5", mbedtls_md5_self_test);
    test_fn!("ripemd160", mbedtls_ripemd160_self_test);
    test_fn!("sha1", mbedtls_sha1_self_test);
    test_fn!("sha256", mbedtls_sha256_self_test);
    test_fn!("sha512", mbedtls_sha512_self_test);
    test_fn!("arc4", mbedtls_arc4_self_test);
    test_fn!("des", mbedtls_des_self_test);
    test_fn!("aes", mbedtls_aes_self_test);
    test_fn!("gcm", mbedtls_gcm_self_test);
    test_fn!("ccm", mbedtls_ccm_self_test);
    test_fn!("nist_kw", mbedtls_nist_kw_self_test);
    test_fn!("cmac", mbedtls_cmac_self_test);
    test_fn!("chacha20", mbedtls_chacha20_self_test);
    test_fn!("poly1305", mbedtls_poly1305_self_test);
    test_fn!("chacha20-poly1305", mbedtls_chachapoly_self_test);
    test_fn!("base64", mbedtls_base64_self_test);
    test_fn!("mpi", mbedtls_mpi_self_test);
    test_fn!("rsa", mbedtls_rsa_self_test);
    /* test_fn!("x509", mbedtls_x509_self_test); */
    test_fn!("xtea", mbedtls_xtea_self_test);
    test_fn!("camellia", mbedtls_camellia_self_test);
    test_fn!("aria", mbedtls_aria_self_test);
    test_fn!("ctr_drbg", mbedtls_ctr_drbg_self_test);
    test_fn!("hmac_drbg", mbedtls_hmac_drbg_self_test);
    test_fn!("ecp", mbedtls_ecp_self_test);
    test_fn!("ecjpake", mbedtls_ecjpake_self_test);
    test_fn!("dhm", mbedtls_dhm_self_test);
    /* test_fn!("entropy", mbedtls_entropy_self_test_wrapper); */
    test_fn!("pkcs5", mbedtls_pkcs5_self_test);
    /* test_fn!("timing", mbedtls_timing_self_test); */
    /* test_fn!("memory_buffer_alloc", mbedtls_memory_buffer_alloc_free_and_self_test); */
}