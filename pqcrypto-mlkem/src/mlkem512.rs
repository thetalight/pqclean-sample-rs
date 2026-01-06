use std::vec;

use crate::{Ciphertext, PublicKey, SecretKey, SharedSecret, ffi};

pub const PQCLEAN_MLKEM512_CLEAN_CRYPTO_SECRETKEYBYTES: usize = 1632;
pub const PQCLEAN_MLKEM512_CLEAN_CRYPTO_PUBLICKEYBYTES: usize = 800;
pub const PQCLEAN_MLKEM512_CLEAN_CRYPTO_CIPHERTEXTBYTES: usize = 768;
pub const PQCLEAN_MLKEM512_CLEAN_CRYPTO_BYTES: usize = 32;

pub fn keypair() -> (PublicKey, SecretKey) {
    let mut pk = vec![0; PQCLEAN_MLKEM512_CLEAN_CRYPTO_PUBLICKEYBYTES];
    let mut sk = vec![0; PQCLEAN_MLKEM512_CLEAN_CRYPTO_SECRETKEYBYTES];
    assert_eq!(
        unsafe { ffi::PQCLEAN_MLKEM512_CLEAN_crypto_kem_keypair(pk.as_mut_ptr(), sk.as_mut_ptr()) },
        0
    );

    (pk, sk)
}

pub fn encapsulate(pk: &PublicKey) -> (SharedSecret, Ciphertext) {
    let mut ss = vec![0; PQCLEAN_MLKEM512_CLEAN_CRYPTO_BYTES];
    let mut ct = vec![0; PQCLEAN_MLKEM512_CLEAN_CRYPTO_CIPHERTEXTBYTES];

    assert_eq!(
        unsafe {
            ffi::PQCLEAN_MLKEM512_CLEAN_crypto_kem_enc(
                ct.as_mut_ptr(),
                ss.as_mut_ptr(),
                pk.as_ptr(),
            )
        },
        0
    );

    (ss, ct)
}

pub fn decapsulate(ct: &Ciphertext, sk: &SecretKey) -> SharedSecret {
    let mut ss = vec![0; PQCLEAN_MLKEM512_CLEAN_CRYPTO_BYTES];

    assert_eq!(
        unsafe {
            ffi::PQCLEAN_MLKEM512_CLEAN_crypto_kem_dec(ss.as_mut_ptr(), ct.as_ptr(), sk.as_ptr())
        },
        0
    );

    ss
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair() {
        let (pk, sk) = keypair();

        let (ss, ct) = encapsulate(&pk);

        let ss2 = decapsulate(&ct, &sk);

        assert_eq!(ss, ss2);
    }
}
