# Event Notifier For Esp32-C3

This is a very simple example of an event notifier that sends tcp requests every 10 seconds to some tcp server.

## Build Requirements
- [Rust](https://www.rust-lang.org/learn/get-started)

After installing rust install, espflash with the following command:

```cargo install espflash cargo-espflash```

## Run Test Tcp Server
Make sure to run Tcp server before flashing device.

```
cargo run -p --release tcp-server
```

## Flashing Event Notifier
```
cd event-notifier
SSID='Enter Wifi SSID here' WIFI_PASS='enter wifi password here' cargo run --release
