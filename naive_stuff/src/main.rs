#![deny(unsafe_code)]
#![no_std]
#![no_main]

extern crate lcd1602a;
use lcd1602a::LCD1602A;

// use nb::block;
use panic_halt as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Acquire the GPIOC peripheral
    let mut gpioc = dp.GPIOC.split();
    let mut gpioa = dp.GPIOA.split();

    // Configure gpio C pin 13 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let p0 = gpioa.pa0.into_push_pull_output(&mut gpioa.crl).erase();
    let p1 = gpioa.pa1.into_push_pull_output(&mut gpioa.crl).erase();
    let p2 = gpioa.pa2.into_push_pull_output(&mut gpioa.crl).erase();
    let p3 = gpioa.pa3.into_push_pull_output(&mut gpioa.crl).erase();
    let p4 = gpioa.pa4.into_push_pull_output(&mut gpioa.crl).erase();
    let p5 = gpioa.pa5.into_push_pull_output(&mut gpioa.crl).erase();
    let p6 = gpioa.pa6.into_push_pull_output(&mut gpioa.crl).erase();
    let p7 = gpioa.pa7.into_push_pull_output(&mut gpioa.crl).erase();

    let e = gpioa.pa8.into_push_pull_output(&mut gpioa.crh).erase(); // H-L - enable
    let rs = gpioa.pa9.into_push_pull_output(&mut gpioa.crh).erase(); // H - data, L - instr

    // Configure the syst timer to trigger an update every second
    let sys_tick = Timer::syst(cp.SYST, &clocks);
    // let mut timer = sys_tick.counter_hz();
    // timer.start(10.Hz()).unwrap();

    let timer_2 = sys_tick.counter_us();

    let mut lcd = LCD1602A::new(p0, p1, p2, p3, p4, p5, p6, p7, e, rs, timer_2);
    lcd.init_wait();

    lcd.clear();
    lcd.on_off(true, true, true);
    lcd.function_set(true, true, false);
    lcd.print("42");
    lcd.display_shift(true);
    lcd.display_shift(true);
    lcd.print("Test");
    lcd.print("Yellow!");

    lcd.move_to_2nd_line();
    lcd.print("Second line");
    lcd.cursor_shift(true);
    lcd.print(":-)");

    // lcd.print("Test One Two Three Four Five Six Seven Nine Ten");
    // lcd.print("Test One Two Three Four Five Six Seven Nine Ten");
    lcd.return_home();

    // Wait for the timer to trigger an update and change the state of the LED
    // timer_2.start(1.secs()).unwrap();
    // led.set_low();
    loop {
        //     block!(timer_2.wait()).unwrap();
        //     led.set_high();
        //     block!(timer_2.wait()).unwrap();
        led.set_high();
        // lcd.display_shift(true);
    }
}
