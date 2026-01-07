use embedded_hal::{digital::{self, OutputPin}, pwm::{self, SetDutyCycle}};

#[derive(Debug)]
pub struct Drv8835<LeftSpeedPin, LeftDirPin, RightSpeedPin, RightDirPin> {
    left_speed_pin: LeftSpeedPin,
    left_dir_pin: LeftDirPin,
    right_speed_pin: RightSpeedPin,
    right_dir_pin: RightDirPin
}

impl<LeftSpeedPin, LeftDirPin, RightSpeedPin, RightDirPin> Drv8835<LeftSpeedPin, LeftDirPin, RightSpeedPin, RightDirPin>
where 
    LeftSpeedPin: SetDutyCycle,
    LeftDirPin: OutputPin,
    RightSpeedPin: SetDutyCycle,
    RightDirPin: OutputPin
{
    pub fn new(left_speed_pin: LeftSpeedPin, left_dir_pin: LeftDirPin, right_speed_pin: RightSpeedPin, right_dir_pin: RightDirPin) -> Self{
        Self {
            left_speed_pin,
            left_dir_pin,
            right_speed_pin,
            right_dir_pin
        }
    }

    pub fn drive_left_motor(&mut self, state: MotorState) -> Result<(), SetMotorStateError<LeftDirPin::Error, LeftSpeedPin::Error>>{
        match state {
            MotorState::Forward(speed) => {
                // Sets low to drive forward
                self.left_dir_pin.set_low().map_err(|error| SetMotorStateError::DirError(error))?;
                self.left_speed_pin.set_duty_cycle_fraction(speed as u16, 255).map_err(|error| SetMotorStateError::SpeedError(error))?;

            },
            MotorState::Backward(speed) => {
                self.left_dir_pin.set_high().map_err(|error| SetMotorStateError::DirError(error))?;
                self.left_speed_pin.set_duty_cycle_fraction(speed as u16, 255).map_err(|error| SetMotorStateError::SpeedError(error))?;
            }
        }
        Ok(())
    }

    pub fn drive_right_motor(&mut self, state: MotorState) -> Result<(), SetMotorStateError<RightDirPin::Error, RightSpeedPin::Error>>{
        match state {
            MotorState::Forward(speed) => {
                // Sets low to drive forward
                self.right_dir_pin.set_low().map_err(|error| SetMotorStateError::DirError(error))?;
                self.right_speed_pin.set_duty_cycle_fraction(speed as u16, 255).map_err(|error| SetMotorStateError::SpeedError(error))?;

            },
            MotorState::Backward(speed) => {
                self.right_dir_pin.set_high().map_err(|error| SetMotorStateError::DirError(error))?;
                self.right_speed_pin.set_duty_cycle_fraction(speed as u16, 255).map_err(|error| SetMotorStateError::SpeedError(error))?;
            }
        }
        Ok(())
    }
}

// Represents a motor's drive state, with the speed stored as a value between 0-255
#[derive(Clone, Copy, Debug)]
pub enum MotorState {
    Forward(u8),
    Backward(u8)
}

impl Default for MotorState {
    fn default() -> Self {
        Self::Forward(0)
    }
}

#[derive(Debug)]
pub enum SetMotorStateError<DirError: digital::Error, SpeedError: pwm::Error> {
    DirError(DirError),
    SpeedError(SpeedError)
}