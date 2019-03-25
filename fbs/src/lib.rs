//! Runtime for rflatc
//!
//! RawBuffer
//! ------
//! Address space   |------------------------------------------------------------|
//! RawBuffer       |xxxx|IDENTIFIER\0-------------------------------------------|
//! DataTable                             |xxxx|xxxx|----------------------------|
//!   table_offset  o-------------------->

use std::{
    alloc, ffi, mem,
    pin::Pin,
    ptr::{self, NonNull},
};

pub mod error;

pub type Result<T> = std::result::Result<T, crate::error::Error>;

/// Entire buffer
#[repr(C, align(32))]
#[derive(Debug)]
struct RawBuffer {
    root_table_offset: u32,
    file_identifier: [u8], // identifier must be '\0'-terminated as FlatBuffers defines,
                           // and following bytes are managed by another struct
}

#[derive(Debug)]
pub struct Buffer {
    raw: Pin<Box<RawBuffer>>,
}

impl RawBuffer {
    /// Allocate a buffer on heap
    unsafe fn alloc(len: usize) -> NonNull<Self> {
        let layout = alloc::Layout::from_size_align(len, 32).expect("Fail to set memory layout");
        let ptr = alloc::alloc(layout);
        let fat_ptr = mem::transmute::<(*mut u8, usize), *mut Self>((ptr, len - 4));
        NonNull::new(fat_ptr).expect("Cannot allocate")
    }

    /// Reinterpret existing buffer without reallocation
    #[allow(unused)]
    unsafe fn transmute(ptr: *mut u8, len: usize) -> *mut Self {
        mem::transmute::<(*mut u8, usize), *mut Self>((ptr, len - 4))
    }
}

impl Buffer {
    /// Create a new empty (non-initialized) buffer
    ///
    /// ## Safety
    /// The containts of this buffer is not initialized. Be sure to write loaded/recived binary.
    pub unsafe fn new(len: usize) -> Self {
        let ptr = RawBuffer::alloc(len);
        Self {
            raw: Box::from_raw(ptr.as_ptr()).into(),
        }
    }

    /// Create new buffer, and copy the containts of slice
    pub fn copy_from_slice(bytes: &[u8]) -> Self {
        let len = bytes.len();
        let mut buf = unsafe { Self::new(len) };
        let ptr: *mut RawBuffer = buf.raw.as_mut().get_mut();
        unsafe { ptr::copy_nonoverlapping(bytes.as_ptr(), ptr as *mut u8, len) };
        buf
    }

    pub fn file_identifier(&self) -> Result<&str> {
        let cstr = ffi::CStr::from_bytes_with_nul(&self.raw.file_identifier)?;
        Ok(cstr.to_str()?)
    }
}

#[repr(C, align(32))]
#[derive(Debug)]
pub struct RawTable {
    vtable_offset: u32,
    field_offset: u32,
    data: [u8],
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_HEADER: &'static [u8] = &[
        0x00, 0x01, 0x00, 0x00, 'N' as u8, 'O' as u8, 'O' as u8, 'B' as u8, '\0' as u8,
    ];

    #[test]
    fn test_new() {
        let buf = Buffer::copy_from_slice(&TEST_HEADER);
        assert_eq!(buf.raw.file_identifier.len(), 5);
    }

    #[test]
    fn test_identifier() {
        let buf = Buffer::copy_from_slice(&TEST_HEADER);
        assert_eq!(buf.file_identifier().unwrap(), "NOOB");
    }
}
