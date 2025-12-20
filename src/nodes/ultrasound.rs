use crate::{
    bus::{
        event::{Event, Ultrasound},
        event_bus::EventBus,
    },
    hal::ultrasound::UltrasoundSensor,
};
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

pub async fn run(bus: EventBus) {
    let mut bus_rx = bus.subscribe();
    let bus_tx = bus.clone();

    let running = Arc::new(AtomicBool::new(true));
    let running_thread = running.clone();

    // === Blocking ultrasound sensor thread ===
    let task = tokio::task::spawn_blocking(move || {
        let mut us = UltrasoundSensor::new(11, 8).expect("Ultrasound init failed");
        let mut avg = 0.0;
        let mut last_avg = 0.0;

        while running_thread.load(Ordering::Relaxed) {
            let dist = us.measure_cm().unwrap_or(0);
            avg = avg * 0.7 + (dist as f64) * 0.3;

            if (avg - last_avg).abs() > 0.1 {
                bus_tx.publish(Event::Ultrasound(Ultrasound { distance: avg }));
                last_avg = avg;
            }

            std::thread::sleep(Duration::from_millis(200));
        }

        println!("Ultrasound Blocking task exited");
    });

    while let Ok(event) = bus_rx.recv().await {
        if matches!(event, Event::Shutdown) {
            println!("Ultrasound node shutting down");
            break;
        }
    }

    running.store(false, Ordering::Relaxed);
    let _ = task.await;
}
