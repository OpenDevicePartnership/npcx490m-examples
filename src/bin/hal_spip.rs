#![no_main]
#![no_std]

use cortex_m::asm::delay;
use defmt_rtt as _;
use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice as MutexSpiDevice;
use embassy_executor::Spawner;
use embassy_npcx::{
    gpio::{Level, OutputOnly, OutputOpenDrain},
    peripherals::SPIP,
    spip::Spip,
};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_npcx::init();

    let spip = Spip::new_8bit(p.SPIP, p.PK12, p.PM12, p.PL12, Default::default());
    let spip = Mutex::<NoopRawMutex, Spip<SPIP, u8>>::new(spip);

    let cs0: OutputOpenDrain<'_, OutputOnly> =
        OutputOpenDrain::<'_, OutputOnly>::new(p.PL10, Level::High);
    let cs1 = OutputOpenDrain::<'_, OutputOnly>::new(p.PK11, Level::High);

    let mut flash0 = MutexSpiDevice::new(&spip, cs0);
    let mut flash1 = MutexSpiDevice::new(&spip, cs1);

    use embedded_hal_async::spi::SpiDevice;

    loop {
        let mut buf = [0; 13];
        buf[0] = 0x4B;
        flash0.transfer_in_place(&mut buf).await.unwrap();
        defmt::info!("flash0 {:?}", buf);
        delay(5_000_000);

        let mut buf = [0x05, 0x00];
        flash1.transfer_in_place(&mut buf).await.unwrap();
        defmt::info!("flash1 {:?}", buf);
        delay(5_000_000);
    }
}
