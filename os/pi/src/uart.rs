use core::fmt;

use volatile::prelude::*;
use volatile::{Volatile, ReadVolatile};

use timer;
use common::IO_BASE;
use gpio::{Gpio, Function};

/// The base address for the `MU` registers.
const MU_REG_BASE: usize = IO_BASE + 0x215040;

/// The `AUXENB` register from page 9 of the BCM2837 documentation.
const AUX_ENABLES: *mut Volatile<u8> = (IO_BASE + 0x215004) as *mut Volatile<u8>;

/// Enum representing bit fields of the `AUX_MU_LSR_REG` register.
#[repr(u8)]
enum LsrStatus {
    DataReady = 1,
    TxAvailable = 1 << 5,
}

#[repr(C)]
#[allow(non_snake_case)]
struct Registers {
    AUX_MU_IO_REG: Volatile<u32>,  // 8 Mini Uart I/O Data  
    AUX_MU_IER_REG: Volatile<u32>,  // 8 Mini Uart Interrupt Enable 
    AUX_MU_IIR_REG: Volatile<u32>,  // 8 Mini Uart Interrupt Identify
    AUX_MU_LCR_REG: Volatile<u32>,  // 8 Mini Uart Line Control 
    AUX_MU_MCR_REG: Volatile<u32>,  // 8 Mini Uart Modem Control 
    AUX_MU_LSR_REG: ReadVolatile<u32>,  // 8 Mini Uart Line Status 
    AUX_MU_MSR_REG: ReadVolatile<u32>,  // 8 Mini Uart Modem Status 
    AUX_MU_SCRATCH: Volatile<u32>,  // 8 Mini Uart Scratch 
    AUX_MU_CNTL_REG: Volatile<u32>,  // 8 Mini Uart Extra Control 
    AUX_MU_STAT_REG: ReadVolatile<u32>,  // 32 Mini Uart Extra Status 
    AUX_MU_BAUD_REG: Volatile<u32>,  // 16 Mini Uart Baudrate 
}

/// The Raspberry Pi's "mini UART".
pub struct MiniUart {
    registers: &'static mut Registers,
    timeout: Option<u32>,
}

impl MiniUart {
    /// Initializes the mini UART by enabling it as an auxiliary peripheral,
    /// setting the data size to 8 bits, setting the BAUD rate to ~115200 (baud
    /// divider of 270), setting GPIO pins 14 and 15 to alternative function 5
    /// (TXD1/RDXD1), and finally enabling the UART transmitter and receiver.
    ///
    /// By default, reads will never time out. To set a read timeout, use
    /// `set_read_timeout()`.
    pub fn new() -> MiniUart {
        let registers = unsafe {
            // Enable the mini UART as an auxiliary device.
            (*AUX_ENABLES).or_mask(1);
            &mut *(MU_REG_BASE as *mut Registers)
        };
        // set GPIO pins 14 and 15 to alternative function 5
        let _pins = &[
            Gpio::new(14).into_alt(Function::Alt5),
            Gpio::new(15).into_alt(Function::Alt5),
        ];
        // set the data size to 8 bits
        registers.AUX_MU_LCR_REG.write(0x3);
        // set the baud rate to 115200
        registers.AUX_MU_BAUD_REG.write(270);
        // enable transmitter and receiver
        registers.AUX_MU_CNTL_REG.or_mask(0x2 | 0x1);
        MiniUart {registers: registers, timeout: None}
    }

    /// Set the read timeout to `milliseconds` milliseconds.
    pub fn set_read_timeout(&mut self, milliseconds: u32) {
        self.timeout = Some(milliseconds);
    }

    /// Write the byte `byte`. This method blocks until there is space available
    /// in the output FIFO.
    pub fn write_byte(&mut self, byte: u8) {
        while 0 == (self.registers.AUX_MU_LSR_REG.read() & LsrStatus::TxAvailable as u32) {
            // block until space for a byte (bit 5 in LSR)
        }
        self.registers.AUX_MU_IO_REG.write(byte as u32)
    }


    /// Returns `true` if there is at least one byte ready to be read. If this
    /// method returns `true`, a subsequent call to `read_byte` is guaranteed to
    /// return immediately. This method does not block.
    pub fn has_byte(&self) -> bool {
        return 1 == self.registers.AUX_MU_LSR_REG.read() & LsrStatus::DataReady as u32
    }

    /// Blocks until there is a byte ready to read. If a read timeout is set,
    /// this method blocks for at most that amount of time. Otherwise, this
    /// method blocks indefinitely until there is a byte to read.
    ///
    /// Returns `Ok(())` if a byte is ready to read. Returns `Err(())` if the
    /// timeout expired while waiting for a byte to be ready. If this method
    /// returns `Ok(())`, a subsequent call to `read_byte` is guaranteed to
    /// return immediately.
    pub fn wait_for_byte(&self) -> Result<(), ()> {
        let start = timer::current_time();
        while !self.has_byte() {
            match self.timeout {
                Some(n) => if timer::current_time() > start + n as u64 {
                    return Err(());
                }
                None => {}
            }
        }
        return Ok(())
    }

    /// Reads a byte. Blocks indefinitely until a byte is ready to be read.
    pub fn read_byte(&mut self) -> u8 {
        while !self.has_byte() {}
        self.registers.AUX_MU_IO_REG.read() as u8 & 0xff
    }
}

// Implement `fmt::Write` for `MiniUart`. A b'\r' byte should be written
// before writing any b'\n' byte.
impl fmt::Write for MiniUart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.as_bytes() {
            if *byte == '\n' as u8 {
                self.write_byte('\r' as u8);
            }
            self.write_byte(*byte);
        }
        return Ok(())
    }
}


#[cfg(feature = "std")]
mod uart_io {
    use std::io;
    use super::MiniUart;

// Implement `io::Read` and `io::Write` for `MiniUart`.
impl io::Read for MiniUart {
    // The `io::Read::read()` implementation must respect the read timeout by
    // waiting at most that time for the _first byte_. It should not wait for
    // any additional bytes but _should_ read as many bytes as possible. If the
    // read times out, an error of kind `TimedOut` should be returned.
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut received = 0;
        match self.wait_for_byte() {
            Ok(_) => {
                while self.has_byte() && received < buf.len() - 1 {
                    buf[received] = self.read_byte();
                    received += 1;
                }
                return Ok(received)
            }
            Err(_) => Err(io::Error::new(io::ErrorKind::TimedOut, "timed out waiting for first byte"))
        }
    }
}

impl io::Write for MiniUart {
    // The `io::Write::write()` method must write all of the requested bytes
    // before returning.
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        for byte in buf {
            self.write_byte(*byte);
        }
        return Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

}
