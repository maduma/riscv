use core::fmt::Write;
use core::fmt::Result;
use volatile_register::RW;
use volatile_register::RO;

#[repr(C)]
pub struct UART {
    pub rbr_thr_dll: RW<u8>,
    pub ier_dlm: RW<u8>,
    pub iir_fcr: RW<u8>,
    pub lcr: RW<u8>,
    pub mcr: RW<u8>,
    pub lsr: RO<u8>,
    pub msr: RO<u8>,
    pub scr: RW<u8>,
}


const UART_ADDR: usize = 0x10000000;
const THR_EMPTY_AND_LINE_IDLE: u8 = 1 << 6;

impl UART {
    // 16550A
    #[allow(dead_code)]
    pub fn new() -> &'static mut UART {
        unsafe { &mut *(UART_ADDR as *mut UART) }
    }

    fn is_thr_empty_and_line_idle(&self) -> bool {
        self.lsr.read() & THR_EMPTY_AND_LINE_IDLE == 0
    }

    fn write_u8(&mut self, c:u8) {
        while self.is_thr_empty_and_line_idle() { }
        unsafe { self.rbr_thr_dll.write(c) };
    }

    fn write_ascii_str(&mut self, msg: &str) {
        for c in msg.bytes() { self.write_u8(c) }
    }

}

impl Write for UART {
    fn write_str(&mut self, msg: &str) -> Result {
        self.write_ascii_str(msg);
        Ok(())
    }
}