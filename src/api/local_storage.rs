use std::iter::IntoIterator;

use internal_prelude::*;


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
