//! copy from RustSBI

#![allow(dead_code)]

pub struct Clint {
    base: usize,
}

impl Clint {
    pub fn new(base: *mut u8) -> Clint {
        Clint {
            base: base as usize,
        }
    }

    pub fn get_mtime(&self) -> u64 {
        unsafe {
            let base = self.base as *mut u8;
            core::ptr::read_volatile(base.add(0xbff8) as *mut u64)
        }
    }

    pub fn set_timer(&mut self, hart_id: usize, instant: u64) {
        unsafe {
            let base = self.base as *mut u8;
            core::ptr::write_volatile((base.offset(0x4000) as *mut u64).add(hart_id), instant);
        }
    }

    pub fn send_soft(&mut self, hart_id: usize) {
        unsafe {
            let base = self.base as *mut u8;
            core::ptr::write_volatile((base as *mut u32).add(hart_id), 1);
        }
    }

    pub fn clear_soft(&mut self, hart_id: usize) {
        unsafe {
            let base = self.base as *mut u8;
            core::ptr::write_volatile((base as *mut u32).add(hart_id), 0);
        }
    }
}
