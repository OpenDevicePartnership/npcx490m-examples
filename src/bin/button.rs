#![no_main]
#![no_std]

use cortex_m_rt::entry;
use defmt_rtt as _;
use npcx490m_pac as pac;
use panic_probe as _;

use defmt::info;

#[entry]
fn main() -> ! {
    info!("To use, configure J2 to '2' and press SW1. LED D8 should light up.");

    if let Some(p) = pac::Peripherals::take() {
        p.gpio6.px_dir().modify(|_, w| w.pin3().input());
        p.gpio6.px_pud().modify(|_, w| w.pin3().pull_up());
        p.gpio6.px_pull().modify(|_, w| w.pin3().enabled());

        p.gpiob.px_dir().modify(|_, w| w.pin7().output());
        p.gpiob.px_pull().modify(|_, w| w.pin7().enabled());

        loop {
            let b = p.gpio6.px_din().read().pin3().is_low();

            p.gpiob.px_dout().modify(|_, w| {
                let pin = w.pin7();
                if b {
                    pin.low()
                } else {
                    pin.high()
                }
            });
        }
    } else {
        panic!("Unable to access Peripherals");
    };
}
