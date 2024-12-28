#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::timer::PeriodicTimer;
use esp_hal::{delay::Delay, prelude::*, rng::Rng, time, timer::timg::TimerGroup};
use esp_println::println;
use esp_wifi::wifi::{ClientConfiguration, Configuration, WifiStaDevice};
use smoltcp::iface::{SocketSet, SocketStorage};
use smoltcp::socket::tcp::{Socket, SocketBuffer};
use smoltcp::time::Instant;
use smoltcp::wire::{IpAddress, IpCidr, Ipv4Address};

const SSID: &str = env!("SSID");
const WIFI_PASS: &str = env!("WIFI_PASS");
const SOCKET_BUF_SIZE: usize = 1024;

enum ConnectionState {
    Connect,
    Request,
    Response,
}

#[entry]
fn main() -> ! {
    // Initialize Peripherals
    let config = esp_hal::Config::default();
    let peripherals = esp_hal::init(config);
    esp_alloc::heap_allocator!(72 * 1024);
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let mut periodic_timer = PeriodicTimer::new(timg0.timer0);
    let mut wifi_device = peripherals.WIFI;
    let mut rng = Rng::new(peripherals.RNG);

    let elapsed_time = || time::now().duration_since_epoch().to_millis();

    // Initialize Wifi Drivers
    let wifi_init = esp_wifi::init(timg0.timer0, rng, peripherals.RADIO_CLK).unwrap();

    let (mut iface, mut wifi_device, mut wifi_controller) =
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

    let res = wifi_controller.set_configuration(&wifi_config);
    println!("Wifi Configuration Result: {:?}", res);
    wifi_controller.start().unwrap();
    println!("Has wifi started {:?}", wifi_controller.is_started());
    println!("Wif connect {:?}", wifi_controller.connect());

    // Connect to Wifi
    loop {
        match wifi_controller.is_connected() {
            Ok(true) => {
                println!("Connected Sucessfully to Wifi");
                break;
            }
            Ok(false) => println!(
                "Establishing Connection to: {:?}",
                wifi_controller.configuration().unwrap(),
            ),
            Err(err) => {
                println!("Connection Error: {err:?}");
                loop {}
            }
        }
    }

    // Create TCP Socket
    let mut rx_buffer = [0; SOCKET_BUF_SIZE];
    let mut tx_buffer = [0; SOCKET_BUF_SIZE];
    let tcp_rx_buffer = SocketBuffer::new(&mut rx_buffer[..]);
    let tcp_tx_buffer = SocketBuffer::new(&mut tx_buffer[..]);
    let tcp_socket = Socket::new(tcp_rx_buffer, tcp_tx_buffer);
    let mut socket_entry: [SocketStorage; 1] = Default::default();
    let mut socket_set = SocketSet::new(&mut socket_entry[..]);
    let tcp_handle = socket_set.add(tcp_socket);
    iface.update_ip_addrs(|ip_addrs| {
        ip_addrs
            .push(IpCidr::new(IpAddress::v4(192, 168, 101, 199), 24))
            .unwrap();
    });
    iface
        .routes_mut()
        .add_default_ipv4_route(Ipv4Address::new(192, 168, 101, 199))
        .unwrap();

    periodic_timer.start(10.secs());
    loop {
        nb::block!(periodic_timer.wait());
        iface.poll(
            Instant::from_millis(time::now().duration_since_epoch().to_millis() as i64),
            &mut wifi_device,
            &mut socket_set,
        );

        let socket = socket_set.get_mut::<Socket>(tcp_handle);
        let cx = iface.context();
        if !socket.is_active() {
            socket
                .connect(cx, (IpAddress::v4(192, 168, 101, 200), 8000), 8008)
                .unwrap();
        }
        if socket.may_send() {
            socket.send_slice(&msg[..]).unwrap();
        }
    }
}
