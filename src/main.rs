#[macro_use]
extern crate clap;

use clap::App;
use signal_device;
use std::time;

fn main() {
    let matches = App::new("modbus_client")
        .author("Thomas Sullivan")
        .version(&crate_version!()[..])
        .about("Modbus TCP Scanner")
        .args_from_usage("<DEVICE> 'The IP address or hostname of the device.")
        .get_matches();

    // This brick is a modbus device, with I/O.  Lets just use a single one for now,
    // located at index 16.  In the future this device will be configured by
    // file located in a resources directory.  So the IP will not need to be passed.
    let device = matches.value_of("DEVICE").unwrap_or("192.168.0.1");
    let mut brick = signal_device::new(device.to_string());
    println!("Using device address of: {}", brick.get_device());

    loop {
        println!("Signal value is: {}", brick.get_signal());

        std::thread::sleep(time::Duration::from_millis(1000));
    }
}
