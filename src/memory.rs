use crate::address_parser::AddressResolver;

pub trait MemoryReaderWriter: AddressResolver {
    fn read_buf(&self, addr: usize, buffer: &mut [u8]);
    fn can_read(&self, addr: usize) -> bool;
}

pub struct NullMemoryReader;

impl MemoryReaderWriter for NullMemoryReader {
    fn read_buf(&self, _addr: usize, _buffer: &mut [u8]) {
        // do nothing
    }

    fn can_read(&self, _addr: usize) -> bool {
        false
    }
}

impl AddressResolver for NullMemoryReader {
    fn module_symbol_to_address(&self, module_name: &str) -> Option<isize> {
        if module_name == obfstr!("sample.dll") {
            return Some(0x1000);
        }
        None
    }

    fn dereference(&self, _address: usize) -> Option<isize> {
        return Some(0);
    }
}

pub trait MemoryState: MemoryReaderWriter + AddressResolver {}

impl MemoryState for NullMemoryReader {}
