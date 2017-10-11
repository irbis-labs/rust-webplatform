#![allow(unused_unsafe)]

extern crate libc;

use std::ffi::{CString, CStr};
use std::{mem, fmt};
use std::str;
use std::borrow::ToOwned;
use std::ops::Deref;
use std::cell::RefCell;
use std::clone::Clone;
use std::rc::Rc;
use std::collections::HashSet;
use std::char;
use std::iter::IntoIterator;

mod webplatform {
    pub use emscripten_asm_const;
    pub use emscripten_asm_const_int;
}

#[repr(C)]
struct ArenaEntryArray {
    start: libc::c_int,
    length: libc::c_int,
}

struct Arena<'a> {
    cstring: Vec<CString>,
    u8array: Vec<&'a [u8]>,
    u8array_parts: Vec<ArenaEntryArray>,
}

impl<'a> Arena<'a> {
    fn new() -> Self { Arena {
        cstring: Vec::new(),
        u8array: Vec::new(),
        u8array_parts: Vec::new()
        } }
}

trait Interop {
    fn as_int(self, _:&mut Arena) -> libc::c_int;
}

impl Interop for i32 {
    fn as_int(self, _:&mut Arena) -> libc::c_int {
        return self;
    }
}

impl<'a> Interop for &'a str {
    fn as_int(self, arena:&mut Arena) -> libc::c_int {
        let c = CString::new(self).unwrap();
        let ret = c.as_ptr() as libc::c_int;
        arena.cstring.push(c);
        return ret;
    }
}

impl<'a> Interop for &'a [u8] {
    fn as_int(self, arena:&mut Arena) -> libc::c_int {
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

#[macro_export]
macro_rules! js {
    ( ($( $x:expr ),*) $y:expr ) => {
        {
            let mut arena = Arena::new();
            const LOCAL: &'static [u8] = $y;
            unsafe { ::webplatform::emscripten_asm_const_int(&LOCAL[0] as *const _ as *const libc::c_char, $(Interop::as_int($x, &mut arena)),*) }
        }
    };
    ( $y:expr ) => {
        {
            const LOCAL: &'static [u8] = $y;
            unsafe { ::webplatform::emscripten_asm_const_int(&LOCAL[0] as *const _ as *const libc::c_char) }
        }
    };
}

extern "C" {
    pub fn emscripten_asm_con(s: *const libc::c_char);
    pub fn emscripten_asm_const(s: *const libc::c_char);
    pub fn emscripten_asm_const_int(s: *const libc::c_char, ...) -> libc::c_int;
    pub fn emscripten_pause_main_loop();
    pub fn emscripten_set_main_loop(m: extern fn(), fps: libc::c_int, infinite: libc::c_int);
}

pub struct WebSocket<'a> {
    id: libc::c_int,
    doc: *const Document<'a>,
}

pub struct HtmlNode<'a> {
    id: libc::c_int,
    doc: *const Document<'a>,
}

impl<'a> fmt::Debug for HtmlNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HtmlNode({:?})", self.id)
    }
}

impl<'a> Drop for HtmlNode<'a> {
    fn drop(&mut self) {
        println!("dropping HTML NODE {:?}", self.id);
    }
}

pub struct JSRef<'a> {
    ptr: *const HtmlNode<'a>,
}

impl<'a> fmt::Debug for JSRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "JSRef(HtmlNode({:?}))", self.id)
    }
}

impl<'a> Clone for JSRef<'a> {
    fn clone(&self) -> JSRef<'a> {
        JSRef {
            ptr: self.ptr,
        }
    }
}

impl<'a> HtmlNode<'a> {
    pub fn root_ref(&self) -> JSRef<'a> {
        JSRef {
            ptr: &*self,
        }
    }
}

impl<'a> Deref for JSRef<'a> {
    type Target = HtmlNode<'a>;

    fn deref(&self) -> &HtmlNode<'a> {
        unsafe {
            &*self.ptr
        }
    }
}

pub struct Event<'a> {
    pub target: Option<HtmlNode<'a>>
}

