use crate::{
    bus::{
        bus::EventBus,
        event::{Event, MotorDirection},
    },
    hal::motor::Motor,
};

pub async fn run(bus: EventBus) {
    let mut rx = bus.subscribe();
    let mut left = Motor::new(26, 21, 4).unwrap();
    let mut right = Motor::new(27, 18, 17).unwrap();

    loop {
        match rx.recv().await {
            Ok(Event::MotorCommand(cmd)) => match cmd.direction {
                MotorDirection::Forward => {
                    let _ = left.forward(cmd.speed);
                    let _ = right.forward(cmd.speed);
                }
                MotorDirection::Backward => {
                    let _ = left.backward(cmd.speed);
                    let _ = right.backward(cmd.speed);
                }
                MotorDirection::Left => {
                    let _ = left.backward(cmd.speed);
                    let _ = right.forward(cmd.speed);
                }
                MotorDirection::Right => {
                    let _ = left.forward(cmd.speed);
                    let _ = right.backward(cmd.speed);
                }
                MotorDirection::Stop => {
                    let _ = left.forward(0);
                    let _ = right.forward(0);
                }
            },
            Ok(Event::Shutdown) => {
                println!("Motor node shutting down");
                break;
            }
            _ => {}
        }
    }
}
