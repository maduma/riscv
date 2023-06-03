use core::fmt::Write;
use core::fmt::Result;
use volatile_register::RW;
use volatile_register::RO;

#[repr(C)]
pub struct UART { // 16550A
    rbr_thr_dll: RW<u8>,
    ier_dlm: RW<u8>,
    iir_fcr: RW<u8>,
    lcr: RW<u8>,
    mcr: RW<u8>,
    lsr: RO<u8>,
    msr: RO<u8>,
    scr: RW<u8>,
}

const THR_EMPTY_AND_LINE_IDLE: u8 = 1 << 6;

impl UART { 
    pub fn new(addr: usize) -> &'static mut UART {
        unsafe { &mut *(addr as *mut UART) }
    }

    fn buffer_full(&self) -> bool {
        self.lsr.read() & THR_EMPTY_AND_LINE_IDLE == 0
    }

    fn write_byte(&mut self, c:u8) {
        while self.buffer_full() { }
        unsafe { self.rbr_thr_dll.write(c) };
    }

    fn write_ascii_str(&mut self, msg: &str) {
        for c in msg.bytes() { self.write_byte(c) }
    }
}

impl Write for UART {
    fn write_str(&mut self, msg: &str) -> Result {
        self.write_ascii_str(msg);
        Ok(())
    }
}