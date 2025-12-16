#[derive(Debug, Clone)]
pub enum MotorDirection {
    Forward,
    Stop,
}

#[derive(Debug, Clone)]
pub struct MotorCommand {
    pub direction: MotorDirection,
    pub speed: u8,
}

#[derive(Debug, Clone)]
pub struct Ultrasound {
    pub distance: f64,
}

#[derive(Debug, Clone)]
pub enum Event {
    MotorCommand(MotorCommand),
    CameraFrameReady,
    Ultrasound(Ultrasound),
    Shutdown,
}
