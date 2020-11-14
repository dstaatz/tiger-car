/* Copyright (C) 2020 Dylan Staatz - All Rights Reserved. */


// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

mod errors;
mod pwm;


////////////////////////////////////////////////////////////////////////////////


use std::thread::sleep;
use std::time::Duration;
use std::sync::{Arc, Mutex};

use rosrust_msg::std_msgs;

use errors::*;
use pwm::DualSoftwarePwm;


// TODO: get parameters instead of hard coding
const DRIVETRAIN_PWM0: u8 = 5;
const DRIVETRAIN_PWM1: u8 = 6;

const DRIVETRAIN_PWM_FREQ: f64 = 50.0;
const DRIVETRAIN_MIN_DUTY_CYCLE: f64 = 0.15;

const STEERING_PWM0: u8 = 12;
const STEERING_PWM1: u8 = 13;

const STEERING_PWM_FREQ: f64 = 50.0;
const STEERING_MIN_DUTY_CYCLE: f64 = 0.2;


pub fn run() -> Result<()> {

    let log_names = rosrust::param("~log_names").unwrap().get().unwrap_or(false);

    rosrust::ros_info!("Starting tiger_car");


    let steering = Arc::new(Mutex::new(
        DualSoftwarePwm::new(
            STEERING_PWM0,
            STEERING_PWM1,
            STEERING_PWM_FREQ,
            STEERING_MIN_DUTY_CYCLE,
        ).unwrap()
    ));

    let drivetrain = Arc::new(Mutex::new(
        DualSoftwarePwm::new(
            DRIVETRAIN_PWM0,
            DRIVETRAIN_PWM1,
            DRIVETRAIN_PWM_FREQ,
            DRIVETRAIN_MIN_DUTY_CYCLE,
        ).unwrap()
    ));

    // Subscriptions
    let steering_subscriber = rosrust::subscribe(
        "/tiger_car/steer",
        8,
        move |v: std_msgs::Float64| {
            rosrust::ros_info!("Steering Received: {}", v.data);
            let result = steering.lock().unwrap().output(v.data);
            if result.is_err() {
                rosrust::ros_err!("Steering Error: {}", result.unwrap_err());
            }
        }
    )?;

    let drivetrain_subscriber = rosrust::subscribe(
        "/tiger_car/speed",
        8,
        move |v: std_msgs::Float64| {
            rosrust::ros_info!("Drivetrain Received: {}", v.data);
            let result = drivetrain.lock().unwrap().output(v.data);
            if result.is_err() {
                rosrust::ros_err!("Drivetrain Error: {}", result.unwrap_err());
            }
        }
    )?;

    // Loop
    if log_names {
        let rate = rosrust::rate(1.0);
        while rosrust::is_ok() {
            rosrust::ros_info!("Steering Publisher uris: {:?}", steering_subscriber.publisher_uris());
            rosrust::ros_info!("Drivertrain Publisher uris: {:?}", drivetrain_subscriber.publisher_uris());
            rate.sleep();
        }
    } else {
        // Block the thread until a shutdown signal is received
        rosrust::spin();
    }

    Ok(())
}


// Manual tests

fn test_drivetrain_range() {
    let mut drivetrain = DualSoftwarePwm::new(
        DRIVETRAIN_PWM0,
        DRIVETRAIN_PWM1,
        DRIVETRAIN_PWM_FREQ,
        DRIVETRAIN_MIN_DUTY_CYCLE,
    ).unwrap();

    for i in -5..6 {
        let val = 0.05 * i as f64;
        println!("Setting {:.2}", val);
        drivetrain.output(val).unwrap();
        sleep(Duration::new(3, 0));
    }
}

fn test_steering_range() {
    let mut steering = DualSoftwarePwm::new(
        STEERING_PWM0,
        STEERING_PWM1,
        STEERING_PWM_FREQ,
        STEERING_MIN_DUTY_CYCLE,
    ).unwrap();

    for i in -10..11 {
        let val = -1.0 * 0.1 * i as f64;
        steering.output(0.0).unwrap();
        sleep(Duration::new(0, 100_000_000));
        println!("Setting {:.2}", val);
        steering.output(val).unwrap();
        sleep(Duration::new(0, 1_000_000_000));
    }
}

