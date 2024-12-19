#![no_main]
#![no_std]

use cortex_m::asm::delay;
use cortex_m_rt::entry;
use defmt_rtt as _;
use npcx490m_pac as pac;
use panic_probe as _;

use defmt::info;

macro_rules! configure_pin {
    ($gpio:expr, $pin:ident) => {
        $gpio.px_dir().modify(|_, w| w.$pin().output());
        $gpio.px_pud().modify(|_, w| w.$pin().pull_down());
        $gpio.px_pull().modify(|_, w| w.$pin().enabled());
    };
}

macro_rules! toggle_pin {
    ($gpio:expr, $pin:ident) => {
        if $gpio.px_dout().read().$pin().is_low() {
            $gpio.px_dout().modify(|_, w| w.$pin().high());
        } else {
            $gpio.px_dout().modify(|_, w| w.$pin().low());
        }
    };
}

#[entry]
fn main() -> ! {
    info!("Hello world");

    if let Some(p) = pac::Peripherals::take() {
        configure_pin!(p.gpiob, pin7);
        configure_pin!(p.gpio6, pin0);
        configure_pin!(p.gpioc, pin0);
    
        loop {
            toggle_pin!(p.gpiob, pin7);
            toggle_pin!(p.gpio6, pin0);
            toggle_pin!(p.gpioc, pin0);

            delay(5_000_000);
        }
    } else {
        panic!("Unable to access Peripherals");
    };
}
