use internal_prelude::*;


pub fn init<'a>() -> Document<'a> {
    js! { b"\
        window.WEBPLATFORM || (window.WEBPLATFORM = {\
            rs_refs: [],\
        });\
    \0" };
    Document {
        refs: Rc::new(RefCell::new(Vec::new())),
    }
}

pub struct Document<'a> {
    refs: Rc<RefCell<Vec<Box<FnMut(Event<'a>) + 'a>>>>,
}

impl<'a> Document<'a> {
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
            Some(HtmlNode::new( id, &*self ))
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

    pub fn push_ref(&self, value: Box<FnMut(Event<'a>) + 'a>) {
        self.refs.borrow_mut().push(value);
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
            Some(HtmlNode::new( id, &*self ))
        }
    }
}
