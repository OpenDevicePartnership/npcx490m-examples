#![no_main]
#![no_std]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_npcx::{
    self as hal,
    gpio::{Level, Output, OutputOnly},
    gpio_miwu::AwaitableInput,
    interrupt::InterruptExt,
};
use panic_probe as _;

use defmt::info;

use embedded_hal_async::digital::Wait;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("To use, configure J2 to '2' and press SW1. LED D8 should light up.");

    let p = embassy_npcx::init();

    let mut button = AwaitableInput::new(p.PJ02, p.MIWU1_73);
    button.enable_pullup();

    let mut led = Output::<'_, OutputOnly>::new(p.PJ07, Level::High);
    unsafe { hal::interrupt::WKINTG_1.enable() };

    loop {
        button.wait_for_low().await.unwrap();
        led.set_low();

        button.wait_for_high().await.unwrap();
        led.set_high();
    }
}
