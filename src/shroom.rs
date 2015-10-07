#![feature(no_std, start, lang_items)]
#![feature(core_str_ext)]

#![no_std]

extern crate sel4_sys;
use sel4_sys::*;

#[start]
pub fn rsl4_init(_:isize, _:*const *const u8) -> isize {
    let mut x: u8 = 0x30;
    loop {
        unsafe { seL4_DebugPutChar(x); }
        x = x + 1;
        if x > 0x39 {
            panic!();
        }
    }
    0
}

#[lang="stack_exhausted"] extern fn stack_exhausted() {}
#[lang="eh_personality"] extern fn eh_personality() {}

//pub fn debugPutStr(s: &str) {
//    for b in s.bytes() {
//        unsafe {
//            seL4_DebugPutChar(b);
//        }
//    }
//}
//struct DebugWriter;
//impl fmt::Write for DebugWriter {
//    fn write_str(&mut self, s: &str) -> fmt::Result {
//        debugPutStr(s);
//        Ok(())
//    }
//}

#[lang = "panic_fmt"]
pub extern fn rust_begin_unwind(_fmt: &core::fmt::Arguments, _file_line: &(&'static str, usize)) -> ! {
//    let mut debugWriter = DebugWriter;
//    let _ = fmt::write(&mut debugWriter, msg);
//    debugPutStr(file);
    loop {}
}

#[no_mangle]
extern fn __aeabi_unwind_cpp_pr0() {}
#[no_mangle]
extern fn __aeabi_unwind_cpp_pr1() {}
