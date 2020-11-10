/* Copyright (C) 2020 Dylan Staatz - All Rights Reserved. */


use tiger_car_ros::*;


fn main() {

    if let Err(_) = env_logger::try_init() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";
        writeln!(stderr, "Failed to initialize logger").expect(errmsg);
        return
    }

    if let Err(_) = rosrust::try_init("tiger_car") {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";
        writeln!(stderr, "Failed to initialize ROS Node").expect(errmsg);
        return
    }

    if let Err(ref e) = run() {

        rosrust::ros_err!("error: {}", e);

        for e in e.iter().skip(1) {
            rosrust::ros_err!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            rosrust::ros_err!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}

