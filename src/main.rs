use applied_device;
use signal_device;
use std::time;

static RETRACT_POS: u64 = 20000;
static EXTEND_POS: u64 = 300000;

fn main() {
    // This is the applied servo that is controllable via modbus tcp
    let mut servo_lifter = match applied_device::new(format!("thing"), format!("servo_lifter")) {
        Ok(s) => s,
        Err(e) => {
            println!("Unable to create applied device, cannot continue: {}", e);
            return;
        }
    };
    // This is the B&R brick
    let mut signal_device = match signal_device::new(format!("thing")) {
        Ok(s) => s,
        Err(e) => {
            println!("Unable to create signal device, cannot continue: {}", e);
            return;
        }
    };

    println!("{}", servo_lifter.to_string());
    println!("Servo status: {:?}", servo_lifter.get_servo_status());
    println!("Servo alarms: {:?}", servo_lifter.get_servo_alarms());
    println!("{}", signal_device.to_string());

    // Start the actual device features needed
    servo_lifter.home_servo();
    servo_lifter.move_servo(400, 400, 1500, RETRACT_POS);

    let signal_name: String = "diInputSensor".to_string();

    // The main application/device loop
    loop {
        match signal_device.get_signal_directly(&signal_name) {
            Ok(true) => servo_lifter.move_servo(400, 400, 1500, EXTEND_POS),
            Ok(false) => servo_lifter.move_servo(400, 400, 1500, RETRACT_POS),
            Err(e) => {
                println!("{}", e)
            }
        };

        std::thread::sleep(time::Duration::from_millis(5));
    }
}
