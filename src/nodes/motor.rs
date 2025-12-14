use crate::bus::{bus::EventBus, event::Event};

pub async fn run(bus: EventBus) {
    let mut rx = bus.subscribe();

    loop {
        match rx.recv().await {
            Ok(Event::MotorCommand(cmd)) => {
                println!(
                    "Received direction={:?}, speed={:?}",
                    cmd.direction, cmd.speed,
                );
                // drive motors
            }
            Ok(Event::Shutdown) => {
                println!("Motor node shutting down");
                break;
            }
            _ => {}
        }
    }
}
