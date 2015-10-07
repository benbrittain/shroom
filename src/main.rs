#![feature(no_std)]
#![feature(core_str_ext)]
#![feature(lang_items)]
#![feature(start)]
#![no_std]
#![crate_type="staticlib"]

extern crate sel4_sys;

use sel4_sys::*;

#[link(name="c")]
extern { fn putchar(_:i32)->i32; }

#[lang="stack_exhausted"] extern fn stack_exhausted() {}
#[lang="eh_personality"] extern fn eh_personality() {}

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

#[start]
fn start(_:isize, _:*const *const u8)-> isize {
    unsafe {
        //seL4_DebugNameThread(3, "test\n");
        let x = seL4_GetCap(32);
        putchar(x as i32);
    }
    0
}

