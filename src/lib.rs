/* Copyright (C) 2020 Dylan Staatz - All Rights Reserved. */


use rppal::gpio;
use rppal::pwm;


/// Maps the input a from the interval a to the interval b.
/// No checks are done on bounds.
fn linear_map(input: f64, a: (f64, f64), b: (f64, f64)) -> f64 {
    (input - a.0) * (b.1 - b.0) / (a.1 - a.0) + b.0
}


////////////////////////////////////////////////////////////////////////////////


pub struct DualSoftwarePwm {
    pwm_0: gpio::OutputPin,
    pwm_1: gpio::OutputPin,
    frequency: f64,
    min_duty_cycle: f64,
}

impl DualSoftwarePwm {

    /// `pwm0_pin` is BCM GPIO pin number to output pwm0 on
    /// 
    /// `pwm1_pin` is BCM GPIO pin number to output pwm1 on
    /// 
    /// `frequency` is specified in hertz (Hz) and must be `>= 0`.
    ///
    /// `min_duty_cycle` is lowest duty cycle that should be outputted. This
    /// value is bounded on the interval `[0.0, 1.0]`. By finding and
    /// setting this value correctly the output functions of will correctly map
    /// the interval `(0.0, 1.0]` to `(min_duty_cycle, 1.0]` while still
    /// allowing the setting of output to `0.0`.
    ///
    /// Implemented with software PWMs, therefore higher freuquencies may not
    /// work. Additionally, general performance may not be consistent.
    pub fn new(
        pwm0_pin: u8,
        pwm1_pin: u8,
        frequency: f64,
        min_duty_cycle: f64)
    -> gpio::Result<Self> {

        let gpio = gpio::Gpio::new()?;
        let pwm_0 = gpio.get(pwm0_pin)?.into_output();
        let pwm_1 = gpio.get(pwm1_pin)?.into_output();

        Ok(Self {
            pwm_0,
            pwm_1,
            frequency: frequency.max(0.0),
            min_duty_cycle: min_duty_cycle.max(0.0).min(1.0),
        })
    }

    /// Sets the magnitude and direction of an output controlled by software pwm
    ///
    /// 'output' is a bounded on the interval `[-1.0, 1.0]`. A Negative number
    /// outputs on pwm0 and a positive number outputs on pwm1 and `0.0` is no
    /// output. This function will correctly map the interval `(0.0, 1.0]` to
    /// `(min_duty_cycle, 1.0]` while still allowing the setting of output to
    /// `0.0`.
    ///
    /// Will panic if output is NaN.
    pub fn output(&mut self, output: f64) -> gpio::Result<()> {

        let output = output.max(-1.0).min(1.0);

        if output == 0.0 {
            self.pwm_0.set_pwm_frequency(self.frequency, 0.0)?;
            self.pwm_1.set_pwm_frequency(self.frequency, 0.0)?;
        } else if output > 0.0 {
            let output = linear_map(output, (0.0, 1.0), (self.min_duty_cycle, 1.0));
            self.pwm_0.set_pwm_frequency(self.frequency, output)?;
            self.pwm_1.set_pwm_frequency(self.frequency, 0.0)?;
        } else {
            let output = linear_map(-1.0 * output, (0.0, 1.0), (self.min_duty_cycle, 1.0));
            self.pwm_0.set_pwm_frequency(self.frequency, 0.0)?;
            self.pwm_1.set_pwm_frequency(self.frequency, output)?;
        }

        Ok(())
    }
}

impl Drop for DualSoftwarePwm {
    fn drop(&mut self) {
        self.output(0.0).unwrap()
    }
}


////////////////////////////////////////////////////////////////////////////////


/// Currently only seems to work in Raspbian
pub struct DualHardwarePwm {
    pwm_0: pwm::Pwm,
    pwm_1: pwm::Pwm,
    min_duty_cycle: f64,
}

impl DualHardwarePwm {
    
