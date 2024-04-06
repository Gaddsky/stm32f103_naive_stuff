#![deny(unsafe_code)]
#![no_std]
#![no_main]

use nb::block;

use stm32f1xx_hal::gpio::{ErasedPin, Output};
use stm32f1xx_hal::timer::SysCounterUs;
use stm32f1xx_hal::prelude::*;

const TIMEOUT_1520: u32 = 1520;
const TIMEOUT_37: u32 = 37;
const INIT_TIMEOUT: u32 = 100_000;

pub struct LCD1602A {
    pins: [ErasedPin<Output>; 8],
    e: ErasedPin<Output>,
    rs: ErasedPin<Output>,
    t: SysCounterUs,
}

impl LCD1602A {
    pub fn new(
        p0: ErasedPin<Output>,
        p1: ErasedPin<Output>,
        p2: ErasedPin<Output>,
        p3: ErasedPin<Output>,
        p4: ErasedPin<Output>,
        p5: ErasedPin<Output>,
        p6: ErasedPin<Output>,
        p7: ErasedPin<Output>,
        e: ErasedPin<Output>,
        rs: ErasedPin<Output>,
        t: SysCounterUs,
    ) -> Self {
        LCD1602A {
            pins: [p0, p1, p2, p3, p4, p5, p6, p7],
            e,
            rs,
            t,
        }
    }

    pub fn init_wait(&mut self) {
        self.t.start(INIT_TIMEOUT.micros()).unwrap();
        block!(self.t.wait()).unwrap();
    }

    fn write(&mut self, byte: u8, rs: bool, timeout_us: u32) {
        if rs {
            self.rs.set_high()
        } else {
            self.rs.set_low()
        };
        self.e.set_high();
        for (n, pin) in self.pins.iter_mut().enumerate() {
            if byte & (1 << n) != 0 {
                pin.set_high();
            } else {
                pin.set_low();
            }
        }
        self.e.set_low();
        self.t.start(timeout_us.micros()).unwrap();
        block!(self.t.wait()).unwrap();
    }

    pub fn clear(&mut self) {
        self.write(0b_0000_0001, false, TIMEOUT_1520);
    }

    pub fn return_home(&mut self) {
        self.write(0b_0000_0010, false, TIMEOUT_1520);
    }

    fn write_command(&mut self, data: u8) {
        self.write(data, false, TIMEOUT_37);
    }

    pub fn on_off(&mut self, display_on: bool, cursor_on: bool, cursor_blink: bool) {
        let mut data = 0b_0000_1000;
        if display_on {
            data |= 0b_0000_0100
        }
        if cursor_on {
            data |= 0b_0000_0010
        }
        if cursor_blink {
            data |= 0b_0000_0001
        }
        self.write_command(data);
    }

    pub fn function_set(&mut self, data_length: bool, line_number: bool, font_type: bool) {
        let mut data = 0b_0010_0000;
        if data_length {
            data |= 0b_0001_0000;
        }
        if line_number {
            data |= 0b_0000_1000;
        }
        if font_type {
            data |= 0b_0000_0100;
        }
        self.write_command(data);
    }

    pub fn cursor_shift(&mut self, move_right: bool) {
        let mut data = 0b_0001_0000;
        if move_right {
            data |= 0b_0000_0100;
        }
        self.write_command(data);
    }

    pub fn display_shift(&mut self, move_right: bool) {
        let mut data = 0b_0001_1000;
        if move_right {
            data |= 0b_0000_0100;
        }
        self.write_command(data);
    }

    fn set_ddram_address(&mut self, address: u8) { // make pub?
        let data = address | 0b_1000_0000;
        self.write_command(data);
    }

    pub fn move_to_2nd_line(&mut self) {
        self.set_ddram_address(0x40);
    }

    pub fn print(&mut self, string: &str) {
        for ch in string.chars() {
            self.write(ch as u8, true, TIMEOUT_37);
        }
    }
}
