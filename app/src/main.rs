#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;
use core::ptr::read_volatile;
use core::ptr::write_volatile;
use core::fmt::Write;
use core::fmt::Result;
use core::arch::asm;
use core::ptr;

const UART: usize = 0x10000000;
const THR_EMPTY_AND_LINE_IDLE: u8 = 1 << 6;

fn ns16550(addr: usize, msg: &str) {
    let base_addr = addr as *mut u8;
    unsafe {
        let status_addr = base_addr.offset(5);
        for c in msg.bytes() {
            while read_volatile(status_addr) & THR_EMPTY_AND_LINE_IDLE == 0 { }
            write_volatile(base_addr, c);
        }
    }
}

struct Serial;

impl Write for Serial {
    fn write_str(&mut self, msg: &str) -> Result {
        ns16550(UART, msg);
        Ok(())
    }
}

#[inline]
fn cmp_addr<T>(p1: *const T, p2: *const T) -> bool {
    let i: usize;
    unsafe {
        asm!(
            "sub {a}, {a}, {b}",
            "snez {c}, {a}",
            a = in(reg) p1,
            b = in(reg) p2,
            c = out(reg) i,
        );
    }
    i == 0
}

#[entry]
fn main() -> ! {

    extern "C" {
        // Boundaries of the .bss section
        static _ebss: u32;
        static _sbss: u32;

        // Boundaries of the .data section
        static _edata: u32;
        static _sdata: u32;
        // Initial values of the .data section (stored in Flash)
        static _sidata: u32;
    }

    let mut cons = Serial;
    let is_eq = unsafe { cmp_addr(ptr::addr_of!(_sdata), ptr::addr_of!(_sidata)) };
    write!(cons, "{}", is_eq).unwrap();

    loop { }
}
