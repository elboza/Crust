#![no_std]
#![no_main]

use core::ffi::{c_int, c_void};
use core::panic::PanicInfo;
use core::ptr;
use core::mem::zeroed;

#[panic_handler]
unsafe fn panic(_info:&PanicInfo) -> ! {
    loop{}
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Array<T> {
    count: usize,
    capacity: usize,
    list: *mut T,
}
extern "C" {
    fn printf(fmt: *const u8, ...) -> c_int;
    fn realloc(prt: *mut c_void, size: usize) -> *mut c_void;
    // fn exit(code:i32)-> !;
}

unsafe fn array_push<T>(l: *mut Array<T>, item: T) {
    if((*l).count>=(*l).capacity) {
        if((*l).capacity==0){
            (*l).capacity=5;
        } else {
            (*l).capacity*=2;
        }
        (*l).list=realloc((*l).list as *mut c_void, size_of::<T>()*(*l).capacity) as *mut T;
    }
    *((*l).list.add((*l).count))=item;
    (*l).count+=1;
}

#[no_mangle]
unsafe extern "C" fn main(argc:i32, argv:*mut *mut u8) -> i32 {
   printf(b"foo %d\n\0".as_ptr(), argc);
   for i in 0 .. argc {
       printf(b"%s\n".as_ptr(), *argv.add(i as usize));
   }
   let mut l:Array<i32>=zeroed();
   for i in 0 .. 100 {
       array_push(&mut l, i);
   }
   for i in 0 .. l.count {
       printf(b"%d ".as_ptr(), *(l.list.add(i as usize)));
   }
   // exit(48);
   48
}

