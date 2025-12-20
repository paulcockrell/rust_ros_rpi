use crate::{
    bus::{
        event::{Event, Ultrasound},
        event_bus::EventBus,
    },
    hal::ultrasound::UltrasoundSensor,
};
use std::time::Duration;

pub async fn run(bus: EventBus) {
    let mut bus_rx = bus.subscribe();

    // Clone bus sender handle for blocking thread
    let bus_tx = bus.clone();

    // === Blocking sensor thread ===
    let sensor_thread = std::thread::spawn(move || {
        let mut us = UltrasoundSensor::new(11, 8).expect("Ultrasound init failed");
        let mut avg = 0.0;
        let mut last_avg = 0.0;

        loop {
            let dist = us.measure_cm().unwrap_or(0);
            let new_value = (dist as f64).clamp(0.0, 100.0);

            avg = avg * 0.7 + new_value * 0.3;

            if (avg - last_avg).abs() > 0.1 {
                bus_tx.publish(Event::Ultrasound(Ultrasound { distance: avg }));
                last_avg = avg;
            }

            std::thread::sleep(Duration::from_millis(100));
        }
    });

    // === Async shutdown watcher ===
    loop {
        match bus_rx.recv().await {
            Ok(Event::Shutdown) => {
                println!("Ultrasound node shutting down");
                break;
            }
            Err(_) => break,
            _ => {}
        }
    }

    // Thread will naturally exit when process shuts down
    let _ = sensor_thread.join();
}
