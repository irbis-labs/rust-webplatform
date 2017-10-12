#![allow(unused_unsafe)]

extern crate libc;


#[macro_use] pub mod macros;

pub mod api;
pub mod event;
pub mod html_node;
pub mod interop;


pub use api::*;
pub use event::*;
pub use html_node::*;


extern "C" {
    pub fn emscripten_asm_con(s: *const libc::c_char);
    pub fn emscripten_asm_const(s: *const libc::c_char);
    pub fn emscripten_asm_const_int(s: *const libc::c_char, ...) -> libc::c_int;
    pub fn emscripten_pause_main_loop();
    pub fn emscripten_set_main_loop(m: extern fn(), fps: libc::c_int, infinite: libc::c_int);
}


extern fn leavemebe() {
    unsafe {
        emscripten_pause_main_loop();
    }
}

pub fn spin() {
    unsafe {
        emscripten_set_main_loop(leavemebe, 0, 1);
    }
}

#[no_mangle]
pub extern "C" fn syscall(a: i32) -> i32 {
    if a == 355 {
        return 55
    }
    return -1
}

pub fn check_last_js_exception() {
    use std::ffi::CStr;
    use std::str;

    let a = js_raw! { "\
                    var exc = WEBPLATFORM.last_exc;\
                    WEBPLATFORM.last_exc = null;\
                    var str = exc == null ? \"\" : exc.toString();\
                    return allocate(intArrayFromString(str), 'i8', ALLOC_STACK);\
                " };
    unsafe {
        let error = str::from_utf8(CStr::from_ptr(a as *const libc::c_char).to_bytes()).unwrap().to_owned();
        if !error.is_empty() {
            panic!(error)
        }
    }
}
