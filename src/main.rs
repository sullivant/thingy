use applied_device;
use log::{error, info};
use log4rs;
use signal_device::SignalDevice;
use signal_scanner::SignalScanner;
use std::time;

static RETRACT_POS: u64 = 20000;
static EXTEND_POS: u64 = 300000;

fn main() {
    log4rs::init_file("thingy/resources/log4rs.yaml", Default::default()).unwrap();

    // Create a modbus signal scanner and pass to it the B&R brick to scan
    let mut signal_scanner = SignalScanner::new(format!("scanman!"));

    // This is the B&R brick
    let coupler = signal_device::new(format!("thing")).expect("Cannot continue:");

    &signal_scanner.register_device(format!("brick"), coupler);
    println!("Signal Scanner name: {}", signal_scanner.get_thread_text());

    let brick: &mut SignalDevice = signal_scanner
        .get_device_mut(&"brick")
        .expect("Unable to find signal device: Brick");

    println!("Brick: {:?}", brick);
    brick.set_coupler_address(format!("1.1.1.1"));

    // This is the applied servo that is controllable via modbus tcp
    let mut servo_lifter = match applied_device::new(format!("thing"), format!("servo_lifter")) {
        Ok(s) => s,
        Err(e) => {
            error!("Unable to create applied device, cannot continue: {}", e);
            return;
        }
    };

    info!("{}", servo_lifter.to_string());
    info!("Servo status: {:?}", servo_lifter.get_servo_status());
    info!("Servo alarms: {:?}", servo_lifter.get_servo_alarms());

    // Start the actual device features needed
    servo_lifter.home_servo();
    servo_lifter.move_servo(400, 400, 1500, RETRACT_POS);

    let signal_name: String = "diInputSensor".to_string();

    // Loop until we get the signal once.
    loop {
        if brick.get_signal_directly(&signal_name).unwrap() {
            break;
        };
        std::thread::sleep(time::Duration::from_millis(5));
    }

    info!("Got signal to extend and then retract.");
    servo_lifter.move_servo(400, 400, 1500, EXTEND_POS);
    std::thread::sleep(time::Duration::from_millis(1000));
    servo_lifter.move_servo(400, 400, 1500, RETRACT_POS);

    // Lets send that disconnect command, see what happens...
    servo_lifter.shutdown();
    info!("Servo status: {:?}", servo_lifter.get_servo_status());
}
