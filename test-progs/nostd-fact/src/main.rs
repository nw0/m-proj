#![feature(lang_items, box_syntax, start, libc, core_intrinsics)]
#![no_std]
use core::intrinsics;
use core::panic::PanicInfo;
use core::slice::from_raw_parts;
use core::char;

extern crate libc;
use libc::c_uint;

#[lang = "owned_box"]
pub struct Box<T>(*mut T);

#[lang = "exchange_malloc"]
unsafe fn allocate(size: usize, _align: usize) -> *mut u8 {
    let p = libc::malloc(size as libc::size_t) as *mut u8;

    // Check if `malloc` failed:
    if p as usize == 0 {
        intrinsics::abort();
    }

    p
}

#[lang = "box_free"]
unsafe fn box_free<T: ?Sized>(ptr: *mut T) {
    libc::free(ptr as *mut libc::c_void)
}

extern {
    pub fn printf(format: *const u8, ...) -> i32;
    pub fn putchar(c: i8) -> i8;
    pub fn strlen(s: *const u8) -> usize;
}

fn fact(x: u32) -> u32 {
    match x {
        0 => 1,
        _ => x * fact(x-1)
    }
}

#[start]
fn main(argc: isize, argv: *const *const u8) -> isize {
    let _x = box 1;

    unsafe {
        printf(b"Hello, world\nReceived %d args\n" as *const u8, argc-1);

        // From https://users.rust-lang.org/t/no-std-question/6454/3
        for arg in from_raw_parts(argv, argc as usize) {
            for ch in from_raw_parts(*arg, libc::strlen(*arg as *const i8)) {
                match (*ch as char).to_digit(10) {
                    // Some(x) => printf(b"%i" as *const u8, x),
                    // Some(x) => putchar(char::from_digit(x, 10).unwrap() as i8) as i32,
                    Some(x) => printf(b"%d\n" as *const u8, fact(x)),
                    _ => putchar(*ch as i8) as i32
                };
            }
            putchar('\n' as i8);
    }
}
    0
}

#[lang = "eh_personality"] extern fn rust_eh_personality() {}
#[lang = "panic_impl"] extern fn rust_begin_panic(info: &PanicInfo) -> ! { unsafe { intrinsics::abort() } }
#[lang = "eh_unwind_resume"] extern fn rust_eh_unwind_resume() {}
#[no_mangle] pub extern fn rust_eh_register_frames () {}
#[no_mangle] pub extern fn rust_eh_unregister_frames () {}
