use applied_device;
use signal_device;
use std::time;

fn main() {
    // This is the applied servo that is controllable via modbus tcp
    let mut servo_lifter = applied_device::new(format!("thing"), format!("servo_lifter"));
    println!("{}", servo_lifter.to_string());
    println!(
        "Current encoder position: {}",
        servo_lifter.get_encoder_count()
    );

    // This brick is a modbus device, with I/O.
    // // This will setup all available signals
    let mut signal_device = signal_device::new(format!("thing"));
    println!("{}", signal_device.to_string());

    // The main application/device loop
    loop {
        println!(
            "Signal value is: {}",
            //thingy.get_signal().get_signal_status()
            signal_device.get_signal_directly() // TODO: pass to it a name of the signal
        );
        std::thread::sleep(time::Duration::from_millis(500));
    }
}
