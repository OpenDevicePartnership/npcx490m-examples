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
        // Set pin 0 as output
        p.gpio0.px_dir().write(|w| w.pin0().output());

        // Connect pull down on pin 0
        p.gpio0.px_pud().write(|w| w.pin0().pull_down());

        // Enable pull on pin 0
        p.gpio0.px_pull().write(|w| w.pin0().enable());

        loop {
            if p.gpio0.px_dout().read().pin0().is_low() {
                p.gpio0.px_dout().write(|w| w.pin0().high());
            } else {
                p.gpio0.px_dout().write(|w| w.pin0().low());
            }

            delay(500_000);
        }
    } else {
        panic!("Unable to access Peripherals");
    };
}
