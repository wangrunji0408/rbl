use core::fmt::{Arguments, Error, Write};
use volatile::*;

const UART_ADDR: usize = 0x10000000;

bitflags::bitflags! {
    struct LineStatusRegister: u8 {
        // Tranmit hold register empty
        // i.e. can send
        const THRE = 0x20;

        // Receiver data ready
        // i.e. can read
        const DR = 0x01;
    }
}

#[repr(C)]
struct Uart16550 {
    dll: ReadWrite<u8>,
    dlm: WriteOnly<u8>,
    fcr: WriteOnly<u8>,
    lcr: WriteOnly<u8>,
    mcr: WriteOnly<u8>,
    lsr: ReadOnly<LineStatusRegister>,
    msr: ReadOnly<u8>,
}

impl Uart16550 {
    fn putc(&mut self, ch: u8) {
        while !self.lsr.read().contains(LineStatusRegister::THRE) {
            core::sync::atomic::spin_loop_hint();
        }
        self.dll.write(ch);
    }

    fn getc(&mut self) -> Option<u8> {
        if self.lsr.read().contains(LineStatusRegister::DR) {
            Some(self.dll.read())
        } else {
            None
        }
    }
}

impl Write for Uart16550 {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        for ch in s.bytes() {
            self.putc(ch);
        }
        Ok(())
    }
}

pub fn print(args: Arguments) {
    let serial = unsafe { &mut *(UART_ADDR as *mut Uart16550) };
    serial.write_fmt(args).unwrap();
}

pub fn getchar() -> Option<u8> {
    let serial = unsafe { &mut *(UART_ADDR as *mut Uart16550) };
    serial.getc()
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::serial::print(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}