extern fn rust_caller<F: FnMut(Event)>(a: *const libc::c_void, docptr: *const libc::c_void, id: i32) {
    let v:&mut F = unsafe { mem::transmute(a) };
    v(Event {
        target: if id == -1 {
            None
        } else {
            Some(HtmlNode {
                id: id,
                doc: unsafe { mem::transmute(docptr) },
            })
        }
        // target: None,
    });
}

/* The _v notation introduced here is a generalization based on the
 * "viii-style" notation of dynCall. Before this is introduced, there's only
 * refs and rust_caller; both operate on FnMut(Event). The _v versions operate
 * on FnMut(), and it is expected that there'll be a _v_u8array version
 * (parting with the original coonventio pretty soon as it turned out not to
 * fit) next. */

extern fn rust_caller_v<F: FnMut()>(a: *const libc::c_void) {
    let v:&mut F = unsafe { mem::transmute(a) };
    v();
}

extern fn rust_caller_v_string<F: FnMut(String)>(a: *const libc::c_void, b: *const libc::c_char) {
    let v:&mut F = unsafe { mem::transmute(a) };
    let b = unsafe { str::from_utf8(CStr::from_ptr(b).to_bytes()).unwrap().to_owned() };
    v(b);
}

extern fn rust_caller_v_u8array<F: FnMut(&[u8])>(a: *const libc::c_void, start: *const libc::c_void, length: libc::c_int) {
    let v:&mut F = unsafe { mem::transmute(a) };
    let b:&[u8] = unsafe { std::slice::from_raw_parts(start as *const u8, length as usize) };
    v(b);
}

impl<'a> HtmlNode<'a> {
    pub fn tagname(&self) -> String {
        let a = js! { (self.id) b"\
            var str = WEBPLATFORM.rs_refs[$0].tagName.toLowerCase();\
            return allocate(intArrayFromString(str), 'i8', ALLOC_STACK);\
        \0" };
        unsafe {
            str::from_utf8(CStr::from_ptr(a as *const libc::c_char).to_bytes()).unwrap().to_owned()
        }
    }

    pub fn focus(&self) {
        js! { (self.id) b"\
            WEBPLATFORM.rs_refs[$0].focus();\
        \0" };
    }

    pub fn html_set(&self, s: &str) {
        js! { (self.id, s) b"\
            WEBPLATFORM.rs_refs[$0].innerHTML = UTF8ToString($1);\
        \0" };
    }

    pub fn html_get(&self) -> String {
        let a = js! { (self.id) b"\
            return allocate(intArrayFromString(WEBPLATFORM.rs_refs[$0].innerHTML), 'i8', ALLOC_STACK);\
        \0" };
        unsafe {
            str::from_utf8(CStr::from_ptr(a as *const libc::c_char).to_bytes()).unwrap().to_owned()
        }
    }

    pub fn class_get(&self) -> HashSet<String> {
        let a = js! { (self.id) b"\
            return allocate(intArrayFromString(WEBPLATFORM.rs_refs[$0].className), 'i8', ALLOC_STACK);\
        \0" };
        let class = unsafe {
            str::from_utf8(CStr::from_ptr(a as *const libc::c_char).to_bytes()).unwrap().to_owned()
        };
        class.trim().split(char::is_whitespace).map(|x| x.to_string()).collect()
    }

    pub fn class_add(&self, s: &str) {
        js! { (self.id, s) b"\
            WEBPLATFORM.rs_refs[$0].classList.add(UTF8ToString($1));\
        \0" };
    }

    pub fn class_toggle(&self, s: &str) {
        js! { (self.id, s) b"\
            WEBPLATFORM.rs_refs[$0].classList.toggle(UTF8ToString($1));\
        \0" };
    }

    pub fn class_remove(&self, s: &str) {
        js! { (self.id, s) b"\
            WEBPLATFORM.rs_refs[$0].classList.remove(UTF8ToString($1));\
        \0" };
    }