    /// `pwm0_channel` is the pwm hardware channel output on
    /// 
    /// `pwm1_channel` is the pwm hardware channel output on
    /// 
    /// `frequency` is specified in hertz (Hz) and must be `>= 0`.
    ///
    /// `min_duty_cycle` is lowest duty cycle that should be outputted. This
    /// value is bounded on the interval `[0.0, 1.0]`. By finding and
    /// setting this value correctly the output functions of will correctly map
    /// the interval `(0.0, 1.0]` to `(min_duty_cycle, 1.0]` while still
    /// allowing the setting of output to `0.0`.
    /// 
    /// ## Is this still true?
    /// The following line will need to be added to last line in /boot/config.txt
    /// ```txt
    /// dtoverlay=pwm-2chan,pin=12,func=4,pin2=13,func2=4
    /// ```
    pub fn new(
        pwm0_channel: pwm::Channel,
        pwm1_channel: pwm::Channel,
        frequency: f64,
        min_duty_cycle: f64)
    -> pwm::Result<Self> {

        Ok(Self {
            pwm_0: pwm::Pwm::with_frequency(
                pwm0_channel,
                frequency.max(0.0),
                0.0,
                pwm::Polarity::Normal,
                true,
            )?,
            pwm_1: pwm::Pwm::with_frequency(
                pwm1_channel,
                frequency.max(0.0),
                0.0,
                pwm::Polarity::Normal,
                true,
            )?,
            min_duty_cycle: min_duty_cycle.max(0.0).min(1.0),
        })
    }

    /// Sets the magnitude and direction of an output controlled by software pwm
    ///
    /// 'output' is a bounded on the interval `[-1.0, 1.0]`. A Negative number
    /// outputs on pwm0 and a positive number outputs on pwm1 and `0.0` is no
    /// output. This function will correctly map the interval `(0.0, 1.0]` to
    /// `(min_duty_cycle, 1.0]` while still allowing the setting of output to
    /// `0.0`.
    ///
    /// Will panic if output is NaN.
    pub fn output(&self, output: f64) -> pwm::Result<()> {

        let output = output.max(-1.0).min(1.0);

        if output == 0.0 {
            self.pwm_0.set_duty_cycle(0.0)?;
            self.pwm_1.set_duty_cycle(0.0)?;
        } else if output > 0.0 {
            let output = linear_map(output, (0.0, 1.0), (self.min_duty_cycle, 1.0));
            self.pwm_0.set_duty_cycle(output)?;
            self.pwm_1.set_duty_cycle(0.0)?;
        } else {
            let output = linear_map(-1.0 * output, (0.0, 1.0), (self.min_duty_cycle, 1.0));
            self.pwm_0.set_duty_cycle(0.0)?;
            self.pwm_1.set_duty_cycle(output)?;
        }

        Ok(())
    }
}

impl Drop for DualHardwarePwm {
    fn drop(&mut self) {
        self.output(0.0).unwrap()
    }
}


////////////////////////////////////////////////////////////////////////////////


pub fn run() {
    unimplemented!();
}


////////////////////////////////////////////////////////////////////////////////


#[cfg(test)]
mod tests {

    use super::*;

    use std::thread::sleep;
    use std::time::Duration;

    // Testing parameters
    const DRIVETRAIN_PWM0: u8 = 5; // 12
    const DRIVETRAIN_PWM1: u8 = 6; // 13

    const DRIVETRAIN_PWM_FREQ: f64 = 50.0;
    const DRIVETRAIN_MIN_DUTY_CYCLE: f64 = 0.2;

    const STEERING_PWM0: pwm::Channel = pwm::Channel::Pwm0;
    const STEERING_PWM1: pwm::Channel = pwm::Channel::Pwm1;

    const STEERING_PWM_FREQ: f64 = 50.0;
    const STEERING_MIN_DUTY_CYCLE: f64 = 0.3;

    #[test]
    fn test_linear_map() {
        assert_eq!(linear_map(6.0, (0.0, 10.0), (0.0, 100.0)), 60.0);
        assert_eq!(linear_map(6.0, (0.0, 10.0), (10.0, 20.0)), 16.0);
        assert_eq!(linear_map(6.0, (0.0, 10.0), (5.0, 10.0)), 8.0);
    }
}
