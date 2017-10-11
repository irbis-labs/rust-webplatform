use internal_prelude::*;


pub trait Interop {
    fn as_int(self, _:&mut Vec<CString>) -> libc::c_int;
}

impl Interop for i32 {
    fn as_int(self, _:&mut Vec<CString>) -> libc::c_int {
        return self;
    }
}

impl<'a> Interop for &'a str {
    fn as_int(self, arena:&mut Vec<CString>) -> libc::c_int {
        let c = CString::new(self).unwrap();
        let ret = c.as_ptr() as libc::c_int;
        arena.push(c);
        return ret;
    }
}

impl<'a> Interop for *const libc::c_void {
    fn as_int(self, _:&mut Vec<CString>) -> libc::c_int {
        return self as libc::c_int;
    }
}
