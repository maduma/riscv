#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;
use core::fmt::Write;
use core::fmt::Result;

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

pub fn console(uart: usize, msg: &str) {
    let base_addr = uart as *mut u8;
    unsafe {
        let status_addr = base_addr.offset(5);
        for c in msg.bytes() {
            while core::ptr::read_volatile(status_addr) & THR_EMPTY_AND_LINE_IDLE == 0 { }
            core::ptr::write_volatile(base_addr, c);
        }
    }
}

struct Serial;

impl Write for Serial {
    fn write_str(&mut self, msg: &str) -> Result {
        console(UART, msg);
        Ok(())
    }
}

#[no_mangle]
#[link_section = ".init"]
pub extern "C" fn _start() -> ! {

    extern "C" {
        static  _stack_start: u64;
    }

    type FnPtr = fn() -> !;
    let th: FnPtr = trap_handler;

    unsafe{
        let sp = &_stack_start;
        asm!("csrw mtvec, {}" ,
            in(reg) th);
        asm!("mv sp, {}" ,
            in(reg) sp);
    }

    let _msg ="\
        Hello World!\n\
        How are you men!\n\
        this\tis\tmy\tname\n\
        one\ttime\tis\tthere\n\
        ";

    let mut cons = Serial;
    write!(cons, "Hello {}!", "Stephane").unwrap();
    
    cons.write_str(&_msg).unwrap();

    loop {}
}
