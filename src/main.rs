use signal_device;
use std::time;

fn main() {
    // This brick is a modbus device, with I/O.  Lets just use a single one for now,
    // in the future we will pass to new() the name of the device config file for it
    // to load device detail from;
    let mut thingy = signal_device::new(format!("thing"));
    println!("Using device address of: {}", thingy.get_device());

    loop {
        println!("Signal value is: {}", thingy.get_signal());
        std::thread::sleep(time::Duration::from_millis(1000));
    }
}
