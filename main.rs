#![no_std]
#![no_main]

use core::ffi::{c_int, c_void};
use core::panic::PanicInfo;
use core::ptr;
use core::mem::{zeroed, size_of};

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

#[repr(C)]
#[derive(Clone, Copy)]
pub struct List<T> {
    prev: *mut List<T>,
    next: *mut List<T>,
    item: T,
}
extern "C" {
    fn printf(fmt: *const u8, ...) -> c_int;
    fn realloc(prt: *mut c_void, size: usize) -> *mut c_void;
    fn malloc(size: usize) -> *mut c_void;
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

unsafe fn list_push<T: Copy>(first: &mut *mut List<T>, last: &mut *mut List<T>, item: T) {
    let new_node = malloc(size_of::<List<T>>()) as *mut List<T>;
    if new_node.is_null() {
        return;
    }
    (*new_node).item = item;
    (*new_node).next = ptr::null_mut();
    (*new_node).prev = *last;

    if (*last).is_null() {
        *first = new_node;
        *last = new_node;
    } else {
        (**last).next = new_node;
        *last = new_node;
    }
}

#[no_mangle]
unsafe extern "C" fn main(argc:i32, argv:*mut *mut u8) -> i32 {
   printf(b"foo %d\n\0".as_ptr(), argc);
   for i in 0 .. argc {
       printf(b"%s\n\0".as_ptr(), *argv.add(i as usize));
   }
   let mut l:Array<i32>=zeroed();
   for i in 0 .. 100 {
       array_push(&mut l, i);
   }
   for i in 0 .. l.count {
       printf(b"%d \0".as_ptr(), *(l.list.add(i as usize)));
   }
   
   printf(b"\n-- Double Linked List Demo --\n\0".as_ptr());
   let mut first: *mut List<i32> = ptr::null_mut();
   let mut last: *mut List<i32> = ptr::null_mut();
   
   list_push(&mut first, &mut last, 32);
   list_push(&mut first, &mut last, 64);
   list_push(&mut first, &mut last, 128);

   let mut current = first;
   while !current.is_null() {
       printf(b"%d \0".as_ptr(), (*current).item);
       current = (*current).next;
   }
   printf(b"\n\0".as_ptr());
   
   48
}

