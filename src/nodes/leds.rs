use crate::{
    bus::{
        event::{Event, Ultrasound},
        event_bus::EventBus,
    },
    hal::neopixel::Neopixel,
};
use std::{sync::mpsc, time::Duration};

pub async fn run(bus: EventBus) {
    let mut bus_rx = bus.subscribe();

    let (tx, rx) = mpsc::channel::<Ultrasound>();

    let leds_task = tokio::task::spawn_blocking(move || {
        let mut neopixel = Neopixel::new().expect("Neopixel failed");
        let mut last_distance_i = 0_i32;

        loop {
            match rx.recv_timeout(Duration::from_millis(50)) {
                Ok(data) => {
                    let distance_i = data.distance as i32;

                    if distance_i != last_distance_i {
                        last_distance_i = distance_i;

                        let (r_val, g_val, b_val) = distance_to_rgb(data.distance);
                        let _ = neopixel.set_pixels(r_val, g_val, b_val, 0);
                    }
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    // idle tick, do nothing
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    break;
                }
            }
        }
    });

    loop {
        match bus_rx.recv().await {
            Ok(Event::Ultrasound(cmd)) => {
                let _ = tx.send(cmd);
            }
            Ok(Event::Shutdown) => {
                println!("LEDs node shutting down");
                break;
            }
            Err(_) => break,
            _ => {}
        }
    }

    drop(tx);
    let _ = leds_task.await;
}

// Convert distance to a red-to-green scale for neopixels
fn distance_to_rgb(distance: f64) -> (u8, u8, u8) {
    let d = distance.clamp(0.0, 100.0);
    let t = d / 100.0;

    let red = (255.0 * (1.0 - t)) as u8;
    let green = (255.0 * t) as u8;
    let blue = 0u8;

    (red, green, blue)
}
