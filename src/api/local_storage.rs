use std::iter::IntoIterator;

use std::ffi::CStr;
use std::str;
use libc;


pub struct LocalStorageInterface;

pub struct LocalStorageIterator {
    index: i32,
}

impl LocalStorageInterface {
    pub fn len(&self) -> i32 {
        js_guarded! { "\
            return window.localStorage.length;\
        " }
    }

    pub fn clear(&self) {
        js_guarded! { "\
            window.localStorage.clear();\
        " };
    }

    pub fn remove(&self, s: &str) {
        js_guarded! { (s) "\
            window.localStorage.removeItem(UTF8ToString($0));\
        " };
    }

    pub fn set(&self, s: &str, v: &str) {
        js_guarded! { (s, v) "\
            window.localStorage.setItem(UTF8ToString($0), UTF8ToString($1));\
        " };
    }

    pub fn get(&self, name: &str) -> Option<String> {
        let a = js_guarded! { (name) "\
            var str = window.localStorage.getItem(UTF8ToString($0));\
            if (str == null) {\
                return -1;\
            }\
            return allocate(intArrayFromString(str), 'i8', ALLOC_STACK);\
        " };
        if a == -1 {
            None
        } else {
            Some(unsafe {
                str::from_utf8(CStr::from_ptr(a as *const libc::c_char).to_bytes()).unwrap().to_owned()
            })
        }
    }

    pub fn key(&self, index: i32) -> String {
        let a = js_guarded! { (index) "\
            var key = window.localStorage.key($0);\
            return allocate(intArrayFromString(str), 'i8', ALLOC_STACK);\
        " };
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
