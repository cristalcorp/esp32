#![no_std]
#![no_main]

mod led;
mod laser;

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::i2c::master::{Config as I2cConfig, I2c};
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::delay::Delay;
use esp_println::println;

// use crate::laser::{Laser, Reading};
use crate::laser::{LaserSensor, Reading, Vl53l0x};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("PANIC: {:?}", info);
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(_spawner: Spawner) -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // --- Init esp-rtos / embassy (pattern du témoin) ---
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

    // --- Init capteur (identique à avant) ---
    let i2c = I2c::new(peripherals.I2C0, I2cConfig::default())
        .unwrap()
        .with_sda(peripherals.GPIO21)
        .with_scl(peripherals.GPIO22);

    let x_shut = Output::new(peripherals.GPIO5, Level::High, OutputConfig::default());
    let mut delay = Delay::new();

    // let mut laser = Laser::new(i2c, x_shut, &mut delay)
    //     .expect("init capteur laser");
    let mut laser = Vl53l0x::new(i2c, x_shut, &mut delay)
        .expect("init capteur laser");

    println!("Capteur prêt (async).");

    // --- Boucle async : Timer.await au lieu du busy-wait ---
    loop {
        match laser.read() {
            Reading::Distance(mm) => println!("Distance: {} mm", mm),
            Reading::OutOfRange => println!("Hors portée"),
            Reading::NotReady => {}
        }
        Timer::after(Duration::from_millis(100)).await;
    }
}
