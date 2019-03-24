//! Runtime for rflatc
//!
//! Buffer
//! ------
//! Address space   |------------------------------------------------------------|
//! Buffer          |xxxx|IDENTIFIER\0-------------------------------------------|
//! DataTable                             |xxxx|xxxx|----------------------------|
//!   table_offset  o-------------------->

use std::ffi::CStr;

/// Handler for entire buffer
#[repr(C, align(32))]
#[derive(Debug)]
pub struct Buffer {
    root_table_offset: u32,
    file_identifier: [u8], // identifier must be '\0'-terminated as FlatBuffers defines,
                           // and following bytes are managed by another struct
}

impl Buffer {
    pub fn new(bytes: &[u8]) -> &Self {
        let ptr = bytes.as_ptr();
        let len = bytes.len();
        assert_eq!(ptr as usize % 32, 0);
        unsafe { &*std::mem::transmute::<(*const u8, usize), *const Self>((ptr, len - 4)) }
    }

    pub fn get_file_identifier(&self) -> &CStr {
        unimplemented!()
    }

    pub fn get_data_table(&self) -> &DataTable {
        let inc = self.root_table_offset as usize / 4;
        DataTable::new(&self.file_identifier[inc..])
    }
}

#[repr(C, align(32))]
#[derive(Debug)]
pub struct DataTable {
    vtable_offset: u32,
    field_offset: u32,
    data: [u8],
}

impl DataTable {
    pub fn new(bytes: &[u8]) -> &Self {
        let ptr = bytes.as_ptr();
        let len = bytes.len();
        assert_eq!(ptr as usize % 32, 0); // Force 32-bit alignment
        unsafe { &*std::mem::transmute::<(*const u8, usize), *const Self>((ptr, len - 4 * 2)) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let buf = vec![
            0x00, 0x01, 0x00, 0x00, 'N' as u8, 'O' as u8, 'O' as u8, 'B' as u8, '\0' as u8,
        ];
        let b = Buffer::new(&buf);
        assert_eq!(b.file_identifier.len(), 5)
    }
}
