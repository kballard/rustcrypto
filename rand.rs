use std::libc::c_int;
use std::vec;

mod libcrypto {
    use std::libc::c_int;

    #[link(name = "crypto")]
    extern {
        pub fn RAND_bytes(buf: *mut u8, num: c_int) -> c_int;
    }
}

pub fn rand_bytes(len: uint) -> ~[u8] {
    let mut out = vec::with_capacity(len);

    let r = unsafe { libcrypto::RAND_bytes(out.as_mut_ptr(), out.len() as c_int) };
    if r != 1 as c_int { fail!() }

    unsafe { out.set_len(len); }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rand_bytes() {
        let bytes = rand_bytes(32u);
        println!("{:?}", bytes);
    }
}
