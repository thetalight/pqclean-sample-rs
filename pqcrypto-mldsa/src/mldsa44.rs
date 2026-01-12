

use crate::{ffi, PublicKey, SecretKey, Signature};

pub const PQCLEAN_MLDSA44_CLEAN_CRYPTO_SECRETKEYBYTES: usize = 2560;
pub const PQCLEAN_MLDSA44_CLEAN_CRYPTO_PUBLICKEYBYTES: usize = 1312;
pub const PQCLEAN_MLDSA44_CLEAN_CRYPTO_BYTES: usize = 2420;


pub fn keypair() -> (PublicKey, SecretKey) {
    let mut pk = vec![0; PQCLEAN_MLDSA44_CLEAN_CRYPTO_PUBLICKEYBYTES];
    let mut sk = vec![0; PQCLEAN_MLDSA44_CLEAN_CRYPTO_SECRETKEYBYTES];
    assert_eq!(
        unsafe { ffi::PQCLEAN_MLDSA44_CLEAN_crypto_sign_keypair(pk.as_mut_ptr(), sk.as_mut_ptr()) },
        0
    );

    (pk, sk)
}

pub fn sign(sk: &[u8],msg: &[u8], ) -> Signature{
    let max_len = msg.len() + PQCLEAN_MLDSA44_CLEAN_CRYPTO_BYTES;
    let mut signed_msg = vec![0;max_len];
    let mut sm_len: usize = 0;


    assert_eq!(
        unsafe { ffi::PQCLEAN_MLDSA44_CLEAN_crypto_sign(signed_msg.as_mut_ptr(), &mut sm_len, msg.as_ptr(), msg.len(), sk.as_ptr()) },
        0
    );

    signed_msg.truncate(sm_len);

    signed_msg
}

pub fn open(pk: &[u8], sign: &[u8]) -> Vec<u8> {
   let mut msg = vec![0;sign.len()];
   let  mut msg_len = 0usize;

    assert_eq!(
        unsafe { ffi::PQCLEAN_MLDSA44_CLEAN_crypto_sign_open(msg.as_mut_ptr(), &mut msg_len, sign.as_ptr(), sign.len(), pk.as_ptr()) },
        0
    );

    msg.truncate(msg_len);

    msg
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mldsa44() {
        let (pk, sk) = keypair();
        
        let msg = b"hello world";

        // include the msg in the signature
        let sign = sign(&sk, msg);

        let open = open(&pk, &sign);

        assert_eq!(msg.to_vec(), open);
    }
}