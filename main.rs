#![no_std]
#![no_main]

use core::ffi::{c_int, c_void};
use core::panic::PanicInfo;
use core::ptr;

#[panic_handler]
unsafe fn panic(_info:&PanicInfo) -> ! {
    loop{}
}

extern "C" {
    fn printf(fmt: *const u8, ...) -> c_int;
    // fn exit(code:i32)-> !;
}

#[no_mangle]
unsafe extern "C" fn main(argc:i32, argv:*mut *mut u8) -> i32 {
   printf(b"foo %d\n\0".as_ptr(), argc);
   for i in 0 .. argc {
       printf(b"%s\n".as_ptr(), *argv.add(i as usize));
   }
   // exit(48);
   48
}

