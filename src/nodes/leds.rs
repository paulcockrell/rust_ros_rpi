use crate::{
    bus::{event::Event, event_bus::EventBus},
    hal::neopixel::Neopixel,
};
use std::time::Duration;

pub async fn run(bus: EventBus) {
    let mut rx = bus.subscribe();
    let mut neopixel = Neopixel::new().expect("Neopixel failed");
    let mut last_ultrasound_i = 0_i32;
    let mut ultrasound = 0.0;
    let mut tick = tokio::time::interval(Duration::from_millis(100));

    loop {
        tokio::select! {
            _ = tick.tick() => {
                update_leds(&mut neopixel, ultrasound, &mut last_ultrasound_i);
            }
            msg = rx.recv() => {
                match msg {
                    Ok(Event::Shutdown) => {
                        println!("Leds node shutting down");
                        break;
                    },
                    Ok(Event::Ultrasound(data))=>{
                        ultrasound = data.distance;
                    }
                    _ => {}
                }
            }
        }
    }
}

fn update_leds(neopixel: &mut Neopixel, distance: f64, last_distance_i: &mut i32) {
    let distance_i = distance as i32;

    if distance_i != *last_distance_i {
        *last_distance_i = distance_i;

        let (r_val, g_val, b_val) = distance_to_rgb(distance);
        let _ = neopixel.set_pixels(r_val, g_val, b_val, 0);
    }
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