    pub fn parent(&self) -> Option<HtmlNode<'a>> {
        let id = js! { (self.id) b"\
            var value = WEBPLATFORM.rs_refs[$0].parentNode;\
            if (!value) {\
                return -1;\
            }\
            return WEBPLATFORM.rs_refs.push(value) - 1;\
        \0" };
        if id < 0 {
            None
        } else {
            Some(HtmlNode {
                id: id,
                doc: self.doc,
            })
        }
    }

    pub fn data_set(&self, s: &str, v: &str) {
        js! { (self.id, s, v) b"\
            WEBPLATFORM.rs_refs[$0].dataset[UTF8ToString($1)] = UTF8ToString($2);\
        \0" };
    }

    pub fn data_get(&self, s: &str) -> Option<String> {
        let a = js! { (self.id, s) b"\
            var str = WEBPLATFORM.rs_refs[$0].dataset[UTF8ToString($1)];\
            if (str == null) return -1;\
            return allocate(intArrayFromString(str), 'i8', ALLOC_STACK);\
        \0" };
        if a == -1 {
            None
        } else {
            Some(unsafe {
                str::from_utf8(CStr::from_ptr(a as *const libc::c_char).to_bytes()).unwrap().to_owned()
            })
        }
    }

    pub fn style_set_str(&self, s: &str, v: &str) {
        js! { (self.id, s, v) b"\
            WEBPLATFORM.rs_refs[$0].style[UTF8ToString($1)] = UTF8ToString($2);\
        \0" };
    }

    pub fn style_get_str(&self, s: &str) -> String {
        let a = js! { (self.id, s) b"\
            return allocate(intArrayFromString(WEBPLATFORM.rs_refs[$0].style[UTF8ToString($1)]), 'i8', ALLOC_STACK);\
        \0" };
        unsafe {
            str::from_utf8(CStr::from_ptr(a as *const libc::c_char).to_bytes()).unwrap().to_owned()
        }
    }

    pub fn prop_set_i32(&self, s: &str, v: i32) {
        js! { (self.id, s, v) b"\
            WEBPLATFORM.rs_refs[$0][UTF8ToString($1)] = $2;\
        \0" };
    }

    pub fn prop_set_str(&self, s: &str, v: &str) {
        js! { (self.id, s, v) b"\
            WEBPLATFORM.rs_refs[$0][UTF8ToString($1)] = UTF8ToString($2);\
        \0" };
    }

    pub fn prop_get_i32(&self, s: &str) -> i32 {
        return js! { (self.id, s) b"\
            return Number(WEBPLATFORM.rs_refs[$0][UTF8ToString($1)])\
        \0" };
    }

    pub fn prop_get_str(&self, s: &str) -> String {
        let a = js! { (self.id, s) b"\
            var a = allocate(intArrayFromString(WEBPLATFORM.rs_refs[$0][UTF8ToString($1)]), 'i8', ALLOC_STACK); console.log(WEBPLATFORM.rs_refs[$0]); return a;\
        \0" };
        unsafe {
            str::from_utf8(CStr::from_ptr(a as *const libc::c_char).to_bytes()).unwrap().to_owned()
        }
    }

    pub fn append(&self, s: &HtmlNode) {
        js! { (self.id, s.id) b"\
            WEBPLATFORM.rs_refs[$0].appendChild(WEBPLATFORM.rs_refs[$1]);\
        \0" };
    }

    pub fn html_append(&self, s: &str) {
        js! { (self.id, s) b"\
            WEBPLATFORM.rs_refs[$0].insertAdjacentHTML('beforeEnd', UTF8ToString($1));\
        \0" };
    }

    pub fn html_prepend(&self, s: &str) {
        js! { (self.id, s) b"\
            WEBPLATFORM.rs_refs[$0].insertAdjacentHTML('afterBegin', UTF8ToString($1));\
        \0" };
    }

    pub fn on<F: FnMut(Event) + 'a>(&self, s: &str, f: F) {
        unsafe {
            let b = Box::new(f);
            let a = &*b as *const _;
            js! { (self.id, s, a as *const libc::c_void,
                rust_caller::<F> as *const libc::c_void,
                self.doc as *const libc::c_void)
                b"\
                WEBPLATFORM.rs_refs[$0].addEventListener(UTF8ToString($1), function (e) {\
                    Runtime.dynCall('viii', $3, [$2, $4, e.target ? WEBPLATFORM.rs_refs.push(e.target) - 1 : -1]);\
                }, false);\
            \0" };
            (&*self.doc).refs.borrow_mut().push(b);
        }
    }

    pub fn captured_on<F: FnMut(Event) + 'a>(&self, s: &str, f: F) {
        unsafe {
            let b = Box::new(f);
            let a = &*b as *const _;
            js! { (self.id, s, a as *const libc::c_void,
                rust_caller::<F> as *const libc::c_void,
                self.doc as *const libc::c_void)
                b"\
                WEBPLATFORM.rs_refs[$0].addEventListener(UTF8ToString($1), function (e) {\
                    Runtime.dynCall('viii', $3, [$2, $4, e.target ? WEBPLATFORM.rs_refs.push(e.target) - 1 : -1]);\
                }, true);\
            \0" };
            (&*self.doc).refs.borrow_mut().push(b);
        }
    }

    pub fn remove_self(&self) {
        js! { (self.id) b"\
            var s = WEBPLATFORM.rs_refs[$0];\
            s.parentNode.removeChild(s);\
        \0" };
    }
}

