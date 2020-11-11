//! RBL Executable

#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![deny(warnings)]

#[macro_use]
mod serial;
mod clint;
mod trap;

use core::panic::PanicInfo;

#[no_mangle]
pub unsafe extern "C" fn boot_first_hart(hartid: usize, dtb: usize) -> ! {
    extern "C" {
        static mut sbss: u32;
        static mut ebss: u32;
    }
    r0::zero_bss(&mut sbss, &mut ebss);

    println!("Welcome to Rust Boot Loader!");

    // enter supervisor mode
    let entry: usize = 0x80010000;
    println!(
        "Going to supervisor mode: entry={:#x}, hartid={}, dtb={:#x}",
        entry, hartid, dtb
    );

    // delegate interrupts
    let interrupts = 1 << 1 | 1 << 5 | 1 << 9; // SSIP | STIP | SEIP
    llvm_asm!("csrw mideleg, $0": : "r"(interrupts) : : "volatile" );

    // delegate exceptions below:
    // instruction address misaligned
    // breakpoint
    // environment call from u-mode
    // instruction page fault
    // load page fault
    // store/amo page fault
    let exceptions = 1 << 0 | 1 << 3 | 1 << 8 | 1 << 12 | 1 << 13 | 1 << 15;
    llvm_asm!("csrw medeleg, $0": : "r"(exceptions) : : "volatile" );

    // MSIE | MTIE
    llvm_asm!("csrw mie, $0": : "r"(1 << 3 | 1 << 7) : : "volatile" );

    let mut mstatus: usize;
    llvm_asm!("csrr $0, mstatus": "=r"(mstatus) :  : : "volatile" );
    mstatus |= !(1 << 13 | 1 << 14); // disable fs
    mstatus |= !(1 << 7); // no mpie
    mstatus |= 1 << 11; // mpp = s
    mstatus |= 1 << 3; // mie
    llvm_asm!("csrw mstatus, $0": : "r"(mstatus) : : "volatile" );

    // set supervisor entry point
    llvm_asm!("csrw mepc, $0": : "r"(entry) : : "volatile" );

    // clear satp
    llvm_asm!("csrw satp, zero": : : : "volatile" );

    // enter supervisor mode
    // pass three args into a0-a2
    let mask: usize = 1;
    llvm_asm!("mret": : "{x10}"(hartid) "{x11}"(dtb) "{x12}"(mask): : "volatile" );
    unreachable!();
}

global_asm!(include_str!("trap.S"));
global_asm!(include_str!("boot.S"));

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n{}", info);
    loop {}
}
