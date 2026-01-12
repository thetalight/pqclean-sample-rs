use core::ffi::c_int;

// ensures we link correctly
#[allow(unused_imports)]
use pqcrypto_common::*;

#[link(name = "ml-kem-512_clean")]
unsafe extern "C" {
    pub fn PQCLEAN_MLKEM512_CLEAN_crypto_kem_keypair(pk: *mut u8, sk: *mut u8) -> c_int;
    pub fn PQCLEAN_MLKEM512_CLEAN_crypto_kem_enc(ct: *mut u8, ss: *mut u8, pk: *const u8) -> c_int;
    pub fn PQCLEAN_MLKEM512_CLEAN_crypto_kem_dec(
        ss: *mut u8,
        ct: *const u8,
        sk: *const u8,
    ) -> c_int;
}