pub fn alert(s: &str) {
    js! { (s) b"\
        alert(UTF8ToString($0));\
    \0" };
}

pub struct Document<'a> {
    refs: Rc<RefCell<Vec<Box<FnMut(Event<'a>) + 'a>>>>,
    refs_v: Rc<RefCell<Vec<Box<FnMut() + 'a>>>>,
    refs_v_u8array: Rc<RefCell<Vec<Box<FnMut(&[u8]) + 'a>>>>,
    refs_v_string: Rc<RefCell<Vec<Box<FnMut(String) + 'a>>>>,
}

impl<'a> WebSocket<'a> {
    /* can and should we make this FnOnce? we'd need to remove the listener,
     * and should only do that if js guarantees this only gets called once, or
     * otherwise there might be uses of calling this more than once */
    pub fn addEventListener_open<F: FnMut() + 'a>(&self, f: F) {
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
            (&*self.doc).refs_v.borrow_mut().push(b);
        }
    }

    pub fn addEventListener_message_string<F: FnMut(String) + 'a>(&self, f: F) {
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
            (&*self.doc).refs_v_string.borrow_mut().push(b);
        }
    }

    pub fn addEventListener_message_binary<F: FnMut(&[u8]) + 'a>(&self, f: F) {
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
            (&*self.doc).refs_v_u8array.borrow_mut().push(b);
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

