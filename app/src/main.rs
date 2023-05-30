#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;
use core::ptr::read_volatile;
use core::ptr::write_volatile;
use core::fmt::Write;
use core::fmt::Result;

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

#[entry]
fn main() -> ! {

    let _msg ="\
        Hello World!\n\
        How are you men!\n\
        this\tis\tmy\tname\n\
        one\ttime\tis\tthere\n\
        ";

    let mut cons = Serial;
    write!(cons, "Hello {}!\n", "Stephane").unwrap();
    extern "C" {
        // Boundaries of the .bss section
        static mut _ebss: u32;
        static mut _sbss: u32;

        // Boundaries of the .data section
        static mut _edata: u32;
        static mut _sdata: u32;

        // Initial values of the .data section (stored in Flash)
        static _sidata: u32;
    }
    unsafe {
        write!(cons, "_sdata {:p}\n", &_sdata).unwrap();
        write!(cons, "_edata {:p}\n", &_edata).unwrap();
        write!(cons, "_sidata {:p}\n", &_sidata).unwrap();
        write!(cons, "_sbss {:p}\n", &_sbss).unwrap();
        write!(cons, "_ebss {:p}\n", &_ebss).unwrap();
    }

    loop { }
}
