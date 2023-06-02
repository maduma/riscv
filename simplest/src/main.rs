#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;
use core::ptr;
use core::fmt::Write;
use core::fmt::Result;

global_asm!(include_str!("asm.s"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

const UART: usize = 0x10000000;
const THR_EMPTY_AND_LINE_IDLE: u8 = 1 << 6;

fn ns16550(addr: usize, msg: &str) {
    let base_addr = addr as *mut u8;
    unsafe {
        let status_addr = base_addr.offset(5);
        for c in msg.bytes() {
            while ptr::read_volatile(status_addr) & THR_EMPTY_AND_LINE_IDLE == 0 { }
            ptr::write_volatile(base_addr, c);
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
pub extern "C" fn _start_rust() -> ! {

    ns16550(UART, "Hello world!\n");
    write!(Serial, "Hello, {}!\n", "Steve").unwrap();

    loop {}
}
