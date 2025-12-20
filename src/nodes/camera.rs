use crate::{
    bus::{event::Event, event_bus::EventBus},
    hal::camera::Camera,
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

    let running = Arc::new(AtomicBool::new(true));
    let running_thread = running.clone();

    let task = tokio::task::spawn_blocking(move || {
        let mut camera = Camera::new().expect("Could not setup camera");
        while running_thread.load(Ordering::Relaxed) {
            if matches!(camera.save_frame(), Ok(true)) {
                println!("Saved frame");
            } else {
                println!("Failed to save frame");
            }

            std::thread::sleep(Duration::from_millis(1000));
        }
    });

    while let Ok(event) = bus_rx.recv().await {
        if matches!(event, Event::Shutdown) {
            println!("LDR node shutting down");
            break;
        }
    }

    running.store(false, Ordering::Relaxed);
    let _ = task.await;
}
