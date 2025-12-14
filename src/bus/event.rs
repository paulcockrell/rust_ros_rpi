#[derive(Debug, Clone)]
pub enum MotorDirection {
    Forward,
    Backward,
    Left,
    Right,
    Stop,
}

#[derive(Debug, Clone)]
pub struct MotorCommand {
    pub direction: MotorDirection,
    pub speed: u8,
}

#[derive(Debug, Clone)]
pub enum MotorState {}

#[derive(Debug, Clone)]
pub enum LedCommand {}

#[derive(Debug, Clone)]
pub enum Event {
    MotorCommand(MotorCommand),
    CameraFrameReady,
    Shutdown,
}
