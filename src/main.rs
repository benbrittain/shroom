#![feature(start)]
#![feature(no_std)]
#![feature(lang_items)]
#![no_std]

#![crate_type="staticlib"]

use rustsel4::boot_info;

#[lang="panic_fmt"]
pub fn panic_fmt(_fmt: &core::fmt::Arguments, _file_line: &(&'static str, usize)) -> !
{
        loop { }
}

#[no_mangle]
pub unsafe fn __aeabi_unwind_cpp_pr0() -> ()
{
        loop {}
}

#[lang="stack_exhausted"] extern fn stack_exhausted() {}
#[lang="eh_personality"] extern fn eh_personality() {}

#[start]
fn start(_:isize, _:*const *const u8)-> isize {
//    let info = sel4::get_boot_info();
    0
}

