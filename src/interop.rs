use std::ffi::CString;
use std::str;
use libc;


#[repr(C)]
pub(crate) struct ArenaEntryArray {
    start: libc::c_int,
    length: libc::c_int,
}

pub(crate) struct Arena<'a> {
    cstring: Vec<CString>,
    _u8array: Vec<&'a [u8]>,
    u8array_parts: Vec<ArenaEntryArray>,
}

impl<'a> Arena<'a> {
    pub(crate) fn new() -> Self { Arena {
        cstring: Vec::new(),
        _u8array: Vec::new(),
        u8array_parts: Vec::new()
    } }
}

pub(crate) trait Interop {
    fn as_int(self, _: &mut Arena) -> libc::c_int;
}

impl Interop for i32 {
    fn as_int(self, _: &mut Arena) -> libc::c_int {
        return self;
    }
}

impl<'a> Interop for &'a str {
    fn as_int(self, arena: &mut Arena) -> libc::c_int {
        let c = CString::new(self).unwrap();
        let ret = c.as_ptr() as libc::c_int;
        arena.cstring.push(c);
        return ret;
    }
}

impl<'a> Interop for &'a [u8] {
    fn as_int(self, arena: &mut Arena) -> libc::c_int {
        let parts = ArenaEntryArray { start: self.as_ptr() as libc::c_int, length: self.len() as libc::c_int };
        let partsptr: *const ArenaEntryArray = &parts;

        arena.u8array_parts.push(parts);
        // BIG FIXME: This is trusting that there is another reference to self
        // around somewhere that outlives the arena. What else could we do --
        // require arguments to be owned? To be boxed/RC'd? Use another arena
        // mechanism that doesn't require storing in a vec?
        //         arena.u8array.push(&self);

        partsptr as libc::c_int
    }
}

impl<'a> Interop for *const libc::c_void {
    fn as_int(self, _:&mut Arena) -> libc::c_int {
        return self as libc::c_int;
    }
}
