use internal_prelude::*;


pub struct WebSocket<'a> {
    id: libc::c_int,
    doc: *const Document<'a>,
}


impl<'a> WebSocket<'a> {
    pub fn new(id: libc::c_int, doc: *const Document<'a>) -> Self {
        WebSocket { id, doc }
    }

    /* can and should we make this FnOnce? we'd need to remove the listener,
     * and should only do that if js guarantees this only gets called once, or
     * otherwise there might be uses of calling this more than once */
    pub fn add_event_listener_open<F: FnMut() + 'a>(&self, f: F) {
        unsafe {
            let b = Box::new(f);
            let a = &*b as *const _;
            js! { (self.id, a as *const libc::c_void,
                rust_caller_v::<F> as *const libc::c_void)
                b"\
                WEBPLATFORM.rs_refs[$0].addEventListener('open', function (e) {\
                    Runtime.dynCall('vi', $2, [$1]);\
                }, false);\
            \0" };
            (&*self.doc).push_ref_v(b);
//            (&*self.doc).refs_v.borrow_mut().push(b);
        }
    }

    pub fn add_event_listener_message_string<F: FnMut(String) + 'a>(&self, f: F) {
        unsafe {
            let b = Box::new(f);
            let a = &*b as *const _;
            js! { (self.id, a as *const libc::c_void,
                rust_caller_v_string::<F> as *const libc::c_void)
                b"\
                WEBPLATFORM.rs_refs[$0].addEventListener('message', function (e) {\
                    if (typeof e.data != 'string') return;\
                    Runtime.dynCall('vii', $2, [$1, allocate(intArrayFromString(e.data), 'i8', ALLOC_STACK)]);\
                }, false);\
            \0" };
            (&*self.doc).push_ref_v_string(b);
//            (&*self.doc).refs_v_string.borrow_mut().push(b);
        }
    }

    pub fn add_event_listener_message_binary<F: FnMut(&[u8]) + 'a>(&self, f: F) {
        unsafe {
            let b = Box::new(f);
            let a = &*b as *const _;
            // BIG FIXME this leaks memory, and i don't want to malloc there in the first place but just pass a pointer to the buffer
            js! { (self.id, a as *const libc::c_void,
                rust_caller_v_u8array::<F> as *const libc::c_void)
                b"\
                WEBPLATFORM.rs_refs[$0].addEventListener('message', function (e) {\
                    if (typeof e.data != 'object') return;\
                    var buf = Module._malloc(e.data.byteLength);\
                    Module.writeArrayToMemory(new Int8Array(e.data), buf);\
                    Runtime.dynCall('viii', $2, [$1, buf, e.data.byteLength]);\
                }, false);\
            \0" };
            (&*self.doc).push_ref_v_u8array(b);
//            (&*self.doc).refs_v_u8array.borrow_mut().push(b);
        }
    }

    pub fn send(&self, data: &str) {
        js! { (self.id, data) b"\
            WEBPLATFORM.rs_refs[$0].send(UTF8ToString($1));\
        \0" };
    }

    pub fn send_binary(&self, data: &[u8]) {
        /* FIXME first three lines should go into a U8ToSlice function like UTF8ToString */
        js! { (self.id, data) b"\
            var start = HEAPU32[$1 / 4];\
            var length = HEAPU32[$1 / 4 + 1];\
            var sliced = HEAP8.slice(start, start + length * 1);\
            WEBPLATFORM.rs_refs[$0].send(sliced);\
        \0" };
    }

    pub fn close(&self, data: &str) {
        js! { (self.id, data) b"\
            WEBPLATFORM.rs_refs[$0].close();\
        \0" };
    }
}
