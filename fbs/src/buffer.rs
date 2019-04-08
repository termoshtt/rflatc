//! Read/Write into on-memory FlatBuffers
//!
//! Links
//! ------
//! - [flatcc/FlatBuffers Binary Format](https://github.com/dvidelabs/flatcc/blob/master/doc/binary-format.md)
//! - [FlatBuffers internals](https://google.github.io/flatbuffers/flatbuffers_internals.html)

use crate::error::*;
use std::{
    alloc, ffi, fs,
    io::{self, Read},
    mem,
    ptr::{self, NonNull},
};

/// Handler of a heap-allocated raw buffer
#[derive(Debug)]
pub struct Buffer {
    raw: Box<RawBuffer>,
}

/// raw entire buffer
#[repr(C, align(32))]
#[derive(Debug)]
struct RawBuffer {
    table_offset: u32,
    file_identifier: [u8], // identifier must be '\0'-terminated as FlatBuffers defines,
                           // and following bytes are managed by another struct
}

#[repr(C, align(32))]
#[derive(Debug)]
struct Table {
    vtable_offset: i32,
    data: [u8],
}

#[repr(C, align(16))]
#[derive(Debug)]
struct VTable {
    vtable_length: u16,
    table_length: u16,
    offsets: [u16],
}

impl RawBuffer {
    /// Allocate a 32-bit aligned buffer on heap
    unsafe fn alloc(len: usize) -> NonNull<Self> {
        let layout = alloc::Layout::from_size_align(len, 32).expect("Fail to set memory layout");
        let ptr = alloc::alloc(layout);
        let fat_ptr = mem::transmute::<(*mut u8, usize), *mut Self>((ptr, len - 4));
        NonNull::new(fat_ptr).expect("Cannot allocate")
    }
}

impl Buffer {
    /// Create a new empty (non-initialized) buffer
    ///
    /// ## Safety
    /// The contents of this buffer is not initialized. Be sure to write loaded/received binary.
    pub unsafe fn new(len: usize) -> Self {
        let ptr = RawBuffer::alloc(len);
        Self {
            raw: Box::from_raw(ptr.as_ptr()).into(),
        }
    }

    /// Create new buffer, and copy the contents of slice
    pub fn copy_from_slice(bytes: &[u8]) -> Self {
        let len = bytes.len();
        let mut buf = unsafe { Self::new(len) };
        let ptr: *mut RawBuffer = buf.raw.as_mut();
        // this never overlap since buf is newly allocated
        unsafe { ptr::copy_nonoverlapping(bytes.as_ptr(), ptr as *mut u8, len) };
        buf
    }

    /// Create new buffer, and copy from the file
    pub fn from_file(filename: &str) -> io::Result<Self> {
        let mut f = fs::File::open(filename)?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;
        Ok(Self::copy_from_slice(&buf))
    }

    /// Get format identifier name
    pub fn file_identifier(&self) -> Result<&str> {
        let cstr = ffi::CStr::from_bytes_with_nul(&self.raw.file_identifier)?;
        Ok(cstr.to_str()?)
    }

    /// Get nth member of the table
    ///
    /// Safety
    /// -------
    /// The value should be broken if incorrect type `T` is specified
    pub unsafe fn get_sized<T>(&self, n: usize) -> Result<&T> {
        let (vtable, table) = self.get_tables();
        let offset = vtable.offsets[n];
        if offset == 0 {
            return Err(Error::DeprecatedMember {});
        }
        let cur = table as *const Table as *const u8;
        let cur = cur.offset(offset as isize);
        Ok(&*(cur as *const T))
    }

    /// Get nth member string on the table
    pub fn get_str(&self, n: usize) -> Result<&str> {
        let cstr = unsafe {
            let (vtable, table) = self.get_tables();
            let offset = vtable.offsets[n];
            if offset == 0 {
                return Err(Error::DeprecatedMember {});
            }
            // head of table
            let cur = table as *const Table as *const u8;
            // seek the offset to string-table
            let cur = cur.offset(offset as isize);
            let offset_to_string = *(cur as *const u32);
            // move to string-table
            let cur = cur.offset(offset_to_string as isize);
            // read the length, but do not use since CStr seeks '\0'
            let _length = *cur;
            // skip length
            let cur = cur.offset(4);
            // read as CStr
            ffi::CStr::from_ptr(cur as *const _)
        };
        Ok(cstr.to_str()?)
    }

    /// Read table and vtable on the buffer
    ///
    /// TODO
    /// -----
    /// - Revise to portable way. Thin ptr to fat ptr transmute is not assured
    /// - Validate the table/vtable length input if these go in the buffer memory
    unsafe fn get_tables(&self) -> (&VTable, &Table) {
        let ptr = &*self.raw as *const RawBuffer as *const u8;
        let table_ptr = ptr.offset(self.raw.table_offset as isize);

        // Read vtable offset from Table header
        let vtable_offset = {
            let table_fat_ptr = mem::transmute::<(*const u8, usize), *const Table>((
                table_ptr,
                0, /* DUMMY size (since the length of trailing length of `data` is unknown here) */
            ));
            (*table_fat_ptr).vtable_offset
        };

        let vtable_ptr = (table_ptr as *const u8).offset(-vtable_offset as isize);

        // Read vtable header
        let (vtable_length, table_length) = {
            let vtable_fat_ptr = mem::transmute::<(*const u8, usize), *const VTable>((
                vtable_ptr, 0, /* DUMMY size */
            ));
            (
                (*vtable_fat_ptr).vtable_length,
                (*vtable_fat_ptr).table_length,
            )
        };

        let vtable: &VTable = &*mem::transmute::<(*const u8, usize), *const VTable>((
            vtable_ptr,
            (vtable_length / 2 - 2) as usize, // vtable_length is a length as a &[u8]
                                              // - divide by 2 to match &[u16]
                                              // - sub 2 for vtable_length(u16) and table_length(u16)
        ));

        let table: &Table = &*mem::transmute::<(*const u8, usize), *const Table>((
            table_ptr,
            (table_length - 4) as usize,
        ));

        (vtable, table)
    }
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

    #[test]
    fn read_example_buffer() {
        let fb = Buffer::from_file("example.bin").unwrap();
        let (vtable, table) = unsafe { fb.get_tables() };

        assert_eq!(vtable.vtable_length, 12);
        assert_eq!(vtable.table_length, 12);
        assert_eq!(vtable.offsets, [8, 0, 4, 10]);

        assert_eq!(table.vtable_offset, -24);

        unsafe {
            // FooBar.meal
            assert_eq!(fb.get_sized::<i8>(0).unwrap(), &42_i8);
            // FooBar.density (deprecated)
            assert!(fb.get_sized::<i64>(1).is_err());
            // FooBar.hight
            assert_eq!(fb.get_sized::<i16>(3).unwrap(), &-8000_i16);
        }
        // FooBar.say
        assert_eq!(fb.get_str(2).unwrap(), "hello");
    }
}
