#![no_std]
#![no_main]

#[allow(dead_code)]
const UART_VIRT_ADDR: usize = 0x10000000;
const UART_SHAKTI_ADDR: usize = 0x11300;

mod serial;

use core::arch::global_asm;
use core::panic::PanicInfo;
use serial::UART_SHAKTI;
use core::fmt::Write;
use core::arch::asm;

global_asm!(include_str!("init.s"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle]
pub extern "C" fn start_rust() -> ! {
    let mut val: usize;
    unsafe { asm!("csrr {}, misa", out(reg) val) };
    let console: &mut UART_SHAKTI = UART_SHAKTI::new(UART_SHAKTI_ADDR);
    write!(console, "Hello, {}!\n", "RISC-V").unwrap();
    write!(console, "misa: {:#b}\n", val).unwrap();

    loop {}
}
