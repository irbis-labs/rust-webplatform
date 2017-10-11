use std::collections::HashSet;

use internal_prelude::*;


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

impl<'a> Deref for JSRef<'a> {
    type Target = HtmlNode<'a>;

    fn deref(&self) -> &HtmlNode<'a> {
        unsafe {
            &*self.ptr
        }
    }
}

// =================================================================================================

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

impl<'a> HtmlNode<'a> {
    pub fn new(id: libc::c_int, doc: *const Document<'a>) -> Self {
        HtmlNode { id, doc }
    }

    pub fn root_ref(&self) -> JSRef<'a> {
        JSRef {
            ptr: &*self,
        }
    }
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
            (&*self.doc).push_ref(b);
//            (&*self.doc).refs.borrow_mut().push(b);
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
            (&*self.doc).push_ref(b);
//            (&*self.doc).refs.borrow_mut().push(b);
        }
    }

    pub fn remove_self(&self) {
        js! { (self.id) b"\
            var s = WEBPLATFORM.rs_refs[$0];\
            s.parentNode.removeChild(s);\
        \0" };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn with_html(html: &str, action: fn(&Document)) {
        let document = init();
        let body = document.element_query("body").unwrap();
        let elem = document.element_create("div").unwrap();
        elem.html_append(html);
        body.append(&elem);
        action(&document);
        elem.remove_self();
    }

    #[test]
    fn test_body_exists() {
        let document = init();
        assert!(document.element_query("body").is_some());
    }

    #[test]
    fn test_query() {
        with_html(
            r#"<div class="my_class"></div>"#,
            |doc| {
                assert!(doc.element_query(".my_class").is_some());
            }
        )
    }

    #[test]
    fn test_tagname() {
        with_html(
            r#"<div class="my_class"></div>"#,
            |doc| {
                assert_eq!(doc.element_query(".my_class").unwrap().tagname(), "div");
            }
        )
    }

    #[test]
    fn test_focus() {
        // TODO needs is_focused
    }

    #[test]
    fn test_html_set_get() {
        with_html(
            r#"<div class="my_class"></div>"#,
            |doc| {
                let elem = doc.element_query(".my_class").unwrap();
                elem.html_set("123");
                assert_eq!(elem.html_get(), "123");
            }
        )
    }

    #[test]
    fn test_class_get() {
        use std::iter::FromIterator;

        with_html(
            r#"<div class="my_class my_class2"></div>"#,
            |doc| {
                let expect = ["my_class", "my_class2"];
                let found = doc.element_query(".my_class").unwrap().class_get();
                assert_eq!(found, HashSet::from_iter(expect.iter().map(|it| it.to_string())));
            }
        )
    }

    #[test]
    fn test_class_add() {
        with_html(
            r#"<div class="my_class"></div>"#,
            |doc| {
                doc.element_query(".my_class").unwrap().class_add("added_class");
                assert!(doc.element_query(".added_class").is_some());
            }
        )
    }

    #[test]
    fn test_class_toggle() {
        with_html(
            r#"<div class="my_class"></div>"#,
            |doc| {
                let elem = doc.element_query(".my_class").unwrap();
                elem.class_toggle("my_class");
                assert!(doc.element_query(".my_class").is_none());
                elem.class_toggle("my_class");
                assert!(doc.element_query(".my_class").is_some());
            }
        )
    }

    #[test]
    fn test_class_remove() {
        with_html(
            r#"<div class="my_class"></div>"#,
            |doc| {
                doc.element_query(".my_class").unwrap().class_remove("my_class");
                assert!(doc.element_query(".my_class").is_none());
            }
        )
    }

    #[test]
    fn test_parent() {
        with_html(
            r#"<div class="parent"><div class="child"></div></div>"#,
            |doc| {
                let child = doc.element_query(".child").unwrap();
                let parent_class = child.parent().unwrap().class_get().into_iter().next().unwrap();
                assert_eq!(parent_class, "parent");
            }
        )
    }

    #[test]
    fn test_data_set_get() {
        with_html(
            r#"<div class="my_class"></div>"#,
            |doc| {
                let elem = doc.element_query(".my_class").unwrap();
                elem.data_set("key", "value");
                assert_eq!(elem.data_get("key"), Some("value".to_string()));
            }
        )
    }

    #[test]
    fn test_style_set_get() {
        with_html(
            r#"<div class="my_class"></div>"#,
            |doc| {
                let elem = doc.element_query(".my_class").unwrap();
                elem.style_set_str("color", "red");
                assert_eq!(elem.style_get_str("color"), "red".to_string()   );
            }
        )
    }

    #[test]
    fn test_prop_set_get() {
        with_html(
            r#"<div class="my_class"></div>"#,
            |doc| {
                let elem = doc.element_query(".my_class").unwrap();
                elem.prop_set_str("id", "value");
                assert_eq!(elem.prop_get_str("id"), "value".to_string()   );
            }
        )
    }

    #[test]
    fn test_append() {
        with_html(
            r#"<div class="my_class"></div>"#,
            |doc| {
                let elem1 = doc.element_query(".my_class").unwrap();
                let elem2 = doc.element_create("div").unwrap();
                elem1.append(&elem2);
                let parent_class = elem2.parent().unwrap().class_get().into_iter().next().unwrap();
                assert_eq!(parent_class, "my_class");
            }
        )
    }

    #[test]
    fn test_remove_self() {
        with_html(
            r#"<div class="my_class"></div>"#,
            |doc| {
                doc.element_query(".my_class").unwrap().remove_self();
                assert!(doc.element_query(".my_class").is_none());
            }
        )
    }

    #[test]
    fn test_html_append() {
        with_html(
            r#"<div class="my_class">aaa</div>"#,
            |doc| {
                let elem = doc.element_query(".my_class").unwrap();
                elem.html_append("bbb");
                assert_eq!(elem.html_get(), "aaabbb");
            }
        )
    }

    #[test]
    fn test_html_prepend() {
        with_html(
            r#"<div class="my_class">aaa</div>"#,
            |doc| {
                let elem = doc.element_query(".my_class").unwrap();
                elem.html_prepend("bbb");
                assert_eq!(elem.html_get(), "bbbaaa");
            }
        )
    }

    // TODO test evens
}
