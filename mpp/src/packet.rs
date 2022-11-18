pub struct MppPacket(*mut mpp_sys::MppPacket, Vec<ffi::c_void>);
use std::{ffi, mem::MaybeUninit, ptr};

pub use crate::error::{Error, Result};

impl Drop for MppPacket {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                Error::from_mpp_ret(mpp_sys::mpp_packet_deinit(self.0))
                    .expect("failed to deinit packet");
            }
        }
    }
}

impl MppPacket {
    pub fn new() -> Result<Self> {
        let mut packet: MaybeUninit<*mut ffi::c_void> = std::mem::MaybeUninit::zeroed();
        let mut buffer = Vec::with_capacity(1024);
        let mpp_error =
            unsafe { mpp_sys::mpp_packet_init(packet.as_mut_ptr(), buffer.as_mut_ptr(), 1024) };
        Error::from_mpp_ret(mpp_error).map(|_| {
            let packet = unsafe { packet.assume_init_mut() };
            debug_assert!(!packet.is_null());
            Self(packet, buffer)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::MppPacket;

    #[test]
    fn create_packet() {
        let _ = MppPacket::new().expect("should create packet");
    }
}
