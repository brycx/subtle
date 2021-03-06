#![no_main]
#![feature(i128_type)]

#[macro_use]
extern crate libfuzzer_sys;
extern crate subtle;
extern crate core;

use core::intrinsics::transmute;

use subtle::ConditionallyAssignable;

fuzz_target!(|data: &[u8]| {
    let chunk_size: usize = 16;

    if data.len() % chunk_size != 0 {
        return;
    }

    for bytes in data.chunks(chunk_size) {
        unsafe {
            let mut x: i128 = 0;
            let y: i128 = transmute::<[u8; 16], i128>([
                bytes[0],  bytes[1],  bytes[2],  bytes[3],
                bytes[4],  bytes[5],  bytes[6],  bytes[7],
                bytes[8],  bytes[9],  bytes[10], bytes[11],
                bytes[12], bytes[13], bytes[14], bytes[15]]);

            x.conditional_assign(&y, 0);
            assert_eq!(x, 0);

            x.conditional_assign(&y, 1);
            assert_eq!(x, y);
        }
    }
});
