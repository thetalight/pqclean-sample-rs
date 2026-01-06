pub mod ffi;
pub mod mlkem512;

pub type PublicKey = Vec<u8>;
pub type SecretKey = Vec<u8>;
pub type SharedSecret = Vec<u8>;
pub type Ciphertext = Vec<u8>;
