#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;
use core::ptr::read_volatile;
use core::ptr::write_volatile;
use core::fmt::Write;
use core::fmt::Result;
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

    unsafe {
        let a = &_sdata as *const u32;
	let b = &_sidata as *const u32;
	let c = &_sdata;
        let d = &_sidata;
        write!(cons, "_sdata {:p}\n", &_sdata).unwrap();
        write!(cons, "_edata {:p}\n", &_edata).unwrap();
        write!(cons, "_sidata {:p}\n", &_sidata).unwrap();
        write!(cons, "_sbss {:p}\n", &_sbss).unwrap();
        write!(cons, "_ebss {:p}\n", &_ebss).unwrap();
        write!(cons, "{:?}\n", ptr::addr_of!(_sidata)).unwrap();
        write!(cons, "{:?} {:?} {}\n", ptr::addr_of!(_sdata), ptr::addr_of!(_sidata), ptr::addr_of!(_sdata) == ptr::addr_of!(_sidata)).unwrap();
        write!(cons, "{:?} {:?}\n", a, b).unwrap();
        write!(cons, "same1? {}\n", a == b).unwrap();
        write!(cons, "same2? {}\n", ptr::eq(c, d)).unwrap();
        write!(cons, "same2? {}\n", ptr::eq(&_sdata, &_sidata)).unwrap();
        write!(cons, "same3? {}\n", ptr::eq(&_sdata as *const _, &_sidata as *const _)).unwrap();
    }

    loop { }
}
