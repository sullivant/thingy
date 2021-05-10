use applied_device::AppliedDevice;
use log::{error, info};
use signal_scanner::SignalScanner;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{thread, time};

static RETRACT_POS: u64 = 20000;
static EXTEND_POS: u64 = 300000;

fn main() {
    log4rs::init_file("thingy/resources/log4rs.yaml", Default::default()).unwrap();

    // Create a modbus signal scanner and pass to it the B&R brick to scan
    // Let's also create the mutex'd version for thread sharing
    let core_signal_scanner = SignalScanner::new("scanman!".to_string());
    let safe_signal_scanner = Arc::new(Mutex::new(core_signal_scanner));

    // This is the B&R brick and its registration to the signal scanner
    let coupler = signal_device::new("thing".to_string()).expect("Cannot continue:");
    safe_signal_scanner
        .lock()
        .unwrap()
        .register_device("brick".to_string(), coupler);
    let brick_name: String = "brick".to_string();
    let signal_name: String = "diInputSensor".to_string();

    // *****************************
    // This is the applied servo that is controllable via modbus tcp
    let mut servo_lifter = match AppliedDevice::new("thing".to_string(), "servo_lifter".to_string())
    {
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

    // *****************************
    // Create a vector of thread handles
    let alive = Arc::new(Mutex::new(true)); // A flag by which we can tell threads to die
    let alive_clone = Arc::clone(&alive);
    let mut thread_handles = vec![];
    let clone_signal_scanner = Arc::clone(&safe_signal_scanner);
    let handle = thread::spawn(move || {
        info!("Scanner thread starting up.");
        while *alive_clone.lock().unwrap() {
            clone_signal_scanner
                .lock()
                .unwrap()
                .refresh_signals()
                .unwrap();
            thread::sleep(Duration::from_millis(1));
        }
        info!("Scanner thread told to shut down.  Shutting down.");
    });
    thread_handles.push(handle);

    // Loop until we get the signal to extend then do our thing
    loop {
        let safe_scanner = Arc::clone(&safe_signal_scanner);
        if safe_scanner
            .lock()
            .unwrap()
            .get_device_signal_status(&brick_name, &signal_name)
            .unwrap_or(false)
        {
            break;
        };
        std::thread::sleep(time::Duration::from_millis(5));
    }

    info!("Got signal to extend and then retract.");
    servo_lifter.move_servo(400, 400, 1500, EXTEND_POS);
    std::thread::sleep(time::Duration::from_millis(1000));
    servo_lifter.move_servo(400, 400, 1500, RETRACT_POS);

    // Killing scanner thread
    let alive_clone = Arc::clone(&alive);
    *alive_clone.lock().unwrap() = false;

    // Lets send that disconnect command, see what happens...
    servo_lifter.shutdown();
    info!("Servo status: {:?}", servo_lifter.get_servo_status());
}

fn cycle_once() {}
