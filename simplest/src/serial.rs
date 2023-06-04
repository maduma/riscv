use core::fmt::Write;
use core::fmt::Result;
use volatile_register::RW;
use volatile_register::RO;

#[repr(C)]
pub struct UART_16550A {
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

impl UART_16550A { 
    #[allow(dead_code)]
    pub fn new(addr: usize) -> &'static mut UART_16550A {
        unsafe { &mut *(addr as *mut UART_16550A) }
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

impl Write for UART_16550A {
    fn write_str(&mut self, msg: &str) -> Result {
        self.write_ascii_str(msg);
        Ok(())
    }
}

#[repr(C)]
pub struct UART_SHAKTI {
    baud: RW<u16>,
    reserv0: RO<u16>,
    tx_reg: RW<u32>,
    rcv_reg: RO<u32>,
    status: RO<u8>,
    reserv1: RO<u8>,
    reserv2: RO<u16>,
    delay: RW<u16>,
    reserv3: RO<u16>,
    control: RW<u16>,
    reserv5: RO<u16>,
    ien: RW<u8>,
    reserv6: RO<u8>,
    reserv7: RO<u16>,
    iqcycles: RW<u8>,
    reserv8: RO<u8>,
    reserv9: RO<u16>,
    rx_threshold: RW<u8>,
    reserv10: RO<u8>,
    reserv11: RO<u16>,
}

const STS_TX_FULL: u8 = 1 << 1;

impl UART_SHAKTI {
    #[allow(dead_code)]
    pub fn new(addr: usize) -> &'static mut UART_SHAKTI {
        unsafe { &mut *(addr as *mut UART_SHAKTI) }
    }
    
    fn buffer_full(&self) -> bool {
        self.status.read() & STS_TX_FULL != 0
    }

    fn write_byte(&mut self, c:u8) {
        while self.buffer_full() { }
        unsafe { self.tx_reg.write(c.into()) };
    }

    fn write_ascii_str(&mut self, msg: &str) {
        for c in msg.bytes() { self.write_byte(c) }
    }
}

impl Write for UART_SHAKTI {
    fn write_str(&mut self, msg: &str) -> Result {
        self.write_ascii_str(msg);
        Ok(())
    }
}