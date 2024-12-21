#![no_std]
#![no_main]

use core::panic::PanicInfo;
use esp_hal::{prelude::*, rng::Rng, time, timer::timg::TimerGroup};
use esp_println::println;
use esp_wifi::wifi::{ClientConfiguration, Configuration, WifiStaDevice};
use smoltcp::iface::{SocketSet, SocketStorage};
use smoltcp::socket::tcp::{Socket, SocketBuffer};

const SSID: &str = env!("SSID");
const WIFI_PASS: &str = env!("WIFI_PASS");
const SOCKET_BUF_SIZE: usize = 1024;
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
    let mut rng = Rng::new(peripherals.RNG);

    let elapsed_time = || time::now().duration_since_epoch().to_millis();

    let wifi_init = esp_wifi::init(timg0.timer0, rng, peripherals.RADIO_CLK).unwrap();

    let (yiface, wifi_device, mut wifi_controller) =
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

    let mut rx_buf = [0u8; 1024];
    let mut tx_buf = [0u8; 1024];
    let mut rx_socket_buf = SocketBuffer::new(&mut rx_buf[..]);
    let mut tx_socket_buf = SocketBuffer::new(&mut tx_buf[..]);
    let mut socket_set_entries: [SocketStorage; 3] = Default::default();
    let mut socket_set = SocketSet::new(&mut socket_set_entries[..]);
    let mut tcp_socket = Socket::new(rx_socket_buf, tx_socket_buf);
    loop {}
}
