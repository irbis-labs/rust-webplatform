#[macro_export]
macro_rules! js_raw {
    ( ($( $x:expr ),*) $y:expr ) => {
        {
            let mut arena = $crate::interop::Arena::new();
            const LOCAL: &'static str = concat!($y, "\0");
            unsafe {
                $crate::emscripten_asm_const_int(
                        LOCAL as *const _ as *const ::libc::c_char,
                        $($crate::interop::Interop::as_int($x, &mut arena)),
                        *
                )
            }
        }
    };
    ( $y:expr ) => {
        {
            const LOCAL: &'static str = concat!($y, "\0");
            unsafe {
                $crate::emscripten_asm_const_int(LOCAL as *const _ as *const ::libc::c_char)
            }
        }
    };
}

#[macro_export]
macro_rules! js_guarded {
    ( ($( $x:expr ),*) $y:expr ) => {
        {
            let mut arena = $crate::interop::Arena::new();
            const LOCAL: &'static str = concat!(
                    "try { ",
                    $y,
                    " } catch (e) { ",
                    "WEBPLATFORM.last_exc = e; return -173642426;",
                    " }\0"
            );
            let ret = unsafe {
                $crate::emscripten_asm_const_int(
                        LOCAL as *const _ as *const ::libc::c_char,
                        $($crate::interop::Interop::as_int($x, &mut arena)),
                        *
                )
            };
            if ret == -173642426 {
                $crate::check_last_js_exception();
            }

            ret
        }
    };
    ( $y:expr ) => {
        {
            const LOCAL: &'static str = concat!(
                    "try { ",
                    $y,
                    " } catch (e) { ",
                    "WEBPLATFORM.last_exc = e; return -173642426;",
                    " }\0"
            );
            let ret = unsafe {
                $crate::emscripten_asm_const_int(LOCAL as *const _ as *const ::libc::c_char)
            };
            if ret == -173642426 {
                $crate::check_last_js_exception();
            }

            ret
        }
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_js_raw_simple() {
        let a = js_raw! { "return 0;" };
        assert_eq!(a, 0);
    }

    #[test]
    fn test_js_raw_value() {
        let a = js_raw! { (42) "return $0;" };
        assert_eq!(a, 42);
    }

    #[test]
    fn test_js_guarded_simple() {
        let a = js_guarded! { "return 0;" };
        assert_eq!(a, 0);
    }

    #[test]
    fn test_js_guarded_value() {
        let a = js_guarded! { (42) "return $0;" };
        assert_eq!(a, 42);
    }

    #[test]
    fn test_js_guarded_exception() {
        use ::std::panic;

        let result = panic::catch_unwind(|| {
            js_guarded! { "throw 'exception';" };
        });
        assert!(result.is_err());
    }
}

