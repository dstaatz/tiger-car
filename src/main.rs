/* Copyright (C) 2020 Dylan Staatz - All Rights Reserved. */

use env_logger;
use rosrust;
use tiger_car_ros::*;
use rppal::pwm;


// Future parameters, defaults?
const DRIVETRAIN_PWM0: u8 = 5; // 12
const DRIVETRAIN_PWM1: u8 = 6; // 13

const STEERING_PWM0: pwm::Channel = pwm::Channel::Pwm0;
const STEERING_PWM1: pwm::Channel = pwm::Channel::Pwm1;

const STEERING_PWM_FREQ: f32 = 50.0;
const STEERING_MIN_DUTY_CYCLE: f32 = 0.3;

const DRIVETRAIN_PWM_FREQ: f32 = 50.0;
const DRIVETRAIN_MIN_DUTY_CYCLE: f32 = 0.2;


fn main() {
    env_logger::init();

    // Initialize node
    rosrust::init("listener");

    // Create subscriber
    // The subscriber is stopped when the returned object is destroyed
    let steering_subscriber = rosrust::subscribe("/tiger-car/control/steering", 8, |v: rosrust_msg::std_msgs::Float32| {
        // Callback for handling received messages
        rosrust::ros_info!("Steering Received: {}", v.data);
    })
    .unwrap();

    // Create subscriber
    // The subscriber is stopped when the returned object is destroyed
    let drivetrain_subscriber = rosrust::subscribe("/tiger-car/control/drivetrain", 8, |v: rosrust_msg::std_msgs::Float32| {
        // Callback for handling received messages
        rosrust::ros_info!("Drivetrain Received: {}", v.data);
    })
    .unwrap();

    let log_names = rosrust::param("~log_names").unwrap().get().unwrap_or(false);

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
}
