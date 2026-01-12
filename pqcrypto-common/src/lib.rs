
use core::ffi::c_int;
use core::slice;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PQCLEAN_randombytes(buf: *mut u8, len: usize) -> c_int {
    let buf = unsafe { slice::from_raw_parts_mut(buf, len) };
    getrandom::fill(buf).expect("RNG Failed");
    0
}

