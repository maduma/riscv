#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;
use core::fmt::Write;
use core::fmt::Result;
use core::ptr::read_volatile;
use core::ptr::write_volatile;

const UART: usize = 0x10000000;
const THR_EMPTY_AND_LINE_IDLE: u8 = 1 << 6;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle]
fn trap_handler() -> ! {
    loop{}
}

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

#[no_mangle]
#[link_section = ".init"]
pub extern "C" fn _start() -> ! {

    extern "C" {
        static  _stack_start: u32;
    }

    type FnPtr = fn() -> !;
    let th: FnPtr = trap_handler;

    unsafe{
        asm!("csrw mtvec, {}",in(reg) th);

        let sp = &_stack_start;
        asm!("mv sp, {}", in(reg) sp);
    }

    let msg ="\
        Hello World!\n\
        How are you men!\n\
        this\tis\tmy\tname\n\
        one\ttime\tis\tthere\n\
        ";

    ns16550(UART, msg);

    loop {}
}
