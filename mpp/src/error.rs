use mpp_sys::MPP_RET;
use std::result::Result as StdResult;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("implement")]
    EndOfStream,
    #[error("implement")]
    Base,
    #[error("implement")]
    BufferIsFull,
    #[error("implement")]
    DisplayIsFull,
    #[error("implement")]
    FatalThreadError,
    #[error("implement")]
    InitializationError,
    #[error("implement")]
    MPP_RET_MPP_ERR_LIST_STREAM,
    #[error("implement")]
    MPP_RET_MPP_ERR_MALLOC,
    #[error("implement")]
    MPP_RET_MPP_ERR_NOMEM,
    #[error("implement")]
    MPP_RET_MPP_ERR_NULL_PTR,
    #[error("implement")]
    MPP_RET_MPP_ERR_OPEN_FILE,
    #[error("implement")]
    MPP_RET_MPP_ERR_PERM,
    #[error("implement")]
    MPP_RET_MPP_ERR_PROTOL,
    #[error("implement")]
    MPP_RET_MPP_ERR_READ_BIT,
    #[error("implement")]
    MPP_RET_MPP_ERR_STREAM,
    #[error("implement")]
    MPP_RET_MPP_ERR_TIMEOUT,
    #[error("implement")]
    MPP_RET_MPP_ERR_UNKNOW,
    #[error("implement")]
    MPP_RET_MPP_ERR_VALUE,
    #[error("implement")]
    MPP_RET_MPP_ERR_VPUHW,
    #[error("implement")]
    MPP_RET_MPP_ERR_VPU_CODEC_INIT,
    #[error("implement")]
    MPP_RET_MPP_FAIL_SPLIT_FRAME,
    #[error("implement")]
    MPP_RET_MPP_NOK,

    #[error("unkown error code {0}")]
    UnkownErrorCode(i32),
}

pub type Result<T> = StdResult<T, Error>;

impl Error {
    pub(crate) fn from_mpp_ret(error_code: MPP_RET) -> Result<()> {
        match error_code {
            mpp_sys::MPP_RET_MPP_EOS_STREAM_REACHED => Err(Self::EndOfStream),
            mpp_sys::MPP_RET_MPP_ERR_BASE => Err(Self::Base),
            mpp_sys::MPP_RET_MPP_ERR_BUFFER_FULL => Err(Self::BufferIsFull),
            mpp_sys::MPP_RET_MPP_ERR_DISPLAY_FULL => Err(Self::DisplayIsFull),
            mpp_sys::MPP_RET_MPP_ERR_FATAL_THREAD => Err(Self::FatalThreadError),
            mpp_sys::MPP_RET_MPP_ERR_INIT => Err(Self::InitializationError),
            mpp_sys::MPP_RET_MPP_ERR_LIST_STREAM => Err(Self::MPP_RET_MPP_ERR_LIST_STREAM),
            mpp_sys::MPP_RET_MPP_ERR_MALLOC => Err(Self::MPP_RET_MPP_ERR_MALLOC),
            mpp_sys::MPP_RET_MPP_ERR_NOMEM => Err(Self::MPP_RET_MPP_ERR_NOMEM),
            mpp_sys::MPP_RET_MPP_ERR_NULL_PTR => Err(Self::MPP_RET_MPP_ERR_NULL_PTR),
            mpp_sys::MPP_RET_MPP_ERR_OPEN_FILE => Err(Self::MPP_RET_MPP_ERR_OPEN_FILE),
            mpp_sys::MPP_RET_MPP_ERR_PERM => Err(Self::MPP_RET_MPP_ERR_PERM),
            mpp_sys::MPP_RET_MPP_ERR_PROTOL => Err(Self::MPP_RET_MPP_ERR_PROTOL),
            mpp_sys::MPP_RET_MPP_ERR_READ_BIT => Err(Self::MPP_RET_MPP_ERR_READ_BIT),
            mpp_sys::MPP_RET_MPP_ERR_STREAM => Err(Self::MPP_RET_MPP_ERR_STREAM),
            mpp_sys::MPP_RET_MPP_ERR_TIMEOUT => Err(Self::MPP_RET_MPP_ERR_TIMEOUT),
            mpp_sys::MPP_RET_MPP_ERR_UNKNOW => Err(Self::MPP_RET_MPP_ERR_UNKNOW),
            mpp_sys::MPP_RET_MPP_ERR_VALUE => Err(Self::MPP_RET_MPP_ERR_VALUE),
            mpp_sys::MPP_RET_MPP_ERR_VPUHW => Err(Self::MPP_RET_MPP_ERR_VPUHW),
            mpp_sys::MPP_RET_MPP_ERR_VPU_CODEC_INIT => Err(Self::MPP_RET_MPP_ERR_VPU_CODEC_INIT),
            mpp_sys::MPP_RET_MPP_FAIL_SPLIT_FRAME => Err(Self::MPP_RET_MPP_FAIL_SPLIT_FRAME),
            mpp_sys::MPP_RET_MPP_NOK => Err(Self::MPP_RET_MPP_NOK),
            mpp_sys::MPP_RET_MPP_OK => Ok(()),
            mpp_sys::MPP_RET_MPP_SUCCESS => Ok(()),
            e => Err(Self::UnkownErrorCode(e)),
        }
    }
}
