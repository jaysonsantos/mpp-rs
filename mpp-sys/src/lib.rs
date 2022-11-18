#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// Haaalp
#![allow(deref_nullptr)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod test {
    #[test]
    fn test_mpp_compat_show() {
        unsafe {
            super::mpp_set_log_level(super::MPP_LOG_VERBOSE as i32);
            super::mpp_compat_show();
        }
    }
}
