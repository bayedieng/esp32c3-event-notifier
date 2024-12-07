use esp_idf_hal::delay::FreeRtos;

fn main() {
    println!("Hello World");
    loop {
        FreeRtos::delay_ms(1000);
    }
}
