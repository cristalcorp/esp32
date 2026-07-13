//! Abstraction de capteur de distance laser ToF.
//! Le trait `LaserSensor` est le contrat commun ; chaque puce l'implémente.

use embedded_hal::delay::DelayNs;
use esp_hal::gpio::Output;
use esp_hal::i2c::master::I2c;

const OUT_OF_RANGE_THRESHOLD_MM: u16 = 2000;

pub enum Reading {
    Distance(u16),
    OutOfRange,
    NotReady,
}

/// Contrat commun à tous les capteurs laser ToF.
pub trait LaserSensor {
    fn read(&mut self) -> Reading;
}

// --- Implémentation VL53L0X ---

pub struct Vl53l0x<'d> {
    inner: vl53l0x_simple::Vl53l0x<I2c<'d, esp_hal::Blocking>, Output<'d>>,
}

impl<'d> Vl53l0x<'d> {
    pub fn new(
        i2c: I2c<'d, esp_hal::Blocking>,
        x_shut: Output<'d>,
        delay: &mut impl DelayNs,
    ) -> Result<Self, ()> {
        let inner = vl53l0x_simple::Vl53l0x::new(i2c, x_shut, 0x29, delay)
            .map_err(|_| ())?;
        Ok(Self { inner })
    }
}

impl<'d> LaserSensor for Vl53l0x<'d> {
    fn read(&mut self) -> Reading {
        match self.inner.try_read() {
            Ok(Some(mm)) if mm >= OUT_OF_RANGE_THRESHOLD_MM => Reading::OutOfRange,
            Ok(Some(mm)) => Reading::Distance(mm),
            Ok(None) => Reading::NotReady,
            Err(_) => Reading::OutOfRange,
        }
    }
}

// --- Implémentation VL53L1X (dans 2 jours, quand le crate sera choisi) ---
//
// pub struct Vl53l1x<'d> { inner: /* driver 1x */ }
//
// impl<'d> Vl53l1x<'d> {
//     pub fn new(...) -> Result<Self, ()> { ... }
// }
//
// impl<'d> LaserSensor for Vl53l1x<'d> {
//     fn read(&mut self) -> Reading {
//         // traduit la donnée brute du 1x en Reading
//     }
// }
