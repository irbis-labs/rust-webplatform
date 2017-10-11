pub mod document;
pub mod local_storage;
pub mod websocket;

pub use self::document::*;
pub use self::local_storage::*;
pub use self::websocket::*;


pub fn alert(s: &str) {
    js_guarded! { (s) "\
        alert(UTF8ToString($0));\
    " };
}
