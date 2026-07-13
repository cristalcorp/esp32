//! Pilote la LED bleue embarquée (GPIO2 sur la plupart des devkits ESP32).

use esp_hal::gpio::{Level, Output, OutputConfig, OutputPin};

pub struct Led<'d> {
    pin: Output<'d>,
}

impl<'d> Led<'d> {
    /// Crée une LED éteinte sur la broche fournie.
    pub fn new(pin: impl OutputPin + 'd) -> Self {
        let pin = Output::new(pin, Level::Low, OutputConfig::default());
        Self { pin }
    }

    pub fn on(&mut self) {
        self.pin.set_high();
    }

    pub fn off(&mut self) {
        self.pin.set_low();
    }

    pub fn toggle(&mut self) {
        self.pin.toggle();
    }
}
