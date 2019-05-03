extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};

#[cfg(target_os = "freebsd")]
#[link(name = "c")]
extern {
    pub fn printf(format: *const u8, ...) -> i32;
    pub fn malloc(size: u64) -> *mut u8;
    pub fn realloc(ptr: *mut u8, new_size: u64) -> *mut u8;
    pub fn free(ptr: *mut u8);
}

macro_rules! prntf {
    ( $fmt:expr ) => {
        unsafe { printf(($fmt.to_owned() + "\0").as_bytes().as_ptr()); }
    };
    ( $fmt:expr, $($var:expr),* ) => {
        unsafe { printf(($fmt.to_owned() + "\0").as_bytes().as_ptr(), $( $var , )*); }
    }
}

pub struct HeapAlloc;

unsafe impl GlobalAlloc for HeapAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        malloc(layout.size() as u64)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr)
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        realloc(ptr, new_size as u64)
    }
}

#[alloc_error_handler]
fn alloc(_: Layout) -> ! { loop {} }

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
#[lang = "eh_personality"] extern fn rust_eh_personality() {}
#[lang = "eh_unwind_resume"] extern fn rust_eh_unwind_resume() {}
#[no_mangle] pub extern fn rust_eh_register_frames () {}
#[no_mangle] pub extern fn rust_eh_unregister_frames () {}
