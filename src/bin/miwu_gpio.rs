#![no_main]
#![no_std]

use cortex_m_rt::entry;
use defmt_rtt as _;
use npcx490m_pac as pac;
use panic_probe as _;

use defmt::info;
use pac::{interrupt, Interrupt};

#[pac::interrupt]
fn WKINTG_1() {
    let miwu1 = unsafe { pac::Miwu1::steal() };

    if miwu1.wkpndn5().read().bits() & (0b1 << 3) != 0 {
        // Turn the LED on
        unsafe { pac::Gpiob::steal() }
            .px_dout()
            .modify(|_, w| w.pin7().low());

        miwu1.wkpcln5().write(|w| unsafe { w.bits(0b1 << 3) });
    }

    info!("Interrupt");
}

#[entry]
fn main() -> ! {
    info!("To use, configure J2 to '2' and press SW1. LED D8 should light up.");

    if let Some(p) = pac::Peripherals::take() {
        p.gpio6.px_dir().modify(|_, w| w.pin3().input());
        p.gpio6.px_pud().modify(|_, w| w.pin3().pull_up());
        p.gpio6.px_pull().modify(|_, w| w.pin3().enabled());

        p.gpiob.px_dir().modify(|_, w| w.pin7().output());
        p.gpiob.px_pull().modify(|_, w| w.pin7().enabled());

        // MIWU1 - WUI73 - WKINTG_1

        // a. Configure the type of input detection for the WUIxy input by writing the WKMDy bit in the WKMODnx register, the
        // WKEDy bit in the WKEDGnx register and/or the WKAEDy bit in the WKAEDGnx register.
        // Set to edge detection (should already be set)
        p.miwu1
            .wkmodn5()
            .modify(|r, w| unsafe { w.bits(r.bits() & !(0b1 << 3)) });

        // Set to specific edge detection (should already be set)
        p.miwu1
            .wkaedgn5()
            .modify(|r, w| unsafe { w.bits(r.bits() & !(0b1 << 3)) });

        // Set to high-to-low-edge detection
        p.miwu1
            .wkedgn5()
            .modify(|r, w| unsafe { w.bits(r.bits() | 0b1 << 3) });

        // Enable the input (should already be set)
        p.miwu1
            .wkinenn5()
            .modify(|r, w| unsafe { w.bits(r.bits() | 0b1 << 3) });

        // b. Clear the WKPDy bit in WKPNDnx register associated with the WUIxy input by writing 1 to the relevant bit in the WKPCLnx register.
        p.miwu1.wkpcln5().write(|w| unsafe { w.bits(0b1 << 3) });

        // c. Enable wake-up by setting the WKEy bit in WKENnx register associated with the WUIxy input.
        p.miwu1
            .wkenn5()
            .modify(|r, w| unsafe { w.bits(r.bits() | (0b1 << 3)) });

        // d. Enable the interrupt by setting the relevant bit in NVIC.
        unsafe { pac::NVIC::unmask(Interrupt::WKINTG_1) };

        loop {
            if p.gpio6.px_din().read().pin3().is_high() {
                // Turn the LED off when button is no longer being pressed
                p.gpiob.px_dout().modify(|_, w| w.pin7().high());
            }
        }
    } else {
        panic!("Unable to access Peripherals");
    };
}
