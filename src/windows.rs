extern crate kernel32;
extern crate winapi;

use self::kernel32::{ReadProcessMemory, OpenProcess};
use self::winapi::winnt::HANDLE;

use libc::c_int;

use ReadsMemory;

struct MemReader {
  pid: u32,
  handle: HANDLE
}

impl MemReader {
  fn new(pid: u32) -> Result<MemReader, c_int> {
    let handle = unsafe { OpenProcess(0x0010, false, pid) };
    if handle.is_null() {
      return Err(1);
    }
    Ok(MemReader {
      pid: pid,
      handle: handle
    })
  }
}

impl ReadsMemory for MemReader {
  fn read_bytes(&self, address: usize, n: usize) -> Result<Vec<u8>, c_int> {
    let mut buffer: Vec<u8> = vec![0; n];
    let mut read: u64 = ::std::mem::uninitialized();
    let res = unsafe {
      ReadProcessMemory(self.handle,
        address as *const _,
        &mut buffer as *mut _,
        n,
        &mut read as *mut _)
    };
    if !res {
      return Err(1);
    }
    if read != n {
      return Err(2);
    }
    Ok(buffer)
  }
}

impl ProvidesSlices for MemReader {
  fn address_slice<'a>(&'a self, start: usize, end: usize) -> MemorySlice<'a> {
    MemorySlice::new(self, start, end)
  }

  fn address_slice_len<'a>(&'a self, start: usize, n: usize) -> MemorySlice<'a> {
    MemorySlice::new(self, start, start + n)
  }
}
