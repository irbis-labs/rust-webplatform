use std::ffi::CStr;
use std::mem;
use std::slice;
use std::str;
use libc;
use ::html_node::*;


pub(crate) extern fn rust_caller<F: FnMut(Event)>(a: *const libc::c_void, docptr: *const libc::c_void, id: i32) {
    let v:&mut F = unsafe { mem::transmute(a) };
    v(Event {
        target: if id == -1 {
            None
        } else {
            Some(HtmlNode::new( id, unsafe { mem::transmute(docptr) } ))
        }
    });
}

/* The _v notation introduced here is a generalization based on the
 * "viii-style" notation of dynCall. Before this is introduced, there's only
 * refs and rust_caller; both operate on FnMut(Event). The _v versions operate
 * on FnMut(), and it is expected that there'll be a _v_u8array version
 * (parting with the original convention pretty soon as it turned out not to
 * fit) next. */

pub(crate) extern fn rust_caller_v<F: FnMut()>(a: *const libc::c_void) {
    let v:&mut F = unsafe { mem::transmute(a) };
    v();
}

pub(crate) extern fn rust_caller_v_string<F: FnMut(String)>(a: *const libc::c_void, b: *const libc::c_char) {
    let v:&mut F = unsafe { mem::transmute(a) };
    let b = unsafe { str::from_utf8(CStr::from_ptr(b).to_bytes()).unwrap().to_owned() };
    v(b);
}

pub(crate) extern fn rust_caller_v_u8array<F: FnMut(&[u8])>(a: *const libc::c_void, start: *const libc::c_void, length: libc::c_int) {
    let v:&mut F = unsafe { mem::transmute(a) };
    let b:&[u8] = unsafe { slice::from_raw_parts(start as *const u8, length as usize) };
    v(b);
}

// =================================================================================================

pub struct Event<'a> {
    pub target: Option<HtmlNode<'a>>
}
