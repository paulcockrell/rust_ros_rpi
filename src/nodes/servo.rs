use crate::{
    bus::{bus::EventBus, event::Event},
    hal::servo::Servo,
};

pub async fn run(bus: EventBus) {
    let mut rx = bus.subscribe();
    let mut servo = Servo::new().unwrap();

    loop {
        match rx.recv().await {
            Ok(Event::ServoCommand(cmd)) => {
                println!("Servo angle: {}", cmd.angle);
                let new_angle = cmd.angle.clamp(0, 180);
                let _ = servo.set_angle(new_angle);
            }
            Ok(Event::Shutdown) => {
                println!("Servo node shutting down");
                break;
            }
            _ => {}
        }
    }
}
