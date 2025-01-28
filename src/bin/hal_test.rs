#![no_main]
#![no_std]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_npcx::{
    self as hal,
    gpio::{InputCapable, Level, Output, OutputOnly},
    gpio_miwu::AwaitableInput,
    interrupt::InterruptExt,
};
use panic_probe as _;

use defmt::info;

use embedded_hal_async::digital::Wait;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_npcx::init();

    let mut led = Output::<'_, InputCapable>::new(p.PK12, Level::Low);

    loop {
        led.set_low();
        cortex_m::asm::delay(10);
        led.set_high();
        cortex_m::asm::delay(10);
    }
}
