#![no_main]
#![no_std]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_npcx::{
    self as hal,
    gpio::{Input, Output, OutputOnly},
    interrupt::InterruptExt,
    miwu::{Level, WakeUp},
};
use panic_probe as _;

use defmt::info;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = hal::init();

    let mut wui = WakeUp::new(p.MIWU1_73);
    wui.disable();

    unsafe { hal::interrupt::WKINTG_1.enable() };

    let input = p.PJ02;
    let mut input = Input::new(input);
    input.enable_pullup();

    let output = p.PJ07;
    let mut output = Output::<'_, OutputOnly>::new(output, embassy_npcx::gpio::Level::Low);

    info!("Starting");

    loop {
        info!("LOW");
        output.set_high();

        wui.wait_for(Level::Low).await;

        info!("HIGH");
        output.set_low();

        wui.wait_for(Level::High).await;
    }
}
