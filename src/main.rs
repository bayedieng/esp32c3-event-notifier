#![no_std]
#![no_main]

use core::panic::PanicInfo;
use esp_hal::{prelude::*, rng::Rng, timer::timg::TimerGroup};
use esp_wifi::wifi::WifiStaDevice;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    esp_println::println!(
        "Panicked at: {:?}\n Line: {:?}",
        info.message(),
        info.location().unwrap().line()
    );
    loop {}
}

#[entry]
fn main() -> ! {
    let config = esp_hal::Config::default();
    let peripherals = esp_hal::init(config);
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let mut wifi_device = peripherals.WIFI;
    let wifi_init = esp_wifi::init(
        timg0.timer0,
        Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    )
    .unwrap();
    let (iface, wifi_device, mut wifi_controller) =
        esp_wifi::wifi::utils::create_network_interface(
            &wifi_init,
            &mut wifi_device,
            WifiStaDevice,
        )
        .unwrap();
    loop {}
}
