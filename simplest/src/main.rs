#![no_std]
#![no_main]

mod serial;

use core::arch::global_asm;
use core::panic::PanicInfo;
use serial::UART16550A;
use core::fmt::Write;

global_asm!(include_str!("init.s"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}


#[no_mangle]
pub extern "C" fn _start_rust() -> ! {

    let console = UART16550A::new();
    write!(console, "Hello, {}!\n", "RISC-V").unwrap();

    loop {}
}
