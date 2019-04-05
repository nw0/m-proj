#![feature(no_core)]
#![feature(lang_items)]
#![feature(optin_builtin_traits)]
#![feature(on_unimplemented)]

#![no_std]

#[cfg(target_os = "freebsd")]
#[link(name = "c")]
extern {
    pub fn printf(format: *const u8, ...) -> i32;
}

#[lang = "start"]
fn start(_main: *const u8, _argc: isize, _argv: *const *const u8) -> isize {
    main();
    0
}

fn print_sp(i: i32) {
    unsafe {
        printf(b"%d \0" as *const u8, i);
    }
}

fn main() {
    use core::mem::swap;
    use core::intrinsics::{copy, copy_nonoverlapping};

    let mut x: i64 = 5;
    let mut y: i64 = 6;
    unsafe { copy(&x, &mut y, 1); }
    print_sp(x as i32);
    print_sp(y as i32);
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
