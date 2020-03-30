#![no_std]

// TODO - feature gates for
// - NJ_USE_LIBC
// - NJ_CHROMA_FILTER

use core::convert::TryFrom;
use core::slice;

pub mod capi;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Error {
    /// Not a JPEG file
    NotJPeg,
    /// Unsupported format
    Unsupported,
    /// Out of memory
    OutOfMemory,
    /// Internal error
    Internal,
    /// Syntax error
    Syntax,
}

#[derive(Debug)]
pub struct ImageInfo {
    width: usize,
    height: usize,
    /// RGB888 if true, otherwise GRAY8
    is_color: bool,
    image: &'static [u8],
}

/// NOTE: there be globals in use here, see nanojpeg.c, `static nj_context_t nj;`
static mut IS_INIT: bool = false;

#[derive(Debug)]
pub struct NanoJPeg {}

impl NanoJPeg {
    pub fn init() -> Self {
        unsafe {
            assert_eq!(IS_INIT, false, "nanojpeg global(s) already initialized");
            IS_INIT = true;
            capi::njInit();
        }

        NanoJPeg {}
    }

    pub fn deinit(self) {
        unsafe {
            assert_eq!(IS_INIT, true, "nanojpeg global(s) not initialized yet");
            IS_INIT = false;
            capi::njDone();
        }
    }

    pub fn decode(&mut self, image: &[u8]) -> Result<ImageInfo, Error> {
        let c_result = unsafe { capi::njDecode(image.as_ptr() as *const _, image.len() as _) };
        let result: Result<(), Error> = c_result.into();
        if result.is_ok() {
            let image_size = get_size()?;
            let image_ptr = unsafe { capi::njGetImage() } as *const u8;
            if image_ptr.is_null() || image_size == 0 {
                return Err(Error::Internal);
            }
            Ok(ImageInfo {
                width: get_width()?,
                height: get_height()?,
                is_color: unsafe { capi::njIsColor() } != 0,
                image: unsafe { slice::from_raw_parts(image_ptr, image_size) },
            })
        } else {
            Err(result.unwrap_err())
        }
    }
}

fn get_width() -> Result<usize, Error> {
    let w = unsafe { capi::njGetWidth() };
    usize::try_from(w).map_err(|_| Error::Internal)
}

fn get_height() -> Result<usize, Error> {
    let h = unsafe { capi::njGetHeight() };
    usize::try_from(h).map_err(|_| Error::Internal)
}

fn get_size() -> Result<usize, Error> {
    let s = unsafe { capi::njGetImageSize() };
    usize::try_from(s).map_err(|_| Error::Internal)
}

impl From<capi::nj_result_t> for Result<(), Error> {
    fn from(r: capi::nj_result_t) -> Result<(), Error> {
        use capi::nj_result_t::*;
        match r {
            NJ_OK => Ok(()),
            NJ_NO_JPEG => Err(Error::NotJPeg),
            NJ_UNSUPPORTED => Err(Error::Unsupported),
            NJ_OUT_OF_MEM => Err(Error::OutOfMemory),
            NJ_INTERNAL_ERR => Err(Error::Internal),
            NJ_SYNTAX_ERROR => Err(Error::Syntax),
            _ => panic!("Got an unhandled nj_result_t ({:?})", r),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // From:
    // https://upload.wikimedia.org/wikipedia/en/f/f6/Sample_0.JPEG
    const FILE_BYTES: &'static [u8; 2628376] = include_bytes!("../test_data/sample.jpeg");

    #[test]
    fn decode() {
        let mut decoder = NanoJPeg::init();
        let info = decoder.decode(&FILE_BYTES[..]).unwrap();
        assert_eq!(info.width, 1203);
        assert_eq!(info.height, 1593);
        assert_eq!(info.is_color, true);
        assert_eq!(info.image.len(), 1203 * 1593 * 3);
        decoder.deinit();
    }
}
