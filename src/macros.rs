#[macro_export]
macro_rules! js {
    ( ($( $x:expr ),*) $y:expr ) => {
        {
            use internal_prelude::*;

            let mut arena = Arena::new();
            const LOCAL: &'static [u8] = $y;
            unsafe { ::emscripten_asm_const_int(&LOCAL[0] as *const _ as *const libc::c_char, $(Interop::as_int($x, &mut arena)),*) }
        }
    };
    ( $y:expr ) => {
        {
            use internal_prelude::*;

            const LOCAL: &'static [u8] = $y;
            unsafe { ::emscripten_asm_const_int(&LOCAL[0] as *const _ as *const libc::c_char) }
        }
    };
}