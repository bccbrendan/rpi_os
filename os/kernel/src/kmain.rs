#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(optin_builtin_traits)]
#![feature(decl_macro)]
#![feature(attr_literals)]
#![feature(never_type)]
#![feature(ptr_internals)]

extern crate pi;
extern crate stack_vec;

pub mod lang_items;
pub mod mutex;
pub mod console;
pub mod shell;

use pi::gpio::Gpio;
use pi::timer::spin_sleep_ms;
use pi::uart::MiniUart;
use std::fmt::Write;

#[no_mangle]
pub extern "C" fn kmain() {
    let pins = &mut [
        Gpio::new(5).into_output(),
        Gpio::new(6).into_output(),
        Gpio::new(13).into_output(),
        Gpio::new(16).into_output(),
        Gpio::new(19).into_output(),
        Gpio::new(26).into_output(),
    ];
    let mut uart = MiniUart::new();
    let mut i = 0;
    loop {
        pins[i].set();
        let byte = uart.read_byte();
        pins[i].clear();
        uart.write_byte(byte);
        i = (i + 1) % pins.len();
    }
}

