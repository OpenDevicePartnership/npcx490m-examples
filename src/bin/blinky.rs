#![no_main]
#![no_std]

use cortex_m::asm::delay;
use cortex_m_rt::entry;
use defmt_rtt as _;
use npcx490m_pac as pac;
use panic_probe as _;

use defmt::info;

#[entry]
fn main() -> ! {
    info!("Hello world");

    if let Some(p) = pac::Peripherals::take() {
        // Set pin 7 as output
        p.gpiob.px_dir().write(|w| w.pin7().output());

        // Connect pull down on pin 7
        p.gpiob.px_pud().write(|w| w.pin7().pull_down());

        // Enable pull on pin 7
        p.gpiob.px_pull().write(|w| w.pin7().enabled());

        loop {
            if p.gpiob.px_dout().read().pin7().is_low() {
                p.gpiob.px_dout().write(|w| w.pin7().high());
            } else {
                p.gpiob.px_dout().write(|w| w.pin7().low());
            }

            delay(5_000_000);
        }
    } else {
        panic!("Unable to access Peripherals");
    };
}
