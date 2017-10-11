use internal_prelude::*;


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

// =================================================================================================

pub struct Event<'a> {
    pub target: Option<HtmlNode<'a>>
}


