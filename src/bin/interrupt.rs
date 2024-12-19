#![no_main]
#![no_std]

use cortex_m::asm::delay;
use cortex_m_rt::entry;
use defmt_rtt as _;
use npcx490m_pac as pac;
use panic_probe as _;

use defmt::info;
use pac::{Interrupt, interrupt};

#[pac::interrupt]
fn WKINTA_0() {
    info!("Interrupt");
}

#[entry]
fn main() -> ! {
    info!("Hello world");

    unsafe { npcx490m_pac::NVIC::unmask(Interrupt::WKINTA_0) };

    loop {
        npcx490m_pac::NVIC::pend(Interrupt::WKINTA_0);
        delay(5_000_000);
    }
}
