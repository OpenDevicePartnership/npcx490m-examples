#![no_main]
#![no_std]

use cortex_m_rt::entry;
use defmt_rtt as _;
use npcx490m_pac as pac;
use panic_probe as _;

use defmt::info;

#[entry]
fn main() -> ! {
    info!("Hello world");

    if let Some(p) = pac::Peripherals::take() {
        // enable lpc mode
        p.sysconfig.devcnt().modify(|_, w| unsafe {w.hif_typ_sel().bits(1)});

        // Set pin 7 as output
        p.gpiob.px_dir().write(|w| w.pin7().output());

        p.sysglue.smb_sel().modify(|_, w| w.smb5_sl().set_bit());
        p.miwu2.wkpcln7().write(|w| w.input0().set_bit());
        p.sysconfig.devalt6().modify(|_, w| w.i2c5_1_sl().set_bit());
        
        p.smb5.smbn_ctl3().write(|w| unsafe {w.scl_lvl().set_bit().sda_lvl().set_bit().bnk_sel().clear_bit()._400k_mode().clear_bit().slp_start().clear_bit().arpmen().clear_bit().sclfrq8_7().bits(0)});
        p.smb5.smbn_ctl4().modify(|_, w| unsafe {w.lvl_we().clear_bit().hldt().bits(15)});
        p.smb5.smbn_fif_ctl().modify(|_, w| w.fifo_en().clear_bit());
        p.smb5.smbn_ctl2().write(|w| unsafe {w.sclfrq6_0().bits(63).enable().clear_bit()});
        p.smb5.smbn_ctl2().modify(|_, w| w.enable().set_bit());

        // Send start edge
        p.smb5.smbn_ctl1().modify(|_, w| w.start().set_bit());

        // Wait for completion
        loop {
            let r = p.smb5.smbn_st().read();
            if r.ber().bit_is_set() || r.sdast().bit_is_set() {
                break;
            }
        }
        if p.smb5.smbn_st().read().ber().bit_is_set() {
            panic!("Error during start");
        }
        
        // Send address
        p.smb5.smbn_sda().write(|w| unsafe {w.bits(0xd4)});
        
        // Wait for completion
        loop {
            let r = p.smb5.smbn_st().read();
            if r.ber().bit_is_set() || r.sdast().bit_is_set() {
                break;
            }
        }
        if p.smb5.smbn_st().read().ber().bit_is_set() {
            panic!("Error during start");
        }
        if p.smb5.smbn_st().read().negack().bit_is_set() {
            panic!("Address byte not acknowledged");
        }

        // Send register address
        p.smb5.smbn_sda().write(|w| unsafe {w.bits(0x0f)});

        // Wait for completion
        loop {
            let r = p.smb5.smbn_st().read();
            if r.ber().bit_is_set() || r.sdast().bit_is_set() {
                break;
            }
        }
        if p.smb5.smbn_st().read().ber().bit_is_set() {
            panic!("Error during start");
        }
        if p.smb5.smbn_st().read().negack().bit_is_set() {
            panic!("Address byte not acknowledged");
        }

        // Repeated start
        p.smb5.smbn_ctl1().modify(|_, w| w.start().set_bit());

        // Wait for completion
        loop {
            let r = p.smb5.smbn_st().read();
            if r.ber().bit_is_set() || r.sdast().bit_is_set() {
                break;
            }
        }
        if p.smb5.smbn_st().read().ber().bit_is_set() {
            panic!("Error during start");
        }

        // Send address
        p.smb5.smbn_ctl1().modify(|_,w| w.ack().set_bit());
        p.smb5.smbn_sda().write(|w| unsafe {w.bits(0xd5)});
        
        // Wait for completion
        loop {
            let r = p.smb5.smbn_st().read();
            if r.ber().bit_is_set() || r.sdast().bit_is_set() {
                break;
            }
        }
        if p.smb5.smbn_st().read().ber().bit_is_set() {
            panic!("Error during start");
        }
        if p.smb5.smbn_st().read().negack().bit_is_set() {
            panic!("Address byte not acknowledged");
        }

        // Read result
        p.smb5.smbn_ctl1().modify(|_,w| w.stop().set_bit());
        let response = p.smb5.smbn_sda().read().bits();
        info!("Received: {:02x}", response);

        info!("Done");

        loop {
        }
    } else {
        panic!("Unable to access Peripherals");
    };
}
