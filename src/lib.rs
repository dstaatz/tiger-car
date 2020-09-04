/* Copyright (C) 2020 Dylan Staatz - All Rights Reserved. */


use rppal::gpio;
use rppal::pwm;


/// Maps the input a from the interval a to the interval b.
/// No checks are done on bounds.
fn linear_map(intput: f64, a: (f64, f64), b: (f64, f64)) -> f64 {
    (input - a.0) * (b.1 - b.0) / (a.1 - a.0) + b.0;
}


////////////////////////////////////////////////////////////////////////////////


const DRIVETRAIN_PWM0: u8 = 5;
const DRIVETRAIN_PWM1: u8 = 6;


struct DrivetrainController {
    gpio: gpio::Gpio,
    pwm_0: gpio::OutputPin,
    pwm_1: gpio::OutputPin,
    frequency: f64,
    min_duty_cycle: f64,
}


impl DrivetrainController {
    
    /// Creates a new drivetrain that is set to the given frequency.
    /// 
    /// `frequency` is specified in hertz (Hz) and must be `>= 0`.
    /// 
    /// `min_duty_cycle` is lowest duty cycle that should be outputted. This
    /// value is bounded on the interval `[0.0, 1.0]`. Most DC motors will try
    /// to drive with low duty cycles, but will fail to move. By finding and
    /// setting this value correctly the output functions of will correctly map
    /// the interval `(0.0, 1.0]` to `(min_duty_cycle, 1.0]` while still
    /// allowing the setting of output to `0.0`.
    ///     
    /// Implemented with software pwm, therefore higher freuquencies may not
    /// work. Additionally, generally performance may not be consistent.
    fn new(frequency: f64, min_duty_cycle: f64) -> gpio::Result<Self> {
        
        let gpio = gpio::Gpio::new()?;
        let pwm_0 = gpio.get(DRIVETRAIN_PWM0)?.into_output();
        let pwm_1 = gpio.get(DRIVETRAIN_PWM1)?.into_output();

        Ok(Self {
            gpio,
            pwm_0,
            pwm_1,
            frequency: frequency.max(0.0),
            min_duty_cycle: min_duty_cycle.max(0.0).min(1.0),
        })
    }


    pub fn set_output(&mut self, output: f64) {
        self.set_output_with_frequency(output, self.frequency);
    }

    /// Sets the power and direction of the drivetrain at the given pwm frequency.
    /// 
    /// 'output' is a bounded on the interval `[-1.0, 1.0]`. Naturally
    /// a negative number specifies reverse and a posiive number specifies
    /// forward and `0.0` is no output. This function will correctly map
    /// the interval `(0.0, 1.0]` to `(min_duty_cycle, 1.0]` while still
    /// allowing the setting of output to `0.0`.
    /// 
    /// `frequency` is specified in hertz (Hz) and must be `>= 0`.
    /// 
    /// Will panic if output or frequency is NaN.
    pub fn set_output_with_frequency(&mut self, output: f64, frequency: f64) {
        
        let output = output.max(-1.0).min(1.0);
        let frequency = frequency.max(0.0);

        if output == 0.0 {
            self.pwm_0.set_pwm_frequency(frequency, 0.0);
            self.pwm_1.set_pwm_frequency(frequency, 0.0);
        } else if output > 0.0 {
            let output = linear_map(output, (0.0, 1.0), (self.min_duty_cycle, 1.0));

        } else {

        }
    }

}


////////////////////////////////////////////////////////////////////////////////


const STEERING_PWM0 = pwm::Channel::Pwm0;
const STEERING_PWM1 = pwm::Channel::Pwm1;


struct SteeringController {
    channel_0: pwm::Pwm,
    channel_1: pwm::Pwm,
}

impl SteeringController {

    fn new() -> pwm::Result<Self> {
        Ok(Self {
            channel_0: pwm::Pwm::new(DRIVETRAIN_PWM0)?,
            channel_1: pwm::Pwm::new(DRIVETRAIN_PWM1)?,
        })
    }
}


/// TODO
/// 
/// The following line will need to be added to last line in /boot/config.txt
/// dtoverlay=pwm-2chan,pin=12,func=4,pin2=13,func2=4
/// 
struct TigerCar {
    steering: DrivetrainController,
    drivetrain: SteeringController,
}


impl TigerCar {

    /// TODO
    pub fn new() -> Self {
        
        Self {
            steering: DrivetrainController::new(40.0, 0.3).unwrap(),
            drivetrain: SteeringController::new().unwrap(),
        }
    }

}


////////////////////////////////////////////////////////////////////////////////


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_linear_map() {
        assert_eq!(linear_map(6.0, (0.0, 10.0), (0.0, 100.0)), 6.0);
    }
}
