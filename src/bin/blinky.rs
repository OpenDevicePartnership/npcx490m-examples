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
        p.gpio0.px_dir().write(|w| unsafe { w.bits(1 << 0) });

        // Connect pull down on pin 0
        p.gpio0.px_pud().write(|w| unsafe { w.bits(1 << 0) });

        // Enable pull on pin 0
        p.gpio0.px_pull().write(|w| unsafe { w.bits(1 << 0) });

        loop {
            let out = p.gpio0.px_dout().read().bits() & (1 << 0);

            if out == 0 {
                p.gpio0.px_dout().modify(|_, w| unsafe { w.bits(1) });
            } else {
                p.gpio0.px_dout().modify(|_, w| unsafe { w.bits(0) });
            }

            delay(500_000);
        }
    } else {
        panic!("Unable to access Peripherals");
    };
}
