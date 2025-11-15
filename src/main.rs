use pigpio_sys::*;
use std::{thread, time::Duration};

fn main() {
    unsafe {
        // Start pigpio in "library mode"
        if gpioInitialise() < 0 {
            panic!("Failed to initialise pigpio");
        }

        let ldr_left_pin = 19;
        let ldr_middle_pin = 16;
        let ldr_right_pin = 20;

        gpioSetMode(ldr_left_pin, PI_INPUT);
        gpioSetMode(ldr_middle_pin, PI_INPUT);
        gpioSetMode(ldr_right_pin, PI_INPUT);

        println!("LDR LEFT started on GPIO{}", ldr_left_pin);
        println!("LDR MIDDLE started on GPIO{}", ldr_middle_pin);
        println!("LDR RIGHT started on GPIO{}", ldr_right_pin);

        loop {
            let ldr_left_level = gpioRead(ldr_left_pin);
            let ldr_middle_level = gpioRead(ldr_middle_pin);
            let ldr_right_level = gpioRead(ldr_right_pin);
            println!("LDR LEFT level: {}", ldr_left_pin);
            println!("LDR MIDDLE level: {}", ldr_middle_pin);
            println!("LDR RIGHT level: {}", ldr_right_pin);

            thread::sleep(Duration::from_millis(500));
        }
    }
}
