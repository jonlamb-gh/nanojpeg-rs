#[allow(non_camel_case_types)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum nj_result_t {
    NJ_OK = 0,
    NJ_NO_JPEG = 1,
    NJ_UNSUPPORTED = 2,
    NJ_OUT_OF_MEM = 3,
    NJ_INTERNAL_ERR = 4,
    NJ_SYNTAX_ERROR = 5,
    NJ_FINISHED = 6,
}

extern "C" {
    pub fn njInit();
}
extern "C" {
    pub fn njDecode(jpeg: *const core::ffi::c_void, size: i32) -> nj_result_t;
}
extern "C" {
    pub fn njGetWidth() -> i32;
}
extern "C" {
    pub fn njGetHeight() -> i32;
}
extern "C" {
    pub fn njIsColor() -> i32;
}
extern "C" {
    pub fn njGetImage() -> *mut u8;
}
extern "C" {
    pub fn njGetImageSize() -> i32;
}
extern "C" {
    pub fn njDone();
}
