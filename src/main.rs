use applied_device::AppliedDevice;
use log::{error, info};
use signal_device::SignalDevice;
use signal_scanner::SignalScanner;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{thread, time};

static RETRACT_POS: u64 = 20000;
static EXTEND_POS: u64 = 300000;

// A convenience struct to pass around to functions like cycle_once(), etc.
pub struct ThisDevice {
    device_name: String,
    core_signal_scanner: SignalScanner,
    servo_lifter: AppliedDevice,
}

fn main() {
    log4rs::init_file("thingy/resources/log4rs.yaml", Default::default()).unwrap();

    // Some initial device configuration
    let mut this_device = ThisDevice {
        device_name: "thing".to_string(),
        core_signal_scanner: SignalScanner::new("device_scanner".to_string()),
        servo_lifter: match AppliedDevice::new("thing".to_string(), "servo_lifter".to_string()) {
            Ok(s) => s,
            Err(e) => {
                error!("Unable to create applied device, cannot continue: {}", e);
                return;
            }
        },
    };

    // Create a modbus signal scanner and pass to it the B&R brick to scan
    // Let's also create the mutex'd version for thread sharing
    //let core_signal_scanner = SignalScanner::new("scanman!".to_string())/
    let safe_signal_scanner = Arc::new(Mutex::new(this_device.core_signal_scanner));

    // This is the B&R brick and its registration to the signal scanner
    safe_signal_scanner.lock().unwrap().register_device(
        "brick".to_string(),
        SignalDevice::new(&this_device.device_name).expect("Cannot continue:"),
    );

    info!("Starting device: {}", &this_device.device_name);

    // Call out to the servo lifter and set things up, home, etc.
    setup_servo_lifter(&mut this_device.servo_lifter);

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
    info!("Main application loop starting.");
    loop {
        cycle_once(
            &mut this_device.servo_lifter,
            Arc::clone(&safe_signal_scanner),
        );

        // Just a ...thing... to let us leave the main loop and tickle shutdown methods.
        if this_device.servo_lifter.get_servo_cycle_count() > 10 {
            break;
        }
        std::thread::sleep(time::Duration::from_millis(5));
    }

    // Killing scanner thread
    let alive_clone = Arc::clone(&alive);
    *alive_clone.lock().unwrap() = false;

    // Lets send that disconnect command, see what happens...
    this_device.servo_lifter.shutdown();
    info!(
        "Servo status: {:?}",
        this_device.servo_lifter.get_servo_status()
    );
}

fn setup_servo_lifter(servo_lifter: &mut AppliedDevice) {
    // *****************************
    // This is the applied servo that is controllable via modbus tcp
    info!("{}", servo_lifter.to_string());
    info!("Servo status: {:?}", servo_lifter.get_servo_status());
    info!("Servo alarms: {:?}", servo_lifter.get_servo_alarms());
    // Start the actual device features needed
    servo_lifter.home_servo();
    servo_lifter.move_servo(400, 400, 1500, RETRACT_POS);
}

fn cycle_once(
    servo_lifter: &mut AppliedDevice,
    safe_scanner: std::sync::Arc<std::sync::Mutex<SignalScanner>>,
) {
    let brick_name: String = "brick".to_string();
    let signal_name: String = "diInputSensor".to_string();
    if safe_scanner
        .lock()
        .unwrap()
        .get_device_signal_status(&brick_name, &signal_name)
        .unwrap_or(false)
    {
        info!("Got signal to extend and then retract.");

        servo_lifter.move_servo(400, 400, 2500, EXTEND_POS);
        std::thread::sleep(time::Duration::from_millis(300));
        servo_lifter.move_servo(400, 400, 2500, RETRACT_POS);
    }
}
