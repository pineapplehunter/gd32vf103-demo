//! This example will alternate PA2 between low and high. If you connect a
//! standard LED (protected with a 220 ohm resistor), you can see it blink.

#![no_std]
#![no_main]

use embedded_hal::delay::blocking::DelayUs;
use embedded_hal::digital::blocking::ToggleableOutputPin;
use gd32vf103_hal::{
    ctimer::CoreTimer,
    delay::Delay,
    gpio::GpioExt,
    pac::Peripherals,
    rcu::{RcuExt, Strict},
};
use panic_halt as _;

use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let mut rcu = dp.RCU.constrain();

    let mut gpioa = dp.GPIOA.split(&mut rcu.apb2);
    let mut pa8 = gpioa
        .pa8
        .into_push_pull_output(&mut gpioa.ctl1)
        .lock(&mut gpioa.lock);
    gpioa.lock.freeze();

    let clocks = Strict::new().freeze(&mut rcu.cfg);
    let ctimer = CoreTimer::new(dp.CTIMER);
    let mut delay = Delay::new(clocks, ctimer);
    loop {
        pa8.toggle().unwrap();
        delay.delay_ms(1000u32).unwrap();
    }
}
