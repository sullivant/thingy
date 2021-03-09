use applied_device;
use signal_device;
use std::time;

fn main() {
    // This brick is a modbus device, with I/O.  Lets just use a single one for now,
    // in the future we will pass to new() the name of the device config file for it
    // to load device detail from;
    let mut thingy = signal_device::new(format!("thing"));
    println!("Using device address of: {}", thingy.get_device());

    // For each of the signals in the yaml hash under "signals:", it should make a signal object.
    // It is then up to the device code (here) to call thingy.get_signal("signalName").status()
    // to get the current status of the signal.
    //
    // Phase 2 will have each of those signals: hash values constantly updated with a scanner
    // thread so we can call get_signal("signalName").status() and it'll return the cached copy
    // (like what happens in the java side now)
    //
    // thingy.get_signal(String) should return a reference to the signal struct from the hashmap of
    // all created signals.

    let mut applied_servo = applied_device::new();

    loop {
        println!(
            "Signal value is: {}",
            //thingy.get_signal().get_signal_status()
            thingy.get_signal_directly()
        );
        std::thread::sleep(time::Duration::from_millis(500));
    }
}
