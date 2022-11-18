// pub mod buffer;
pub mod api;
pub mod error;
pub mod packet;

use std::{ffi, ptr};

pub use crate::error::{Error, Result};
pub struct MppCompat(*mut mpp_sys::MppCompat);

pub enum MppCompatId {
    Butt = mpp_sys::MppCompatId_e_MPP_COMPAT_BUTT as isize,
    EncAsyncInput = mpp_sys::MppCompatId_e_MPP_COMPAT_ENC_ASYNC_INPUT as isize,
    IncFbcBufSize = mpp_sys::MppCompatId_e_MPP_COMPAT_INC_FBC_BUF_SIZE as isize,
}

impl MppCompat {
    pub fn new() -> Self {
        Self(unsafe { mpp_sys::mpp_compat_query() })
    }
}
