use anyhow::{Context, Ok, Result};
use rppal::gpio::{Gpio, OutputPin};

const PWM_FREQ: f64 = 1000.0;

pub struct Motor {
    in1: OutputPin,
    in2: OutputPin,
    en: OutputPin,
}

impl Motor {
    pub fn new(in1: u8, in2: u8, en: u8) -> Result<Motor> {
        let gpio = Gpio::new().context("Failed to initialize GPIO")?;
        let in1 = gpio.get(in1)?.into_output_low();
        let in2 = gpio.get(in2)?.into_output_low();
        let en = gpio.get(en)?.into_output();

        Ok(Self { in1, in2, en })
    }

    pub fn forward(&mut self, speed: u8 /* 0..100 */) -> Result<()> {
        self.in1.set_low();
        self.in2.set_high();

        let speed = speed.clamp(1, 100);
        let duty_cycle = (speed as f64) / 100.0;

        self.en.set_pwm_frequency(PWM_FREQ, duty_cycle)?;

        Ok(())
    }

    pub fn backward(&mut self, speed: u8 /* 0..100 */) -> Result<()> {
        self.in1.set_high();
        self.in2.set_low();

        let speed = speed.clamp(1, 100);
        let duty_cycle = (speed as f64) / 100.0;

        self.en.set_pwm_frequency(PWM_FREQ, duty_cycle)?;

        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        self.en.set_pwm_frequency(PWM_FREQ, 0.0)?;

        Ok(())
    }
}

impl Drop for Motor {
    fn drop(&mut self) {
        self.in1.set_low();
        self.in2.set_low();
        self.en.set_pwm_frequency(0.0, 0.0).unwrap();
    }
}
