/// Runs all self tests
#[test]
fn selftest() {
    mbedcrypto_sys::all_selftests()
}