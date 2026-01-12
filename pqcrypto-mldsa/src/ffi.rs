use core::ffi::c_int;

// ensures we link correctly
#[allow(unused_imports)]
use pqcrypto_common::*;


#[link(name = "ml-dsa-44_clean")]
unsafe extern "C" {
    pub fn PQCLEAN_MLDSA44_CLEAN_crypto_sign_keypair(pk: *mut u8, sk: *mut u8) -> c_int;
    pub fn PQCLEAN_MLDSA44_CLEAN_crypto_sign(
        sm: *mut u8,
        smlen: *mut usize,
        msg: *const u8,
        len: usize,
        sk: *const u8,
    ) -> c_int;
    pub fn PQCLEAN_MLDSA44_CLEAN_crypto_sign_ctx(
        sm: *mut u8,
        smlen: *mut usize,
        msg: *const u8,
        len: usize,
        ctx: *const u8,
        ctxlen: usize,
        sk: *const u8,
    ) -> c_int;
    pub fn PQCLEAN_MLDSA44_CLEAN_crypto_sign_open(
        m: *mut u8,
        mlen: *mut usize,
        sm: *const u8,
        smlen: usize,
        pk: *const u8,
    ) -> c_int;
    pub fn PQCLEAN_MLDSA44_CLEAN_crypto_sign_open_ctx(
        m: *mut u8,
        mlen: *mut usize,
        sm: *const u8,
        smlen: usize,
        ctx: *const u8,
        ctxlen: usize,
        pk: *const u8,
    ) -> c_int;
    pub fn PQCLEAN_MLDSA44_CLEAN_crypto_sign_signature(
        sig: *mut u8,
        siglen: *mut usize,
        m: *const u8,
        mlen: usize,
        sk: *const u8,
    ) -> c_int;
    pub fn PQCLEAN_MLDSA44_CLEAN_crypto_sign_signature_ctx(
        sig: *mut u8,
        siglen: *mut usize,
        m: *const u8,
        mlen: usize,
        ctx: *const u8,
        ctxlen: usize,
        sk: *const u8,
    ) -> c_int;
    pub fn PQCLEAN_MLDSA44_CLEAN_crypto_sign_verify(
        sig: *const u8,
        siglen: usize,
        m: *const u8,
        mlen: usize,
        pk: *const u8,
    ) -> c_int;
    pub fn PQCLEAN_MLDSA44_CLEAN_crypto_sign_verify_ctx(
        sig: *const u8,
        siglen: usize,
        m: *const u8,
        mlen: usize,
        ctx: *const u8,
        ctxlen: usize,
        pk: *const u8,
    ) -> c_int;
}