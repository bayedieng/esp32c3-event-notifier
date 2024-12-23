#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{prelude::*, rng::Rng, time, timer::timg::TimerGroup};
use esp_println::println;
use esp_wifi::wifi::{ClientConfiguration, Configuration, WifiStaDevice};

use smoltcp::socket::tcp::{Socket, SocketBuffer};

const SSID: &str = env!("SSID");
const WIFI_PASS: &str = env!("WIFI_PASS");
const SOCKET_BUF_SIZE: usize = 1024;

#[entry]
fn main() -> ! {
    let config = esp_hal::Config::default();
    let peripherals = esp_hal::init(config);
    esp_alloc::heap_allocator!(72 * 1024);
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let mut wifi_device = peripherals.WIFI;
    let mut rng = Rng::new(peripherals.RNG);

    let elapsed_time = || time::now().duration_since_epoch().to_millis();

    let wifi_init = esp_wifi::init(timg0.timer0, rng, peripherals.RADIO_CLK).unwrap();

    let (iface, wifi_device, mut wifi_controller) =
        esp_wifi::wifi::utils::create_network_interface(
            &wifi_init,
            &mut wifi_device,
            WifiStaDevice,
        )
        .unwrap();

    let wifi_config = Configuration::Client(ClientConfiguration {
        ssid: SSID.try_into().unwrap(),
        password: WIFI_PASS.try_into().unwrap(),
        ..Default::default()
    });

    wifi_controller.set_configuration(&wifi_config).unwrap();
    wifi_controller.start().unwrap();
    wifi_controller.connect().unwrap();
    println!("Waiting for connection");

    if wifi_controller.is_connected().unwrap() {
        println!("Connected to Wifi")
    } else {
        println!("Could not connect to Wifi");
    }

    loop {}
}
