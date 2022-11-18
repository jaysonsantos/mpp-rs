pub struct MppApi(*mut mpp_sys::MppApi, mpp_sys::MppCtx);
use std::{ffi, mem::MaybeUninit, ptr};

pub use crate::error::{Error, Result};

impl Drop for MppApi {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                Error::from_mpp_ret(mpp_sys::mpp_destroy(self.1)).expect("failed to deinit packet");
            }
        }
    }
}

impl MppApi {
    pub fn new() -> Result<Self> {
        let mut api: MaybeUninit<*mut mpp_sys::MppApi_t> = std::mem::MaybeUninit::zeroed();
        let mut context: MaybeUninit<mpp_sys::MppCtx> = std::mem::MaybeUninit::zeroed();
        let mpp_error = unsafe { mpp_sys::mpp_create(context.as_mut_ptr(), api.as_mut_ptr()) };
        Error::from_mpp_ret(mpp_error).map(|_| {
            let api = unsafe { api.assume_init() };
            let context = unsafe { context.assume_init() };
            debug_assert!(!api.is_null());
            debug_assert!(!context.is_null());

            Self(api, context)
        })
    }
    pub fn intialize(&self) -> Result<()> {
        Error::from_mpp_ret(unsafe {
            let mut needs_split: mpp_sys::RK_U32 = 1;
            (*self.0).control.expect("no control function?")(
                self.1,
                mpp_sys::MpiCmd_MPP_DEC_SET_PARSER_SPLIT_MODE,
                &mut needs_split as *mut _ as *mut ffi::c_void,
            )
        })?;
        Error::from_mpp_ret(unsafe {
            mpp_sys::mpp_init(
                self.1,
                mpp_sys::MppCtxType_MPP_CTX_DEC,
                mpp_sys::MppCodingType_MPP_VIDEO_CodingMPEG2,
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::MppApi;

    #[test]
    fn create_packet() {
        let api = MppApi::new().expect("should create api");
        api.intialize().expect("should initialize");
    }
}
