#![no_std]
#![no_main]

const UART_ADDR: usize = 0x10000000;

mod serial;

use core::arch::global_asm;
use core::panic::PanicInfo;
use serial::UART;
use core::fmt::Write;

global_asm!(include_str!("init.s"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}


#[no_mangle]
pub extern "C" fn start_rust() -> ! {
    let console = UART::new(UART_ADDR);
    write!(console, "Hello, {}!\n", "RISC-V").unwrap();

    loop {}
}
