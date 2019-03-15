#![feature(no_core)]
#![feature(lang_items)]
#![feature(optin_builtin_traits)]
#![feature(on_unimplemented)]

#![no_core]

#[cfg(target_os = "freebsd")]
#[link(name = "c")]
extern {}

#[lang = "sized"]
pub trait Sized {}
#[lang = "copy"]
pub trait Copy {}

#[lang = "freeze"]
pub(crate) unsafe auto trait Freeze {}

#[lang = "sync"]
#[rustc_on_unimplemented(
    message="`{Self}` cannot be shared between threads safely",
    label="`{Self}` cannot be shared between threads safely"
)]
pub unsafe auto trait Sync {}

#[lang = "start"]
fn start(_main: *const u8, _argc: isize, _argv: *const *const u8) -> isize {
    1
}

fn main() {
}
