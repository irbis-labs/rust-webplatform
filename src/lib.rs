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


pub(crate) mod internal_prelude {
    pub(crate) use std::cell::RefCell;
    pub(crate) use std::char;
    pub(crate) use std::clone::Clone;
    pub(crate) use std::ffi::{CString, CStr};
    pub(crate) use std::fmt;
    pub(crate) use std::mem;
    pub(crate) use std::ops::Deref;
    pub(crate) use std::rc::Rc;
    pub(crate) use std::slice;
    pub(crate) use std::str;

    pub(crate) use libc;

    pub(crate) use api::*;
    pub(crate) use event::*;
    pub(crate) use html_node::*;
    pub(crate) use interop::*;
}

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
