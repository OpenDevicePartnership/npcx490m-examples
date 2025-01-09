#![no_main]
#![no_std]

use cortex_m_rt::entry;
use defmt_rtt as _;
use npcx490m_pac as pac;
use panic_probe as _;

use defmt::info;
use pac::{interrupt, Interrupt};

// MIWU1 - WUI73 - WKINTG_1
const GROUP: usize = 6;
const SUBGROUP: u8 = 3;

#[pac::interrupt]
fn WKINTG_1() {
    let miwu1 = unsafe { pac::Miwu1::steal() };

    if miwu1.wkpndn(GROUP).read().input(SUBGROUP).is_pending() {
        // Turn the LED on
        unsafe { pac::Gpiob::steal() }
            .px_dout()
            .modify(|_, w| w.pin7().low());

        miwu1.wkpcln(GROUP).write(|w| w.input(SUBGROUP).clear());
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

        // a. Configure the type of input detection for the WUIxy input by writing the WKMDy bit in the WKMODnx register, the
        // WKEDy bit in the WKEDGnx register and/or the WKAEDy bit in the WKAEDGnx register.
        // Set to edge detection (should already be set)
        p.miwu1
            .wkmodn(GROUP)
            .modify(|_, w| w.input(SUBGROUP).edge());

        // Set to specific edge detection (should already be set)
        p.miwu1
            .wkaedgn(GROUP)
            .modify(|_, w| w.input(SUBGROUP).edge());

        // Set to high-to-low-edge detection
        p.miwu1
            .wkedgn(GROUP)
            .modify(|_, w| w.input(SUBGROUP).low_falling());

        // Enable the input (should already be set)
        p.miwu1
            .wkinenn(GROUP)
            .modify(|_, w| w.input(SUBGROUP).enabled());

        // b. Clear the WKPDy bit in WKPNDnx register associated with the WUIxy input by writing 1 to the relevant bit in the WKPCLnx register.
        p.miwu1.wkpcln(GROUP).write(|w| w.input(SUBGROUP).clear());

        // c. Enable wake-up by setting the WKEy bit in WKENnx register associated with the WUIxy input.
        p.miwu1
            .wkenn(GROUP)
            .modify(|_, w| w.input(SUBGROUP).enabled());

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
