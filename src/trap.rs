use riscv::register::{
    mcause::{self, *},
    mepc, mhartid, mie, mip, mtval,
};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TrapFrame {
    x0: usize,  // zero
    x1: usize,  // ra
    x2: usize,  // sp
    x3: usize,  // gp
    x4: usize,  // tp
    x5: usize,  // t0
    x6: usize,  // t1
    x7: usize,  // t2
    x8: usize,  // s0/fp
    x9: usize,  // s1
    x10: usize, // a0
    x11: usize, // a1
    x12: usize, // a2
    x13: usize, // a3
    x14: usize, // a4
    x15: usize, // a5
    x16: usize, // a6
    x17: usize, // a7
    x18: usize, // s2
    x19: usize, // s3
    x20: usize, // s4
    x21: usize, // s5
    x22: usize, // s6
    x23: usize, // s7
    x24: usize, // s8
    x25: usize, // s9
    x26: usize, // s10
    x27: usize, // s11
    x28: usize, // t3
    x29: usize, // t4
    x30: usize, // t5
    x31: usize, // t6
}

/// Trap handler
/// Return the new epc
#[no_mangle]
pub extern "C" fn trap_handler(tf: &mut TrapFrame) -> usize {
    let mcause = mcause::read().cause();
    let mtval = mtval::read();
    let mut mepc = mepc::read();

    match mcause {
        Trap::Exception(Exception::SupervisorEnvCall) => {
            // arguments
            let which = tf.x17; // a7
            let arg1 = tf.x10; // a0
            let arg2 = tf.x11; // a1
            let arg3 = tf.x12; // a2

            let ret = match which {
                0 => sbi_set_timer(((arg2 as u64) << 32) | arg1 as u64),
                1 => sbi_console_putchar(arg1),
                2 => sbi_console_getchar(),
                3 => sbi_clear_ipi(),
                4 => sbi_send_ipi(arg1),
                5 => sbi_remote_fence_i(arg1),
                6 => sbi_remote_sfence_vma(arg1, arg2, arg3),
                _ => (-38isize as usize), // NOSYS
            };

            // skip ecall instruction
            mepc += 4;
            // return code at a0
            tf.x10 = ret;
        }
        Trap::Interrupt(Interrupt::MachineTimer) => unsafe {
            mie::clear_mtimer();
            mip::set_stimer();
        },
        _ => unimplemented!(
            "trap: mcause={:?}, mepc={:#x}, mtval={:#x}\n{:#x?}",
            mcause,
            mepc,
            mtval,
            tf
        ),
    }
    mepc
}

fn sbi_set_timer(time: u64) -> usize {
    const CLINT_ADDR: usize = 0x2000000;
    let mut clint = crate::clint::Clint::new(CLINT_ADDR as _);
    clint.set_timer(mhartid::read(), time);
    unsafe {
        mie::set_mtimer();
        mip::clear_stimer();
    }
    0
}

fn sbi_console_putchar(ch: usize) -> usize {
    print!("{}", ch as u8 as char);
    0
}

fn sbi_console_getchar() -> usize {
    match crate::serial::getchar() {
        Some(c) => c as usize,
        None => usize::max_value(),
    }
}

fn sbi_clear_ipi() -> usize {
    // do nothing
    println!("sbi: clear ipi");
    0
}

fn sbi_send_ipi(hart_mask: usize) -> usize {
    // do nothing
    println!("sbi: send ipi hart mask {}", hart_mask);
    0
}

fn sbi_remote_fence_i(hart_mask: usize) -> usize {
    // do nothing
    println!("sbi: remote fence i hart mask {}", hart_mask);
    0
}

fn sbi_remote_sfence_vma(hart_mask: usize, start: usize, size: usize) -> usize {
    // do nothing
    println!(
        "sbi: remote sfence vma hart mask {} start {:#X} size {:#X}",
        hart_mask, start, size
    );
    0
}
