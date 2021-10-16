#![no_std]

#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static TEST: &[u8] = b"Hey what are you looking at?";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let buffer = 0xb8000 as *mut u8;
    for (i, &byte) in TEST.iter().enumerate() {
        unsafe {
            *buffer.offset(i as isize * 2) = byte;
            *buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}