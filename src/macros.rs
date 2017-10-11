#[macro_export]
macro_rules! js {
    ( ($( $x:expr ),*) $y:expr ) => {
        {
            use internal_prelude::*;
            use ::emscripten_asm_const_int;

            let mut arena:Vec<CString> = Vec::new();
            const LOCAL: &'static [u8] = $y;
            unsafe { emscripten_asm_const_int(&LOCAL[0] as *const _ as *const libc::c_char, $(Interop::as_int($x, &mut arena)),*) }
        }
    };
    ( $y:expr ) => {
        {
            use internal_prelude::*;
            use ::emscripten_asm_const_int;

            const LOCAL: &'static [u8] = $y;
            unsafe { emscripten_asm_const_int(&LOCAL[0] as *const _ as *const libc::c_char) }
        }
    };
}
