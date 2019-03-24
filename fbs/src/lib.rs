//! Runtime for rflatc

use std::ffi::CStr;

/// Handler for entire buffer
#[repr(C, align(32))]
pub struct Buffer {
    root_table_offset: u32,
    file_identifier: [u8], // identifier must be '\0'-terminated as FlatBuffers defines,
                           // and following bytes are managed by another struct
}

#[repr(C, align(32))]
pub struct DataTable {
    vtable_offset: u32,
    field_offset: u32,
    data: [u8],
}

impl Buffer {
    pub fn get_file_identifier(&self) -> &CStr {
        unimplemented!()
    }

    pub fn get_data_table(&self) -> &DataTable {
        let p = self as *const _ as *const u8; // Drop size of file_identifier
        let inc = self.root_table_offset / 8;
        unsafe {
            let table_ptr = p.offset(inc as isize);
            let size = 0_usize;
            // **CAUTION** Cast thin pointer to fat pointer
            let fat_ptr =
                std::mem::transmute::<(*const u8, usize), *const DataTable>((table_ptr, size));

            &*fat_ptr
        }
    }
}
