use crate::{
    bus::{event::Event, event_bus::EventBus},
    hal::camera::Camera,
};
use tokio::time::Duration;

pub async fn run(bus: EventBus) {
    let mut rx = bus.subscribe();
    let mut camera = Camera::new().expect("Could not setup camera");

    loop {
        tokio::select! {
            _ = tokio::time::sleep(Duration::from_secs(1))=>{
                let _ = camera.frame();
            }
            msg = rx.recv()=>{
                if matches!(msg, Ok(Event::Shutdown)) {
                    println!("Camera node shutting down");
                    break;
                }
            }
        }
    }
}
