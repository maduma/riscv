#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

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
        core::ptr::write_volatile(base_addr.offset(3), 1 << 7);
        let tmp2: u8 = core::ptr::read_volatile(base_addr);
        asm!("mv t2, {}", in(reg) tmp2);
        let tmp1: u8 = core::ptr::read_volatile(base_addr.offset(3));
        asm!("mv t1, {}", in(reg) tmp1);
        core::ptr::write_volatile(base_addr.offset(3), 0);
        let status_addr = base_addr.offset(5);
        for c in msg.bytes() {
            while core::ptr::read_volatile(status_addr) & THR_EMPTY_AND_LINE_IDLE == 0 { }
            core::ptr::write_volatile(base_addr, c);
        }
    }
}

#[no_mangle]
#[link_section = ".init"]
pub fn _start() -> ! {

    extern "C" {
        static  _stack_start: u32;
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

    let msg ="\
        Hello World!\n\
        How are you men!\n\
        this\tis\tmy\tname\n\
        one\ttime\tis\tthere\n\
        ";

    console(UART, msg);

    loop {}
}