impl<'a> Document<'a> {
    pub fn websocket_create<'b>(&'b self, url: &str) -> Option<WebSocket<'a>> {
        let id = js! { (url) b"\
            var value = new WebSocket(UTF8ToString($0));\
            if (!value) {\
                return -1;\
            }\
            value.binaryType = 'arraybuffer';\
            return WEBPLATFORM.rs_refs.push(value) - 1;\
        \0" };

        if id < 0 {
            None
        } else {
            Some(WebSocket {
                id: id,
                doc: &*self,
            })
        }
    }

    pub fn element_create<'b>(&'b self, s: &str) -> Option<HtmlNode<'a>> {
        let id = js! { (s) b"\
            var value = document.createElement(UTF8ToString($0));\
            if (!value) {\
                return -1;\
            }\
            return WEBPLATFORM.rs_refs.push(value) - 1;\
        \0" };

        if id < 0 {
            None
        } else {
            Some(HtmlNode {
                id: id,
                doc: &*self,
            })
        }
    }

    pub fn location_hash_get(&self) -> String {
        let a = js! { b"\
            return allocate(intArrayFromString(window.location.hash), 'i8', ALLOC_STACK);\
        \0" };
        unsafe {
            str::from_utf8(CStr::from_ptr(a as *const libc::c_char).to_bytes()).unwrap().to_owned()
        }
    }

    pub fn on<F: FnMut(Event) + 'a>(&self, s: &str, f: F) {
        unsafe {
            let b = Box::new(f);
            let a = &*b as *const _;
            js! { (0, s, a as *const libc::c_void,
                rust_caller::<F> as *const libc::c_void,
                &*self as *const _ as *const libc::c_void)
                b"\
                window.addEventListener(UTF8ToString($1), function (e) {\
                    Runtime.dynCall('viii', $3, [$2, $4, e.target ? WEBPLATFORM.rs_refs.push(e.target) - 1 : -1]);\
                }, false);\
            \0" };
            self.refs.borrow_mut().push(b);
        }
    }

    pub fn element_query<'b>(&'b self, s: &str) -> Option<HtmlNode<'a>> {
        let id = js! { (s) b"\
            var value = document.querySelector(UTF8ToString($0));\
            if (!value) {\
                return -1;\
            }\
            return WEBPLATFORM.rs_refs.push(value) - 1;\
        \0" };

        if id < 0 {
            None
        } else {
            Some(HtmlNode {
                id: id,
                doc: self,
            })
        }
    }
}

pub struct LocalStorageInterface;

pub struct LocalStorageIterator {
    index: i32,
}

impl LocalStorageInterface {
    pub fn len(&self) -> i32 {
        js! { b"\
            return window.localStorage.length;\
        \0" }
    }

    pub fn clear(&self) {
        js! { b"\
            window.localStorage.clear();\
        \0" };
    }

    pub fn remove(&self, s: &str) {
        js! { (s) b"\
            window.localStorage.removeItem(UTF8ToString($0));\
        \0" };
    }

    pub fn set(&self, s: &str, v: &str) {
        js! { (s, v) b"\
            window.localStorage.setItem(UTF8ToString($0), UTF8ToString($1));\
        \0" };
    }

    pub fn get(&self, name: &str) -> Option<String> {
        let a = js! { (name) b"\
            var str = window.localStorage.getItem(UTF8ToString($0));\
            if (str == null) {\
                return -1;\
            }\
            return allocate(intArrayFromString(str), 'i8', ALLOC_STACK);\
        \0" };
        if a == -1 {
            None
        } else {
            Some(unsafe {
                str::from_utf8(CStr::from_ptr(a as *const libc::c_char).to_bytes()).unwrap().to_owned()
            })
        }
    }

    pub fn key(&self, index: i32) -> String {
        let a = js! { (index) b"\
            var key = window.localStorage.key($0);\
            return allocate(intArrayFromString(str), 'i8', ALLOC_STACK);\
        \0" };
        unsafe {
            str::from_utf8(CStr::from_ptr(a as *const libc::c_char).to_bytes()).unwrap().to_owned()
        }
    }
}

impl IntoIterator for LocalStorageInterface {
    type Item = String;
    type IntoIter = LocalStorageIterator;

    fn into_iter(self) -> LocalStorageIterator {
        LocalStorageIterator { index: 0 }
    }
}

impl Iterator for LocalStorageIterator {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        if self.index >= LocalStorage.len() {
            None
        } else {
            LocalStorage.get(&LocalStorage.key(self.index))
        }
    }
}

#[allow(non_upper_case_globals)]
pub const LocalStorage: LocalStorageInterface = LocalStorageInterface;

pub fn init<'a>() -> Document<'a> {
    js! { b"\
        window.WEBPLATFORM || (window.WEBPLATFORM = {\
            rs_refs: [],\
        });\
    \0" };
    Document {
        refs: Rc::new(RefCell::new(Vec::new())),
        refs_v: Rc::new(RefCell::new(Vec::new())),
        refs_v_u8array: Rc::new(RefCell::new(Vec::new())),
        refs_v_string: Rc::new(RefCell::new(Vec::new())),
    }
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
