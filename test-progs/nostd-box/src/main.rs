#![feature(lang_items)]
#![feature(optin_builtin_traits)]
#![feature(on_unimplemented)]
#![feature(alloc)]
#![feature(alloc_error_handler)]

#![no_std]

extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};

use alloc::boxed::Box;

#[cfg(target_os = "freebsd")]
#[link(name = "c")]
extern {
    pub fn printf(format: *const u8, ...) -> i32;
    pub fn malloc(size: u64) -> *mut u8;
    pub fn free(ptr: *mut u8);
}

#[lang = "start"]
fn start(_main: *const u8, _argc: isize, _argv: *const *const u8) -> isize {
    main();
    0
}

struct HeapAlloc;

unsafe impl GlobalAlloc for HeapAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        malloc(layout.size() as u64)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        free(ptr)
    }
}

#[global_allocator]
static HEAP: HeapAlloc = HeapAlloc;

#[alloc_error_handler]
fn alloc(_: Layout) -> ! { loop {} }

fn print_sp(i: i32) {
    unsafe {
        printf(b"%d \0" as *const u8, i);
    }
}

fn main() {
    let x: Box<i32> = Box::new(4);
    print_sp(*x);
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
#[lang = "eh_personality"] extern fn rust_eh_personality() {}
#[lang = "eh_unwind_resume"] extern fn rust_eh_unwind_resume() {}
#[no_mangle] pub extern fn rust_eh_register_frames () {}
#[no_mangle] pub extern fn rust_eh_unregister_frames () {}
